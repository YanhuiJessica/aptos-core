
<a id="0x1_randomness"></a>

# Module `0x1::randomness`

On-chain randomness utils.


-  [Resource `PerBlockRandomness`](#0x1_randomness_PerBlockRandomness)
-  [Resource `Ghost$var`](#0x1_randomness_Ghost$var)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_randomness_initialize)
-  [Function `on_new_block`](#0x1_randomness_on_new_block)
-  [Function `next_blob`](#0x1_randomness_next_blob)
-  [Function `u8_integer`](#0x1_randomness_u8_integer)
-  [Function `u16_integer`](#0x1_randomness_u16_integer)
-  [Function `u32_integer`](#0x1_randomness_u32_integer)
-  [Function `u64_integer`](#0x1_randomness_u64_integer)
-  [Function `u128_integer`](#0x1_randomness_u128_integer)
-  [Function `u256_integer`](#0x1_randomness_u256_integer)
-  [Function `u8_range`](#0x1_randomness_u8_range)
-  [Function `u16_range`](#0x1_randomness_u16_range)
-  [Function `u32_range`](#0x1_randomness_u32_range)
-  [Function `u64_range`](#0x1_randomness_u64_range)
-  [Function `u128_range`](#0x1_randomness_u128_range)
-  [Function `u256_range`](#0x1_randomness_u256_range)
-  [Function `permutation`](#0x1_randomness_permutation)
-  [Function `safe_add_mod`](#0x1_randomness_safe_add_mod)
-  [Function `safe_add_mod_for_verification`](#0x1_randomness_safe_add_mod_for_verification)
-  [Function `fetch_and_increment_txn_counter`](#0x1_randomness_fetch_and_increment_txn_counter)
-  [Function `is_safe_call`](#0x1_randomness_is_safe_call)
-  [Specification](#@Specification_1)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `on_new_block`](#@Specification_1_on_new_block)
    -  [Function `next_blob`](#@Specification_1_next_blob)
    -  [Function `u8_integer`](#@Specification_1_u8_integer)
    -  [Function `u16_integer`](#@Specification_1_u16_integer)
    -  [Function `u32_integer`](#@Specification_1_u32_integer)
    -  [Function `u64_integer`](#@Specification_1_u64_integer)
    -  [Function `u128_integer`](#@Specification_1_u128_integer)
    -  [Function `u256_integer`](#@Specification_1_u256_integer)
    -  [Function `u8_range`](#@Specification_1_u8_range)
    -  [Function `u64_range`](#@Specification_1_u64_range)
    -  [Function `u256_range`](#@Specification_1_u256_range)
    -  [Function `permutation`](#@Specification_1_permutation)
    -  [Function `safe_add_mod_for_verification`](#@Specification_1_safe_add_mod_for_verification)
    -  [Function `fetch_and_increment_txn_counter`](#@Specification_1_fetch_and_increment_txn_counter)
    -  [Function `is_safe_call`](#@Specification_1_is_safe_call)


<pre><code><b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">0x1::hash</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="system_addresses.md#0x1_system_addresses">0x1::system_addresses</a>;
<b>use</b> <a href="transaction_context.md#0x1_transaction_context">0x1::transaction_context</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_randomness_PerBlockRandomness"></a>

## Resource `PerBlockRandomness`

32-byte randomness seed unique to every block.
This resource is updated in every block prologue.


<pre><code><b>struct</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> <b>has</b> drop, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>seed: <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_randomness_Ghost$var"></a>

## Resource `Ghost$var`



<pre><code><b>struct</b> Ghost$<a href="randomness.md#0x1_randomness_var">var</a> <b>has</b> <b>copy</b>, drop, store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>v: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_randomness_DST"></a>



<pre><code><b>const</b> <a href="randomness.md#0x1_randomness_DST">DST</a>: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [65, 80, 84, 79, 83, 95, 82, 65, 78, 68, 79, 77, 78, 69, 83, 83];
</code></pre>



<a id="0x1_randomness_E_API_USE_SUSCEPTIBLE_TO_TEST_AND_ABORT"></a>

Randomness APIs calls must originate from a private entry function. Otherwise, test-and-abort attacks are possible.


<pre><code><b>const</b> <a href="randomness.md#0x1_randomness_E_API_USE_SUSCEPTIBLE_TO_TEST_AND_ABORT">E_API_USE_SUSCEPTIBLE_TO_TEST_AND_ABORT</a>: u64 = 1;
</code></pre>



<a id="0x1_randomness_initialize"></a>

## Function `initialize`

Called in genesis.move.
Must be called in tests to initialize the <code><a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a></code> resource.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_initialize">initialize</a>(framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_initialize">initialize</a>(framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <a href="system_addresses.md#0x1_system_addresses_assert_aptos_framework">system_addresses::assert_aptos_framework</a>(framework);
    <b>move_to</b>(framework, <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
        epoch: 0,
        round: 0,
        seed: <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>(),
    });
}
</code></pre>



</details>

<a id="0x1_randomness_on_new_block"></a>

## Function `on_new_block`

Invoked in block prologues to update the block-level randomness seed.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="randomness.md#0x1_randomness_on_new_block">on_new_block</a>(vm: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, epoch: u64, round: u64, seed_for_new_block: <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="randomness.md#0x1_randomness_on_new_block">on_new_block</a>(vm: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, epoch: u64, round: u64, seed_for_new_block: Option&lt;<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;) <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <a href="system_addresses.md#0x1_system_addresses_assert_vm">system_addresses::assert_vm</a>(vm);
    <b>if</b> (<b>exists</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework)) {
        <b>let</b> <a href="randomness.md#0x1_randomness">randomness</a> = <b>borrow_global_mut</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework);
        <a href="randomness.md#0x1_randomness">randomness</a>.epoch = epoch;
        <a href="randomness.md#0x1_randomness">randomness</a>.round = round;
        <a href="randomness.md#0x1_randomness">randomness</a>.seed = seed_for_new_block;
    }
}
</code></pre>



</details>

<a id="0x1_randomness_next_blob"></a>

## Function `next_blob`

Generate 32 random bytes.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>assert</b>!(<a href="randomness.md#0x1_randomness_is_safe_call">is_safe_call</a>(), <a href="randomness.md#0x1_randomness_E_API_USE_SUSCEPTIBLE_TO_TEST_AND_ABORT">E_API_USE_SUSCEPTIBLE_TO_TEST_AND_ABORT</a>);

    <b>let</b> input = <a href="randomness.md#0x1_randomness_DST">DST</a>;
    <b>let</b> <a href="randomness.md#0x1_randomness">randomness</a> = <b>borrow_global</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework);
    <b>let</b> seed = *<a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&<a href="randomness.md#0x1_randomness">randomness</a>.seed);
    <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> input, seed);
    <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> input, <a href="transaction_context.md#0x1_transaction_context_get_transaction_hash">transaction_context::get_transaction_hash</a>());
    <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> input, <a href="randomness.md#0x1_randomness_fetch_and_increment_txn_counter">fetch_and_increment_txn_counter</a>());
    <a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash_sha3_256">hash::sha3_256</a>(input)
}
</code></pre>



</details>

<a id="0x1_randomness_u8_integer"></a>

## Function `u8_integer`

Generates an u8 uniformly at random.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u8_integer">u8_integer</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u8_integer">u8_integer</a>(): u8 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> raw = <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>();
    <b>let</b> ret: u8 = <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> raw);
    ret
}
</code></pre>



</details>

<a id="0x1_randomness_u16_integer"></a>

## Function `u16_integer`

Generates an u16 uniformly at random.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u16_integer">u16_integer</a>(): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u16_integer">u16_integer</a>(): u16 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> raw = <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>();
    <b>let</b> i = 0;
    <b>let</b> ret: u16 = 0;
    <b>while</b> (i &lt; 2) {
        ret = ret * 256 + (<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> raw) <b>as</b> u16);
        i = i + 1;
    };
    ret
}
</code></pre>



</details>

<a id="0x1_randomness_u32_integer"></a>

## Function `u32_integer`

Generates an u32 uniformly at random.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u32_integer">u32_integer</a>(): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u32_integer">u32_integer</a>(): u32 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> raw = <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>();
    <b>let</b> i = 0;
    <b>let</b> ret: u32 = 0;
    <b>while</b> (i &lt; 4) {
        ret = ret * 256 + (<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> raw) <b>as</b> u32);
        i = i + 1;
    };
    ret
}
</code></pre>



</details>

<a id="0x1_randomness_u64_integer"></a>

## Function `u64_integer`

Generates an u64 uniformly at random.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u64_integer">u64_integer</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u64_integer">u64_integer</a>(): u64 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> raw = <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>();
    <b>let</b> i = 0;
    <b>let</b> ret: u64 = 0;
    <b>while</b> (i &lt; 8) {
        ret = ret * 256 + (<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> raw) <b>as</b> u64);
        i = i + 1;
    };
    ret
}
</code></pre>



</details>

<a id="0x1_randomness_u128_integer"></a>

## Function `u128_integer`

Generates an u128 uniformly at random.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u128_integer">u128_integer</a>(): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u128_integer">u128_integer</a>(): u128 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> raw = <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>();
    <b>let</b> i = 0;
    <b>let</b> ret: u128 = 0;
    <b>while</b> (i &lt; 16) {
        ret = ret * 256 + (<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> raw) <b>as</b> u128);
        i = i + 1;
    };
    ret
}
</code></pre>



</details>

<a id="0x1_randomness_u256_integer"></a>

## Function `u256_integer`

Generates a u256 uniformly at random.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>(): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>(): u256 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> raw = <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>();
    <b>let</b> i = 0;
    <b>let</b> ret: u256 = 0;
    <b>while</b> (i &lt; 32) {
        ret = ret * 256 + (<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> raw) <b>as</b> u256);
        i = i + 1;
    };
    ret
}
</code></pre>



</details>

<a id="0x1_randomness_u8_range"></a>

## Function `u8_range`

Generates a number $n \in [min_incl, max_excl)$ uniformly at random.

NOTE: The uniformity is not perfect, but it can be proved that the bias is negligible.
If you need perfect uniformity, consider implement your own via rejection sampling.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u8_range">u8_range</a>(min_incl: u8, max_excl: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u8_range">u8_range</a>(min_incl: u8, max_excl: u8): u8 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> range = ((max_excl - min_incl) <b>as</b> u256);
    <b>let</b> sample = ((<a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>() % range) <b>as</b> u8);
    min_incl + sample
}
</code></pre>



</details>

<a id="0x1_randomness_u16_range"></a>

## Function `u16_range`

Generates a number $n \in [min_incl, max_excl)$ uniformly at random.

NOTE: The uniformity is not perfect, but it can be proved that the bias is negligible.
If you need perfect uniformity, consider implement your own via rejection sampling.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u16_range">u16_range</a>(min_incl: u16, max_excl: u16): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u16_range">u16_range</a>(min_incl: u16, max_excl: u16): u16 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> range = ((max_excl - min_incl) <b>as</b> u256);
    <b>let</b> sample = ((<a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>() % range) <b>as</b> u16);
    min_incl + sample
}
</code></pre>



</details>

<a id="0x1_randomness_u32_range"></a>

## Function `u32_range`

Generates a number $n \in [min_incl, max_excl)$ uniformly at random.

NOTE: The uniformity is not perfect, but it can be proved that the bias is negligible.
If you need perfect uniformity, consider implement your own via rejection sampling.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u32_range">u32_range</a>(min_incl: u32, max_excl: u32): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u32_range">u32_range</a>(min_incl: u32, max_excl: u32): u32 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> range = ((max_excl - min_incl) <b>as</b> u256);
    <b>let</b> sample = ((<a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>() % range) <b>as</b> u32);
    min_incl + sample
}
</code></pre>



</details>

<a id="0x1_randomness_u64_range"></a>

## Function `u64_range`

Generates a number $n \in [min_incl, max_excl)$ uniformly at random.

NOTE: The uniformity is not perfect, but it can be proved that the bias is negligible.
If you need perfect uniformity, consider implement your own via rejection sampling.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u64_range">u64_range</a>(min_incl: u64, max_excl: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u64_range">u64_range</a>(min_incl: u64, max_excl: u64): u64 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> range = ((max_excl - min_incl) <b>as</b> u256);
    <b>let</b> sample = ((<a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>() % range) <b>as</b> u64);
    min_incl + sample
}
</code></pre>



</details>

<a id="0x1_randomness_u128_range"></a>

## Function `u128_range`

Generates a number $n \in [min_incl, max_excl)$ uniformly at random.

NOTE: The uniformity is not perfect, but it can be proved that the bias is negligible.
If you need perfect uniformity, consider implement your own via rejection sampling.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u128_range">u128_range</a>(min_incl: u128, max_excl: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u128_range">u128_range</a>(min_incl: u128, max_excl: u128): u128 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> range = ((max_excl - min_incl) <b>as</b> u256);
    <b>let</b> sample = ((<a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>() % range) <b>as</b> u128);
    min_incl + sample
}
</code></pre>



</details>

<a id="0x1_randomness_u256_range"></a>

## Function `u256_range`

Generates a number $n \in [min_incl, max_excl)$ uniformly at random.

NOTE: The uniformity is not perfect, but it can be proved that the bias is negligible.
If you need perfect uniformity, consider implement your own with <code><a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>()</code> + rejection sampling.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u256_range">u256_range</a>(min_incl: u256, max_excl: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u256_range">u256_range</a>(min_incl: u256, max_excl: u256): u256 <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> range = max_excl - min_incl;
    <b>let</b> r0 = <a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>();
    <b>let</b> r1 = <a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>();

    // Will compute sample := (r0 + r1*2^256) % range.

    <b>let</b> sample = r1 % range;
    <b>let</b> i = 0;
    <b>while</b> ({
        <b>spec</b> {
            <b>invariant</b> sample &gt;= 0 && sample &lt; max_excl - min_incl;
        };
        i &lt; 256
    }) {
        sample = <a href="randomness.md#0x1_randomness_safe_add_mod">safe_add_mod</a>(sample, sample, range);
        i = i + 1;
    };

    <b>let</b> sample = <a href="randomness.md#0x1_randomness_safe_add_mod">safe_add_mod</a>(sample, r0 % range, range);
    <b>spec</b> {
        <b>assert</b> sample &gt;= 0 && sample &lt; max_excl - min_incl;
    };

    min_incl + sample
}
</code></pre>



</details>

<a id="0x1_randomness_permutation"></a>

## Function `permutation`

Generate a permutation of <code>[0, 1, ..., n-1]</code> uniformly at random.


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_permutation">permutation</a>(n: u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_permutation">permutation</a>(n: u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; <b>acquires</b> <a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a> {
    <b>let</b> values = <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>[];

    // Initialize into [0, 1, ..., n-1].
    <b>let</b> i = 0;
    <b>while</b> ({
        <b>spec</b> {
            <b>invariant</b> i &lt;= n;
            <b>invariant</b> len(values) == i;
        };
        i &lt; n
    }) {
        std::vector::push_back(&<b>mut</b> values, i);
        i = i + 1;
    };
    <b>spec</b> {
        <b>assert</b> len(values) == n;
    };

    // Shuffle.
    <b>let</b> tail = n - 1;
    <b>while</b> ({
        <b>spec</b> {
            <b>invariant</b> tail &gt;= 0 && tail &lt; len(values);
        };
        tail &gt; 0
    }) {
        <b>let</b> pop_position = <a href="randomness.md#0x1_randomness_u64_range">u64_range</a>(0, tail + 1);
        <b>spec</b> {
            <b>assert</b> pop_position &lt; len(values);
        };
        std::vector::swap(&<b>mut</b> values, pop_position, tail);
        tail = tail - 1;
    };

    values
}
</code></pre>



</details>

<a id="0x1_randomness_safe_add_mod"></a>

## Function `safe_add_mod`

Compute <code>(a + b) % m</code>, assuming <code>m &gt;= 1, 0 &lt;= a &lt; m, 0&lt;= b &lt; m</code>.


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_safe_add_mod">safe_add_mod</a>(a: u256, b: u256, m: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="randomness.md#0x1_randomness_safe_add_mod">safe_add_mod</a>(a: u256, b: u256, m: u256): u256 {
    <b>let</b> neg_b = m - b;
    <b>if</b> (a &lt; neg_b) {
        a + b
    } <b>else</b> {
        a - neg_b
    }
}
</code></pre>



</details>

<a id="0x1_randomness_safe_add_mod_for_verification"></a>

## Function `safe_add_mod_for_verification`



<pre><code>#[verify_only]
<b>fun</b> <a href="randomness.md#0x1_randomness_safe_add_mod_for_verification">safe_add_mod_for_verification</a>(a: u256, b: u256, m: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_safe_add_mod_for_verification">safe_add_mod_for_verification</a>(a: u256, b: u256, m: u256): u256 {
    <b>let</b> neg_b = m - b;
    <b>if</b> (a &lt; neg_b) {
        a + b
    } <b>else</b> {
        a - neg_b
    }
}
</code></pre>



</details>

<a id="0x1_randomness_fetch_and_increment_txn_counter"></a>

## Function `fetch_and_increment_txn_counter`

Fetches and increments a transaction-specific 32-byte randomness-related counter.


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_fetch_and_increment_txn_counter">fetch_and_increment_txn_counter</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="randomness.md#0x1_randomness_fetch_and_increment_txn_counter">fetch_and_increment_txn_counter</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



</details>

<a id="0x1_randomness_is_safe_call"></a>

## Function `is_safe_call`

Called in each randomness generation function to ensure certain safety invariants.
1. Ensure that the TXN that led to the call of this function had a private (or friend) entry function as its TXN payload.
2. TBA


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_is_safe_call">is_safe_call</a>(): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="randomness.md#0x1_randomness_is_safe_call">is_safe_call</a>(): bool;
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification



<pre><code><b>pragma</b> verify = <b>true</b>;
<b>invariant</b> [suspendable] <a href="chain_status.md#0x1_chain_status_is_operating">chain_status::is_operating</a>() ==&gt; <b>exists</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework);
<a id="0x1_randomness_var"></a>
<b>global</b> <a href="randomness.md#0x1_randomness_var">var</a>: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



<a id="@Specification_1_initialize"></a>

### Function `initialize`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_initialize">initialize</a>(framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>




<pre><code><b>let</b> framework_addr = <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(framework);
<b>aborts_if</b> framework_addr != @aptos_framework;
<b>aborts_if</b> <b>exists</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(framework_addr);
<b>ensures</b> <b>global</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(framework_addr).seed == <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_spec_none">option::spec_none</a>&lt;<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;();
</code></pre>



<a id="@Specification_1_on_new_block"></a>

### Function `on_new_block`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="randomness.md#0x1_randomness_on_new_block">on_new_block</a>(vm: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, epoch: u64, round: u64, seed_for_new_block: <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;)
</code></pre>




<pre><code><b>aborts_if</b> <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(vm) != @vm;
<b>ensures</b> <b>exists</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework) ==&gt; <b>global</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework).seed == seed_for_new_block;
<b>ensures</b> <b>exists</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework) ==&gt; <b>global</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework).epoch == epoch;
<b>ensures</b> <b>exists</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework) ==&gt; <b>global</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework).round == round;
</code></pre>



<a id="@Specification_1_next_blob"></a>

### Function `next_blob`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_next_blob">next_blob</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>




<pre><code><b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
<b>let</b> input = b"APTOS_RANDOMNESS";
<b>let</b> <a href="randomness.md#0x1_randomness">randomness</a> = <b>global</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework);
<b>let</b> seed = <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_spec_borrow">option::spec_borrow</a>(<a href="randomness.md#0x1_randomness">randomness</a>.seed);
<b>let</b> txn_hash = <a href="transaction_context.md#0x1_transaction_context_spec_get_txn_hash">transaction_context::spec_get_txn_hash</a>();
<b>let</b> txn_counter = <a href="randomness.md#0x1_randomness_spec_fetch_and_increment_txn_counter">spec_fetch_and_increment_txn_counter</a>();
<b>ensures</b> len(result) == 32;
<b>ensures</b> result == <a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash_sha3_256">hash::sha3_256</a>(concat(concat(concat(input, seed), txn_hash), txn_counter));
</code></pre>




<a id="0x1_randomness_NextBlobAbortsIf"></a>


<pre><code><b>schema</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a> {
    <b>let</b> <a href="randomness.md#0x1_randomness">randomness</a> = <b>global</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework);
    <b>aborts_if</b> <a href="../../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_spec_is_none">option::spec_is_none</a>(<a href="randomness.md#0x1_randomness">randomness</a>.seed);
    <b>aborts_if</b> !<a href="randomness.md#0x1_randomness_spec_is_safe_call">spec_is_safe_call</a>();
    <b>aborts_if</b> !<b>exists</b>&lt;<a href="randomness.md#0x1_randomness_PerBlockRandomness">PerBlockRandomness</a>&gt;(@aptos_framework);
}
</code></pre>



