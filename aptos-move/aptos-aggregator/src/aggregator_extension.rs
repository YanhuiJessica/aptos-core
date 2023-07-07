// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    delta_change_set::{addition, subtraction},
    resolver::AggregatorResolver,
};
use aptos_types::{state_store::state_key::StateKey, vm_status::StatusCode};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::account_address::AccountAddress;
use move_table_extension::TableHandle;
use std::{sync::atomic::{AtomicU64, Ordering}, collections::{BTreeMap, BTreeSet}};

/// Describes the state of each aggregator instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AggregatorState {
    // If aggregator stores a known value.
    Data,
    // If aggregator stores a non-negative delta.
    PositiveDelta,
    // If aggregator stores a negative delta.
    NegativeDelta,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AggregatorHandle(pub AccountAddress);

/// Uniquely identifies each aggregator instance in storage.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AggregatorID {
    // Aggregator V1 is implemented as a Table item, and so can be queried by the
    // state key.
    Legacy {
        // A handle that is shared across all aggregator instances created by the
        // same `AggregatorFactory` and which is used for fine-grained storage
        // access.
        handle: TableHandle,
        // Unique key associated with each aggregator instance. Generated by
        // taking the hash of transaction which creates an aggregator and the
        // number of aggregators that were created by this transaction so far.
        key: AggregatorHandle,
    },
    // Aggregator V2 is implemented in place with ephemeral identifiers which are
    // unique per block.
    Ephemeral(u128),
}

impl AggregatorID {
    pub fn legacy(handle: TableHandle, key: AggregatorHandle) -> Self {
        AggregatorID::Legacy { handle, key }
    }

    pub fn into_state_key(self) -> Option<StateKey> {
        match self {
            AggregatorID::Legacy { handle, key } => {
                Some(StateKey::table_item(handle.into(), key.0.to_vec()))
            },
            AggregatorID::Ephemeral(_) => None,
        }
    }
}

/// Tracks values seen by aggregator. In particular, stores information about
/// the biggest and the smallest deltas seen during execution in the VM. This
/// information can be used by the executor to check if delta should have
/// failed. Most importantly, it allows commutativity of adds/subs. Example:
///
///
/// This graph shows how delta of aggregator changed during a single transaction
/// execution:
///
/// +A ===========================================>
///            ||
///          ||||                               +X
///         |||||  ||||||                    ||||
///      |||||||||||||||||||||||||          |||||
/// +0 ===========================================> time
///                       ||||||
///                         ||
///                         ||
/// -B ===========================================>
///
/// Clearly, +X succeeds if +A and -B succeed. Therefore each delta
/// validation consists of:
///   1. check +A did not overflow
///   2. check -A did not drop below zero
/// Checking +X is irrelevant since +A >= +X.
///
/// TODO: while we support tracking of the history, it is not yet fully used on
/// executor side because we don't know how to throw errors.
#[derive(Debug)]
pub struct History {
    pub max_positive: u128,
    pub min_negative: u128,
}

impl History {
    fn new() -> Self {
        History {
            max_positive: 0,
            min_negative: 0,
        }
    }

    fn record_positive(&mut self, value: u128) {
        self.max_positive = u128::max(self.max_positive, value);
    }

    fn record_negative(&mut self, value: u128) {
        self.min_negative = u128::max(self.min_negative, value);
    }
}

/// Internal aggregator data structure.
#[derive(Debug)]
pub struct Aggregator {
    // Describes a value of an aggregator.
    value: u128,
    // Describes a state of an aggregator.
    state: AggregatorState,
    // Describes an upper bound of an aggregator. If `value` exceeds it, the
    // aggregator overflows.
    // TODO: Currently this is a single u128 value since we use 0 as a trivial
    // lower bound. If we want to support custom lower bounds, or have more
    // complex postconditions, we should factor this out in its own struct.
    limit: u128,
    // Describes values seen by this aggregator. Note that if aggregator knows
    // its value, then storing history doesn't make sense.
    history: Option<History>,
}

