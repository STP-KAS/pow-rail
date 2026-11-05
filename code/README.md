# The bytes

The short branch pages say what to seal. This page says how a Kaspa node commits to those bytes, and the crate next to it builds them.

Experimental. Not advice. Not Kaspa core. Not a wallet. `cargo test` in `kaspa-seal` checks the hash against vectors published in the node. It does not broadcast a transaction and it does not take a key.

Node pin: [rusty-kaspa v2.1.0](https://github.com/kaspanet/rusty-kaspa/releases/tag/v2.1.0), commit [`01b532e8b553523216471682649693af92f0fd16`](https://github.com/kaspanet/rusty-kaspa/commit/01b532e8b553523216471682649693af92f0fd16). Line numbers below are that commit. Master has moved. An open pull, [kaspanet/rusty-kaspa#1141](https://github.com/kaspanet/rusty-kaspa/pull/1141), would change the transient-mass representation. It was open on this pass. The pin stays the tag.

Compiler pin: [SilverScript v1.0.0](https://github.com/kaspanet/silverscript/releases/tag/v1.0.0), commit `3ed973335b59269293564805cc2c58a14595ec03`.

## What the crate does

[`kaspa-seal`](kaspa-seal) does three things.

1. It computes `PayloadDigest`, the keyed BLAKE3 the node uses as the payload half of a version-1 transaction id.
2. It packs an 81-byte seal: a 13-byte magic, a version, a branch, a kind, a flag, and one or two 32-byte file hashes.
3. It builds a 20-byte user-lane id in the shape [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) section 2.1 requires, or the 20 zero bytes of the native subnetwork.

The file hash is unkeyed BLAKE3. The payload digest is a different function. The tests refuse to treat them as the same hash.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Launch,
    kaspa_seal::Kind::PublicFile,
    file_bytes,
);
let payload_half = kaspa_seal::payload_digest(&payload);
```

`payload_half` is 32 bytes. It is not the transaction id.

## How version 1 commits to a payload

`Transaction` in [`consensus/core/src/tx.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/tx.rs#L254-L261) carries `payload: Vec<u8>` beside the inputs, the outputs, the lock time, the subnetwork id, and the gas.

`hashing::tx::id` ([`hashing/tx.rs` lines 34–36](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L34-L36)) sends version 0 through `id_v0` and every other version through `id_v1`.

`id_v1` ([lines 207–218](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L207-L218)) does this:

1. `payload_digest` of `tx.payload`.
2. `TransactionRest` over the transaction with the payload, the signature scripts, and the mass field left out. The flag set is `EXCLUDE_PAYLOAD | EXCLUDE_SIGNATURE_SCRIPT | EXCLUDE_MASS_COMMIT` ([lines 230–235](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L230-L235)).
3. `TransactionV1Id` over those two 32-byte digests, payload first.

[`constants.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs#L5-L15) still names `TX_VERSION` as 0 and `TX_VERSION_TOCCATA` as 1. Version 1 is the version that adds covenant bindings, compute budgets, user lanes, and gas. The id function keys off `version == 0`, not off the name of the constant. A seal that wants the split digest uses version 1.

A version-0 id still covers the payload, inside one older hash, because `id_v0` does not set `EXCLUDE_PAYLOAD`. The separated `PayloadDigest` is the version-1 object. This crate computes that object.

`payload_digest` ([lines 221–223](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L221-L223)) returns a hardcoded constant for an empty payload and `PayloadDigest::hash` otherwise. The node's own test, `test_zero_payload_digest`, asserts the constant equals `PayloadDigest::hash([])`.

## The key

`blake3_hasher!` in [`crypto/hashes/src/hashers.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/hashes/src/hashers.rs#L120-L138) builds the BLAKE3 key in a const block: a 32-byte array of zeros, then the domain bytes copied in from the front. [`PayloadDigest`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/hashes/src/hashers.rs#L39-L44) uses the domain `b"PayloadDigest"`. That string is 13 bytes, so the remaining 19 key bytes stay zero.

`TransactionRest` and `TransactionV1Id` are the same macro with their own domains. Three different keys. Mixing them up produces a hash the node will not treat as an id.

`Hasher::hash` ([lines 11–15](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/hashes/src/hashers.rs#L11-L15)) is one-shot: new hasher, `update` once, `finalize`.

## The test vectors are incremental

`test_blake3_vectors` ([lines 289–355](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/hashes/src/hashers.rs#L289-L355)) builds one `PayloadDigest` hasher and updates it with five inputs in order, without a reset. The comment in `run_test_vector` says so. The five expected strings are hashes of the running concatenation.

The empty one-shot matches the first string, and it matches `ZERO_PAYLOAD_DIGEST`:

```text
9c0ca2acb45e92ffe6ceb4ae29188b35c82d9676cdd3ce067fd6ccc30a9c4a38
```

An empty `update` does not change BLAKE3 state, so the second string is also the one-shot of the single byte `0x01`:

```text
a69883bfb2bb000249639d8bbf5019fd08ceb7daa7a86f170530522dfedba124
```

The third, fourth, and fifth strings are not one-shot hashes of the 32-byte sample, of 64 bytes of `0x2a`, or of 8 zero bytes. The crate's tests assert that inequality, so a later edit cannot "simplify" the vectors into one-shot checks and still pass.

## The seal

| Offset | Length | Field |
| --- | --- | --- |
| 0 | 13 | ASCII `pow-rail/seal` |
| 13 | 1 | version `1` |
| 14 | 1 | branch, 1 through 15, in the order of the pages |
| 15 | 1 | kind, listed below |
| 16 | 1 | flags. Bit 0 is set when a second hash is present. Other bits are rejected by the decoder. |
| 17 | 32 | unkeyed BLAKE3 of the file the parties keep |
| 49 | 32 | second file hash, or zeros |

81 bytes in total. The primary field is a hash. The file itself does not fit, and the layout does not have a slot for it.

Kinds:

| Value | Name | Use |
| --- | --- | --- |
| 1 | `PublicFile` | A file that is already public, or that the verifier will be handed. |
| 2 | `Receipt` | A receipt, a job acceptance, a fare, a session, a meter row. |
| 3 | `WeightOrImage` | A published weight file, a firmware image, a software build. |
| 4 | `Inspection` | An inspection pack, a traveler, a calibration, a shipment record. |
| 5 | `Eval` | A published eval, a system card, a notes run. |
| 6 | `PassportExtract` | A battery or parts extract. |
| 7 | `UnclassifiedDelivery` | An unclassified delivery record. |
| 8 | `PublicStatus` | An already-public status page or on/off notice. |

A short secret is a bad input. Unkeyed BLAKE3 of a guessable string can be searched. Seal a file the verifier is supposed to see. Keep export-controlled designs, prompts, neural recordings, vehicle traces, and recipes off the input as well as off the payload.

## Putting the bytes in a transaction

The builder docs show `payload` as a field of `accountsSend`:

<https://docs.kaspa.org/integrate/transaction-payload>

That page also says a standard transaction is capped at mass `100_000`, and that a payload-dominated transaction then fits about 25 KB because transient mass is serialized size times 4. The factor of 4 is in the v2.1.0 tag. The symbol `100_000` as a standard-transaction mass cap was not found in that tag.

What the tag does contain:

- `TRANSIENT_BYTE_TO_MASS_FACTOR = 4` in [`constants.rs` lines 28–31](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs#L28-L31). The comment above it still says a block mass of 500_000 limits the body to 125_000 bytes, citing KIP-13.
- Mainnet `block_mass_limits` in [`params.rs` line 626](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/params.rs#L617-L626) is compute 500_000, storage 500_000, transient 1_000_000. The comment on the signature-script limit says the transient footprint of a block is 250 KB. 1_000_000 / 4 = 250_000. The 125_000 sentence and the 250 KB sentence are both in the tag. The struct that mainnet uses is the 1_000_000 transient limit.
- `calc_non_contextual_masses` sets `transient_mass = size * TRANSIENT_BYTE_TO_MASS_FACTOR` ([`mass/mod.rs` line 367](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/mod.rs#L340-L367)). Compute mass is a different sum: size times `mass_per_tx_byte` (mainnet value 1), plus script-public-key bytes times 10, plus either sigop mass or, for version 1, compute-budget mass.
- `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE = 100_000` in [`mining/src/mempool/config.rs` line 20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20). `SOMPI_PER_KASPA` is 100_000_000, so this constant is 0.001 KAS. It is a fee floor in the mempool config. It is not a mass cap. Miners can still prefer a higher fee.
- Standardness in `check_transaction_standard.rs` rejects non-standard scripts and P2SH inputs with more than `MAX_STANDARD_P2SH_SIG_OPS` (15). It does not mention a 100_000 mass cap.

An 81-byte payload is far inside the transient limit the mainnet struct names. That is the claim this crate makes. It does not claim the docs-site 25 KB figure is a consensus constant, because that constant was not in the tag.

This repository does not wrap a wallet and does not take a secret. A wallet that already speaks to a node can place the 81 bytes in the payload field the docs describe. Pay with a QR code, a `kaspa:` URI, or a pasted txid.

## Lanes

[KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) is Active. Commit [`e4ae2332`](https://github.com/kaspanet/kips/commit/e4ae2332117b5cb68bd6188e065ef885b6d17939) (15 July 2026) is the commit that marked KIP-16, KIP-17, KIP-20, and KIP-21 Active together. The header value `accepted_id_merkle_root` becomes `SeqCommit(B)`. The KIP's own reader map is: group accepted transactions by `subnetwork_id`, update each touched lane tip, commit the active-lane tips in a sparse Merkle tree, and also commit block context and mergeset miner payloads under the same chain.

Section 2.1, read from master on 5 November 2026:

- A lane id is 20 bytes.
- Native is `0x00` followed by 19 zeros.
- Coinbase is `0x01` followed by 19 zeros.
- A user lane is a 4-byte namespace followed by 16 zeros, and at least one of `namespace[1]`, `namespace[2]`, `namespace[3]` is non-zero. Any other id with a 19-byte zero tail is reserved and invalid.

`user_lane_id` implements that check and nothing else. It does not submit the transaction. SilverScript v1.0.0 types `OpTxSubnetId` as `byte[20]` (`silverscript-lang/tests/compiler_tests.rs` in that tag). The node field is `Transaction::subnetwork_id`.

Mainnet lane limits in the tag ([`config/constants.rs` lines 97–100](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L93-L100)): 50 lanes per block, 1_000_000_000 gas per lane. The comment says that at 10 blocks a second this allows a worst case of 500 lane-tip updates a second. [Kaspa Explained](https://kaspaexplained.com/kaspa-tps-explained) describes the same two numbers as the cap on user-lane churn and on gas per lane. The constant, not the article, is the pin.

A seal on the native subnetwork is already ordered. Every accepted transaction sits in the lane of its subnetwork id, and the header commits to those lane tips. A separate user lane is for a prover who should follow one program and not the rest of the DAG. The KIP's goal is prover cost proportional to that lane's activity. A factory, a constellation, and a charger do not each need a lane on day one. They need a lane when someone is actually proving that lane.

[KIP-2](https://github.com/kaspanet/kips/blob/master/kip-0002.md) (DAGKnight) was still Proposed on this pass. It is not in v2.1.0. Nothing here waits on it.

## The covenant

[`covenant/seal_escrow.sil`](covenant/seal_escrow.sil) locks KAS, not a dollar and not a token. The spend releases the input amount, minus a fee the script names, to the buyer or the seller, and only if the spending transaction's payload is an 81-byte seal of the document hash the covenant was constructed with.

The introspection opcodes are in the v2.1.0 node: `OpTxPayloadLen` (`0xc4`) and `OpTxPayloadSubstr` (`0xb8`). The older example `announcement.sil` in the SilverScript tag still has a comment that says the payload field is a TODO. The compiler in that same tag already lowers `OpTxPayloadLen` and `OpTxPayloadSubstr`. The escrow follows the opcodes, not the TODO.

The script reads the amount from `this.activeInputIndex` only. It does not call `readInputState` on any other input. [KIP-17](https://github.com/kaspanet/kips/blob/master/kip-0017.md) is the covenant and introspection KIP. [KIP-20](https://github.com/kaspanet/kips/blob/master/kip-0020.md) is the covenant-id lineage. Both were Active in the same status commit as KIP-21. [Kaspa Explained](https://kaspaexplained.com/kaspa-covenants-explained) is a secondary write-up of the same idea: the script constrains the next output, and a covenant id stops a lookalike UTXO. The KIP is the rule. The article is the map.

The script does not check the branch byte. The signer does, because the signer sees the payload. Details, the dummy constructor arguments, and the compiled artifact are in [covenant/README.md](covenant/README.md).

[KCC-20](https://github.com/kaspanet/kccs/blob/main/kcc-0020.md) was Final on this pass. It is a layout for a fungible covenant. A battery, a wafer, a receipt, and a person are not that layout. [KCC-1](https://github.com/kaspanet/kccs/blob/main/kcc-0001.md) is the vocabulary KCC-20 requires. Final means implementers who want to agree have a text. It does not mint a settlement currency.

Kasplex is not this. The [Kaspa wiki page](https://wiki.kaspa.org/en/Kasplex_KRC_20) calls Kasplex a third-party data-insertion protocol, indexer, and API, and KRC-20 a token and NFT convention on top of that. The [wiki FAQ](https://wiki.kaspa.org/faq) says the L2 set people can point at is KRC-20 and Kasplex zkEVM, with smart-contract work outside the core team. KRC-20 is not KCC-20. This repository does not emit KRC-20 payloads.

## Precompiles

[KIP-16](https://github.com/kaspanet/kips/blob/master/kip-0016.md) is Active. `OpZkPrecompile` is opcode `0xa6`. In the node ([`opcodes/mod.rs` lines 758–771](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/opcodes/mod.rs#L758-L771)) the opcode reads a tag, charges that tag's cost, calls `verify_zk`, and pushes true when verification returns.

The tag enum in [`zk_precompiles/tags.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs) has two variants:

| Tag | Name | Script-unit cost in the tag file |
| --- | --- | --- |
| `0x20` | Groth16 | `1000 * 140` |
| `0x21` | RISC0-Succinct | `1000 * 250` |

Any other byte is `ZkIntegrityError::UnknownTag`. The file's tests expect a block's compute-mass limit to hold 3 Groth16 verifications or 2 succinct verifications. KIP-16's text names the same two tags and says the opcode dispatches to predefined programs. It is not a general computer.

A proof checks a statement that was actually proved: a program identity and a public output, in the shape that precompile expects. It does not observe a rocket, a car, a grid, or a training run by itself. [Kaspa Explained](https://kaspaexplained.com/toccata-explained) says the same limit in plainer words. The opcode is the source.

[vprogs](https://github.com/kaspanet/vprogs) had no release on this pass. A based app that executes off-chain and settles a proof is described in the [Toccata guide](https://docs.kaspa.org/toccata) and in [covenant state](https://docs.kaspa.org/toccata/covenant-state). Designing one for a company system waits on a tagged runtime and on a statement the company is willing to prove in public. The seal above does not wait on that.

## What this crate will not do

It will not build a version-1 transaction id. That needs the rest digest, which covers inputs, outputs, lock time, subnetwork, and gas. Those bytes are the wallet's transaction, not the seal.

It will not talk to a node. Inclusion is a miner's act. The digest is how anyone later shows which payload was committed.

It will not move a license, a beam, a contactor, a breaker, or a vehicle. The companies that operate those remain the companies a government can order.