<a id="@Specification_1_u8_integer"></a>

### Function `u8_integer`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u8_integer">u8_integer</a>(): u8
</code></pre>




<pre><code><b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
</code></pre>



<a id="@Specification_1_u16_integer"></a>

### Function `u16_integer`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u16_integer">u16_integer</a>(): u16
</code></pre>




<pre><code><b>pragma</b> unroll = 2;
<b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
</code></pre>



<a id="@Specification_1_u32_integer"></a>

### Function `u32_integer`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u32_integer">u32_integer</a>(): u32
</code></pre>




<pre><code><b>pragma</b> unroll = 4;
<b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
</code></pre>



<a id="@Specification_1_u64_integer"></a>

### Function `u64_integer`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u64_integer">u64_integer</a>(): u64
</code></pre>




<pre><code><b>pragma</b> unroll = 8;
<b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
</code></pre>



<a id="@Specification_1_u128_integer"></a>

### Function `u128_integer`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u128_integer">u128_integer</a>(): u128
</code></pre>




<pre><code><b>pragma</b> unroll = 16;
<b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
</code></pre>



<a id="@Specification_1_u256_integer"></a>

### Function `u256_integer`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u256_integer">u256_integer</a>(): u256
</code></pre>




<pre><code><b>pragma</b> unroll = 32;
<b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
<b>ensures</b> [abstract] result == <a href="randomness.md#0x1_randomness_spec_u256_integer">spec_u256_integer</a>();
</code></pre>