impl Aggregator {
    /// Records observed delta in history. Should be called after an operation
    /// to record its side-effects.
    fn record(&mut self) {
        if let Some(history) = self.history.as_mut() {
            match self.state {
                AggregatorState::PositiveDelta => history.record_positive(self.value),
                AggregatorState::NegativeDelta => history.record_negative(self.value),
                AggregatorState::Data => {
                    unreachable!("history is not tracked when aggregator knows its value")
                },
            }
        }
    }

    /// Implements logic for adding to an aggregator.
    pub fn add(&mut self, value: u128) -> PartialVMResult<()> {
        match self.state {
            AggregatorState::Data => {
                // If aggregator knows the value, add directly and keep the state.
                self.value = addition(self.value, value, self.limit)?;
                return Ok(());
            },
            AggregatorState::PositiveDelta => {
                // If positive delta, add directly but also record the state.
                self.value = addition(self.value, value, self.limit)?;
            },
            AggregatorState::NegativeDelta => {
                // Negative delta is a special case, since the state might
                // change depending on how big the `value` is. Suppose
                // aggregator has -X and want to do +Y. Then, there are two
                // cases:
                //     1. X <= Y: then the result is +(Y-X)
                //     2. X  > Y: then the result is -(X-Y)
                if self.value <= value {
                    self.value = subtraction(value, self.value)?;
                    self.state = AggregatorState::PositiveDelta;
                } else {
                    self.value = subtraction(self.value, value)?;
                }
            },
        }

        // Record side-effects of addition in history.
        self.record();
        Ok(())
    }

    /// Implements logic for subtracting from an aggregator.
    pub fn sub(&mut self, value: u128) -> PartialVMResult<()> {
        match self.state {
            AggregatorState::Data => {
                // Aggregator knows the value, therefore we can subtract
                // checking we don't drop below zero. We do not need to
                // record the history.
                self.value = subtraction(self.value, value)?;
                return Ok(());
            },
            AggregatorState::PositiveDelta => {
                // Positive delta is a special case because the state can
                // change depending on how big the `value` is. Suppose
                // aggregator has +X and want to do -Y. Then, there are two
                // cases:
                //     1. X >= Y: then the result is +(X-Y)
                //     2. X  < Y: then the result is -(Y-X)
                if self.value >= value {
                    self.value = subtraction(self.value, value)?;
                } else {
                    // Check that we can subtract in general: we don't want to
                    // allow -10000 when limit is 10.
                    // TODO: maybe `subtraction` should also know about the limit?
                    subtraction(self.limit, value)?;

                    self.value = subtraction(value, self.value)?;
                    self.state = AggregatorState::NegativeDelta;
                }
            },
            AggregatorState::NegativeDelta => {
                // Since we operate on unsigned integers, we have to add
                // when subtracting from negative delta. Note that if limit
                // is some X, then we cannot subtract more than X, and so
                // we should return an error there.
                self.value = addition(self.value, value, self.limit)?;
            },
        }

        // Record side-effects of addition in history.
        self.record();
        Ok(())
    }

    /// Implements logic for reading the value of an aggregator. As a
    /// result, the aggregator knows it value (i.e. its state changes to
    /// `Data`).
    pub fn read_and_materialize(
        &mut self,
        resolver: &dyn AggregatorResolver,
        id: &AggregatorID,
    ) -> PartialVMResult<u128> {
        // If aggregator has already been read, return immediately.
        if self.state == AggregatorState::Data {
            return Ok(self.value);
        }

        // Otherwise, we have a delta and have to go to storage and apply it.
        // In theory, any delta will be applied to existing value. However,
        // something may go wrong, so we guard by throwing an error in
        // extension.
        let value_from_storage = resolver
            .resolve_aggregator_value(id)
            .map_err(|_| extension_error("could not find the value of the aggregator"))?;

        // Sanity checks.
        debug_assert!(
            self.history.is_some(),
            "resolving aggregator with no history"
        );
        let history = self.history.as_ref().unwrap();

        // Validate history of the aggregator, ensure that there
        // was no violation of postcondition. We can do it by
        // emulating addition and subtraction.
        addition(value_from_storage, history.max_positive, self.limit)?;
        subtraction(value_from_storage, history.min_negative)?;

        // Validation succeeded, and now we can actually apply the delta.
        match self.state {
            AggregatorState::PositiveDelta => {
                self.value = addition(value_from_storage, self.value, self.limit)?;
            },
            AggregatorState::NegativeDelta => {
                self.value = subtraction(value_from_storage, self.value)?;
            },
            AggregatorState::Data => {
                unreachable!("history is not tracked when aggregator knows its value")
            },
        }

        // Change the state and return the new value. Also, make
        // sure history is no longer tracked.
        self.state = AggregatorState::Data;
        self.history = None;
        Ok(self.value)
    }

    /// Unpacks aggregator into its fields.
    pub fn into(self) -> (u128, AggregatorState, u128, Option<History>) {
        (self.value, self.state, self.limit, self.history)
    }
}

/// Stores all information about aggregators (how many have been created or
/// removed), what are their states, etc. per single transaction).
#[derive(Default)]
pub struct AggregatorData {
    // All aggregators that were created in the current transaction, stored as ids.
    // Used to filter out aggregators that were created and destroyed in the
    // within a single transaction.
    new_aggregators: BTreeSet<AggregatorID>,
    // All aggregators that were destroyed in the current transaction, stored as ids.
    destroyed_aggregators: BTreeSet<AggregatorID>,
    // All aggregator instances that exist in the current transaction.
    aggregators: BTreeMap<AggregatorID, Aggregator>,
    // Counter for generating identifiers for AggregatorSnapshots.
    pub id_counter: AtomicU64
}

impl AggregatorData {

    pub fn new(id_counter: u64) -> Self {
        Self {
            id_counter: AtomicU64::new(id_counter),
            ..Default::default()
        }
    }
    /// Returns a mutable reference to an aggregator with `id` and a `limit`.
    /// If transaction that is currently executing did not initialize it, a new aggregator instance is created.
    /// The state of the new aggregator instance depends on the `aggregator_enabled` flag.
    /// If the `aggregator_enabled` flag is true, the new aggregator instance
    /// is initialized with zero and in a delta state.
    /// If the `aggregator_enabled` flag is false, the new aggregator instance
    /// is initialized in the Data state with its latest value.
    /// Note: when we say "aggregator instance" here we refer to Rust struct and
    /// not to the Move aggregator.
    pub fn get_aggregator(
        &mut self,
        id: AggregatorID,
        limit: u128,
        resolver: &dyn AggregatorResolver,
        aggregator_enabled: bool,
    ) -> PartialVMResult<&mut Aggregator> {
        let aggregator = self.aggregators.entry(id).or_insert(Aggregator {
            value: 0,
            state: AggregatorState::PositiveDelta,
            limit,
            history: Some(History::new()),
        });

        if !aggregator_enabled {
            aggregator.read_and_materialize(resolver, &id)?;
        }
        Ok(aggregator)
    }

    /// Returns the number of aggregators that are used in the current transaction.
    pub fn num_aggregators(&self) -> u128 {
        self.aggregators.len() as u128
    }

    /// Creates and a new Aggregator with a given `id` and a `limit`. The value
    /// of a new aggregator is always known, therefore it is created in a data
    /// state, with a zero-initialized value.
    pub fn create_new_aggregator(&mut self, id: AggregatorID, limit: u128) {
        let aggregator = Aggregator {
            value: 0,
            state: AggregatorState::Data,
            limit,
            history: None,
        };
        self.aggregators.insert(id, aggregator);
        self.new_aggregators.insert(id);
    }

    /// If aggregator has been used in this transaction, it is removed. Otherwise,
    /// it is marked for deletion.
    pub fn remove_aggregator(&mut self, id: AggregatorID) {
        // Aggregator no longer in use during this transaction: remove it.
        self.aggregators.remove(&id);

        if self.new_aggregators.contains(&id) {
            // Aggregator has been created in the same transaction. Therefore, no
            // side-effects.
            self.new_aggregators.remove(&id);
        } else {
            // Otherwise, aggregator has been created somewhere else.
            self.destroyed_aggregators.insert(id);
        }
    }

