// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::db_v2::IndexerAsyncV2;
use anyhow::Result;
use aptos_types::{
    indexer::table_info_reader::TableInfoReader,
    state_store::table::{TableHandle, TableInfo},
};

/// Table info reader is to create a thin interface for other services to read the db data,
/// this standalone db is officially not part of the AptosDB anymore.
/// For services that need table info mapping, they need to acquire this reader in the FN bootstrapping stage.

impl TableInfoReader for IndexerAsyncV2 {
    fn get_table_info(&self, handle: TableHandle) -> Result<Option<TableInfo>> {
        Ok(self.get_table_info_with_retry(handle)?)
    }
}