<a id="0x1_randomness_spec_u256_integer"></a>


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_spec_u256_integer">spec_u256_integer</a>(): u256;
</code></pre>



<a id="@Specification_1_u8_range"></a>

### Function `u8_range`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u8_range">u8_range</a>(min_incl: u8, max_excl: u8): u8
</code></pre>




<pre><code><b>pragma</b> verify_duration_estimate = 120;
<b>pragma</b> opaque;
<b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
<b>aborts_if</b> min_incl &gt;= max_excl;
<b>ensures</b> result &gt;= min_incl && result &lt; max_excl;
</code></pre>



<a id="@Specification_1_u64_range"></a>

### Function `u64_range`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u64_range">u64_range</a>(min_incl: u64, max_excl: u64): u64
</code></pre>




<pre><code><b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
<b>aborts_if</b> min_incl &gt;= max_excl;
<b>ensures</b> result &gt;= min_incl && result &lt; max_excl;
</code></pre>



<a id="@Specification_1_u256_range"></a>

### Function `u256_range`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_u256_range">u256_range</a>(min_incl: u256, max_excl: u256): u256
</code></pre>




<pre><code><b>pragma</b> verify_duration_estimate = 120;
<b>include</b> <a href="randomness.md#0x1_randomness_NextBlobAbortsIf">NextBlobAbortsIf</a>;
<b>aborts_if</b> min_incl &gt;= max_excl;
<b>ensures</b> result &gt;= min_incl && result &lt; max_excl;
</code></pre>