    pub fn generate_id(&mut self) -> u64 {
        self.id_counter.fetch_add(1, Ordering::SeqCst);
        self.id_counter.load(Ordering::SeqCst)
    }

    /// Unpacks aggregator data.
    pub fn into(
        self,
    ) -> (
        BTreeSet<AggregatorID>,
        BTreeSet<AggregatorID>,
        BTreeMap<AggregatorID, Aggregator>,
    ) {
        (
            self.new_aggregators,
            self.destroyed_aggregators,
            self.aggregators,
        )
    }
}

/// Returns partial VM error on extension failure.
pub fn extension_error(message: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(message.to_string())
}

// ================================= Tests =================================

#[cfg(test)]
mod test {
    use super::*;
    use crate::{aggregator_id_for_test, AggregatorStore};
    use claims::{assert_err, assert_ok};
    use once_cell::sync::Lazy;

    #[allow(clippy::redundant_closure)]
    static TEST_RESOLVER: Lazy<AggregatorStore> = Lazy::new(|| AggregatorStore::default());

    #[test]
    fn test_materialize_not_in_storage() {
        let mut aggregator_data = AggregatorData::default();

        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(300), 700, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_err!(aggregator.read_and_materialize(&*TEST_RESOLVER, &aggregator_id_for_test(700)));
    }

    #[test]
    fn test_materialize_known() {
        let mut aggregator_data = AggregatorData::default();
        aggregator_data.create_new_aggregator(aggregator_id_for_test(200), 200);

        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(200), 200, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_ok!(aggregator.add(100));
        assert_ok!(aggregator.read_and_materialize(&*TEST_RESOLVER, &aggregator_id_for_test(200)));
        assert_eq!(aggregator.value, 100);
    }

    #[test]
    fn test_get_stored_aggregator_disabled() {
        let mut fake_resolver = AggregatorStore::default();
        fake_resolver.set_from_id(aggregator_id_for_test(500), 150);

        let mut aggregator_data = AggregatorData::default();
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(500), 500, &fake_resolver, false)
            .expect("Get aggregator failed");
        assert_eq!(aggregator.state, AggregatorState::Data);
        assert_eq!(aggregator.value, 150);
        assert_ok!(aggregator.add(50));
        assert_eq!(aggregator.state, AggregatorState::Data);
        assert_eq!(aggregator.value, 200);
    }

    #[test]
    fn test_get_created_aggregator_disabled() {
        let mut aggregator_data = AggregatorData::default();
        aggregator_data.create_new_aggregator(aggregator_id_for_test(500), 500);
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(500), 500, &*TEST_RESOLVER, false)
            .expect("Get aggregator failed");
        assert_eq!(aggregator.state, AggregatorState::Data);
        assert_eq!(aggregator.value, 0);
    }

    #[test]
    fn test_unknown_aggregator_disabled_fail() {
        let mut aggregator_data = AggregatorData::default();
        assert_err!(aggregator_data.get_aggregator(
            aggregator_id_for_test(200),
            200,
            &*TEST_RESOLVER,
            false,
        ));
    }

    #[test]
    fn test_materialize_overflow() {
        let mut aggregator_data = AggregatorData::default();

        // +0 to +400 satisfies <= 600 and is ok, but materialization fails
        // with 300 + 400 > 600!
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(600), 600, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_ok!(aggregator.add(400));
        assert_err!(aggregator.read_and_materialize(&*TEST_RESOLVER, &aggregator_id_for_test(600)));
    }

    #[test]
    fn test_materialize_overflow_aggregator_disabled() {
        let mut fake_resolver = AggregatorStore::default();
        fake_resolver.set_from_id(aggregator_id_for_test(500), 200);

        let mut aggregator_data = AggregatorData::default();
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(500), 500, &fake_resolver, false)
            .expect("Get aggregator failed");
        assert_err!(aggregator.add(400));
    }

    #[test]
    fn test_materialize_underflow() {
        let mut aggregator_data = AggregatorData::default();

        // +0 to -400 is ok, but materialization fails with 300 - 400 < 0!
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(600), 600, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_ok!(aggregator.add(400));
        assert_err!(aggregator.read_and_materialize(&*TEST_RESOLVER, &aggregator_id_for_test(600)));
    }

    #[test]
    fn test_materialize_underflow_aggregator_disabled() {
        let mut fake_resolver = AggregatorStore::default();
        fake_resolver.set_from_id(aggregator_id_for_test(500), 150);

        let mut aggregator_data = AggregatorData::default();
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(500), 600, &fake_resolver, false)
            .expect("Get aggregator failed");
        assert_err!(aggregator.sub(400));
    }

    #[test]
    fn test_materialize_non_monotonic_1() {
        let mut aggregator_data = AggregatorData::default();

        // +0 to +400 to +0 is ok, but materialization fails since we had 300 + 400 > 600!
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(600), 600, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_ok!(aggregator.add(400));
        assert_ok!(aggregator.sub(300));
        assert_eq!(aggregator.value, 100);
        assert_eq!(aggregator.state, AggregatorState::PositiveDelta);
        assert_err!(aggregator.read_and_materialize(&*TEST_RESOLVER, &aggregator_id_for_test(600)));
    }

    #[test]
    fn test_materialize_non_monotonic_2() {
        let mut aggregator_data = AggregatorData::default();

        // +0 to -301 to -300 is ok, but materialization fails since we had 300 - 301 < 0!
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(600), 600, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_ok!(aggregator.sub(301));
        assert_ok!(aggregator.add(1));
        assert_eq!(aggregator.value, 300);
        assert_eq!(aggregator.state, AggregatorState::NegativeDelta);
        assert_err!(aggregator.read_and_materialize(&*TEST_RESOLVER, &aggregator_id_for_test(600)));
    }

    #[test]
    fn test_add_overflow() {
        let mut aggregator_data = AggregatorData::default();

        // +0 to +800 > 600!
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(600), 600, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_err!(aggregator.add(800));

        // 0 + 300 > 200!
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(200), 200, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_err!(aggregator.add(300));
    }

    #[test]
    fn test_sub_underflow() {
        let mut aggregator_data = AggregatorData::default();
        aggregator_data.create_new_aggregator(aggregator_id_for_test(200), 200);

        // +0 to -601 is impossible!
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(600), 600, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_err!(aggregator.sub(601));

        // Similarly, we cannot subtract anything from 0...
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(200), 200, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_err!(aggregator.sub(2));
    }

    #[test]
    fn test_commutative() {
        let mut aggregator_data = AggregatorData::default();

        // +200 -300 +50 +300 -25 +375 -600.
        let aggregator = aggregator_data
            .get_aggregator(aggregator_id_for_test(600), 600, &*TEST_RESOLVER, true)
            .expect("Get aggregator failed");
        assert_ok!(aggregator.add(200));
        assert_ok!(aggregator.sub(300));

        assert_eq!(aggregator.value, 100);
        assert_eq!(aggregator.history.as_ref().unwrap().max_positive, 200);
        assert_eq!(aggregator.history.as_ref().unwrap().min_negative, 100);
        assert_eq!(aggregator.state, AggregatorState::NegativeDelta);

        assert_ok!(aggregator.add(50));
        assert_ok!(aggregator.add(300));
        assert_ok!(aggregator.sub(25));

        assert_eq!(aggregator.value, 225);
        assert_eq!(aggregator.history.as_ref().unwrap().max_positive, 250);
        assert_eq!(aggregator.history.as_ref().unwrap().min_negative, 100);
        assert_eq!(aggregator.state, AggregatorState::PositiveDelta);

        assert_ok!(aggregator.add(375));
        assert_ok!(aggregator.sub(600));

        assert_eq!(aggregator.value, 0);
        assert_eq!(aggregator.history.as_ref().unwrap().max_positive, 600);
        assert_eq!(aggregator.history.as_ref().unwrap().min_negative, 100);
        assert_eq!(aggregator.state, AggregatorState::PositiveDelta);
    }
}