<a id="@Specification_1_permutation"></a>

### Function `permutation`


<pre><code><b>public</b> <b>fun</b> <a href="randomness.md#0x1_randomness_permutation">permutation</a>(n: u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>aborts_if</b> n == 0;
</code></pre>



<a id="@Specification_1_safe_add_mod_for_verification"></a>

### Function `safe_add_mod_for_verification`


<pre><code>#[verify_only]
<b>fun</b> <a href="randomness.md#0x1_randomness_safe_add_mod_for_verification">safe_add_mod_for_verification</a>(a: u256, b: u256, m: u256): u256
</code></pre>




<pre><code><b>aborts_if</b> m &lt; b;
<b>aborts_if</b> a &lt; m - b && a + b &gt; MAX_U256;
<b>ensures</b> result == <a href="randomness.md#0x1_randomness_spec_safe_add_mod">spec_safe_add_mod</a>(a, b, m);
</code></pre>




<a id="0x1_randomness_spec_safe_add_mod"></a>


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_spec_safe_add_mod">spec_safe_add_mod</a>(a: u256, b: u256, m: u256): u256 {
   <b>if</b> (a &lt; m - b) {
       a + b
   } <b>else</b> {
       a - (m - b)
   }
}
</code></pre>



<a id="@Specification_1_fetch_and_increment_txn_counter"></a>

### Function `fetch_and_increment_txn_counter`


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_fetch_and_increment_txn_counter">fetch_and_increment_txn_counter</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] result == <a href="randomness.md#0x1_randomness_spec_fetch_and_increment_txn_counter">spec_fetch_and_increment_txn_counter</a>();
</code></pre>




<a id="0x1_randomness_spec_fetch_and_increment_txn_counter"></a>


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_spec_fetch_and_increment_txn_counter">spec_fetch_and_increment_txn_counter</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



<a id="@Specification_1_is_safe_call"></a>

### Function `is_safe_call`


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_is_safe_call">is_safe_call</a>(): bool
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] result == <a href="randomness.md#0x1_randomness_spec_is_safe_call">spec_is_safe_call</a>();
</code></pre>




<a id="0x1_randomness_spec_is_safe_call"></a>


<pre><code><b>fun</b> <a href="randomness.md#0x1_randomness_spec_is_safe_call">spec_is_safe_call</a>(): bool;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY