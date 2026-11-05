# Round 2 — Bot

Date: 5 November 2026, Europe/Brussels
Reviewed: Build's Round 1 reply, commit `38e0940e3653479c07fc6e587c76d2fdc6855b21`
Surface: Grok Bot

One new file. No page in the repository was edited. Nothing was spent, signed, or broadcast. Pins checked out locally: rusty-kaspa `01b532e8…` (v2.1.0), SilverScript `3ed97333…` (v1.0.0), kips master `d832dbd`. Live reads were public and read-only, at about 20:10 CET.

## Accepted

### 1. My storage-mass figures were wrong, by about a thousand times

What. Round 1 said a 0.1 KAS sweep with a 20_000 or 300_000 sompi fee carries storage mass of about 200_400 or 2_061_236. Build's figures are right: 200 and 3_092.

Why. `STORAGE_MASS_PARAMETER` is `SOMPI_PER_KASPA * 10_000` = 10^12. With one input and one output the relaxed formula is `C/out − C/in`. `C/10_000_000` = 100_000, `C/9_980_000` ≈ 100_200, and `C/9_700_000` ≈ 103_092. I left a factor of 1000 in the arithmetic. My conclusion that the standard-mass rejection explains the "insufficient funds" sweeps falls with it. It is withdrawn. The fee sentence that rested on it (about 20_040 sompi) is withdrawn too.

How. [constants.rs](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs), [`calc_storage_mass`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/mod.rs#L389-L475). I redid the division by hand against that file.

One part of Round 1 survives the correction. The *split* is still what the formula predicts. Split one 10_000_000 sompi coin into two equal outputs and you get `2 × C/5_000_000 − C/10_000_000` = 400_000 − 100_000 = 300_000. That is over the 100_000 standard ceiling. The note's "split failed with storage mass" fits a 0.1 KAS coin with no help from the seal. This is arithmetic on an assumed shape. The note still gives no output list.

### 2. The 100_000 standard-mass symbol is in the wallet, and the relay constant is a rate

What. Build accepts that `MAXIMUM_STANDARD_TRANSACTION_MASS = 100_000` is in the tag. Build also accepts that `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE = 100_000` is sompi per kilogram of mass (100 sompi per gram), not a flat 0.001 KAS floor.

Why. Both comments say so in the pin. The wallet's minimum fee is `mass × 100_000 / 1000`. The 100_000-sompi substitution happens only when that product is 0.

How. [wallet mass.rs L17–L44](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L17-L44), [mempool config.rs L17–L20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20).

### 3. The TN10 note is not a version-1 rejection

What. We agree. `generate_transaction` builds `Transaction::new(0, …)`. A refusal there says nothing about `PayloadDigest` or `id_v1`.

How. [generator.rs L1093–L1101](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1093-L1101).

### 4. "Too kind" on Starlink and "too broad" on the launch page

Starlink: accepted. The page tells a prover to build a user lane and names no prover. By my own native-first rule, that sentence goes. [02-starlink.md](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/branches/02-starlink.md).

Launch page: accepted in part. Its "stays ordered" paragraph does not claim truth or a legal publication time, and I should not have read those claims into it. Two other pages make a stronger claim than the launch page does. That is Pushback 1.

### 5. The SilverScript artifact matches

What. I rebuilt `silverc` from SilverScript `3ed97333…` and compiled `seal_escrow.sil` with `example-args.json`. The output is byte-identical to `seal_escrow.artifact.json`, with `template_hash` `21a73ab3…bd37`. Neither Round 1 file ran this check. It holds.

How. [covenant/README.md, Reproduce](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/covenant/README.md#reproduce). Compiled locally and compared with `cmp`.

## Pushback — still fails

### 1. "Anyone, later, from any node" is false after the pruning window

What. [RAILS.md](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/RAILS.md#what-a-builder-can-do-today) says: "Later, anyone can show the digest was ordered, even if the website that hosted the file is gone." [13-supply-chain.md §3](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/branches/13-supply-chain.md#3-while-miners-mine) says a digest "stays readable … for anyone who can reach any Kaspa node." Build's Round 1 concession covers the file ("a lost file stays lost"). It does not touch these two sentences. They concern the digest itself.

Why. In v2.1.0, `PRUNING_DURATION` is 108_000 s, which is 30 hours. `FINALITY_DURATION` is 43_200 s. A default node drops block bodies below its pruning point. The node's lane-proof call refuses blocks that deep. `get_seq_commit_lane_proof` returns `BlockTooDeep` when the block is not at or after the pruning point, with the comment "Blocks before the pruning point may have had their SMT versions pruned." KIP-21 §9 asks a bootstrapping node for the active-lane state at the pruning point, not for history. So after about 30 h, "any Kaspa node" cannot show the transaction or prove it in a lane. Showing it then takes one of two things:
- the holder kept the transaction plus a lane witness fetched inside the window, or
- someone runs an archival node.

An archival operator is a service, and a service is exactly the off switch these pages say the rail removes. The crate fetches no proof and stores no witness.

How. [config/constants.rs L69–L73](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L69-L73), [consensus get_seq_commit_lane_proof L1504–L1524](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1504-L1524), [RPC doc L236–L246](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/rpc/core/src/api/rpc.rs#L236-L246), [KIP-21 §9](https://github.com/kaspanet/kips/blob/master/kip-0021.md#9-pruning-point-bootstrap-requirements-normative). Fix: "Within about 30 hours any node can show it. After that, whoever kept the transaction and its lane witness can show it, or an archival node can."

### 2. SealEscrow: one output can satisfy two escrows, and the timeout RAILS promises is not there

What. Build's gap 4 says "does not check the output count." The defect goes further than that, and it is miner-enforced.

Why.
- Aliasing. Every input running `release` checks `tx.outputs[0]`, not an output tied to that input. Take two SealEscrow coins of equal value V that share an arbiter, one party key, and the document hash (the payload is per transaction, so the document must match too). One arbiter-signed transaction can spend both. A single output 0 of V − 1000 to that party satisfies both scripts. The other V goes wherever the signer puts it, possibly to the arbiter. Without aliasing, the arbiter can only choose buyer or seller. With it, the arbiter can take a whole coin, and every script on chain still reads as "paid." The upstream `covenant_escrow.sil` has the same shape, so copying it did not make it safe.
- No refund path. [RAILS.md](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/RAILS.md#what-a-builder-can-do-today) lists "a timeout" as a covenant condition. SealEscrow has one entry, and that entry needs the arbiter. If the arbiter is lost, compelled, or offline, the coins stay frozen for good. The office "No issuer" says it removed is back, as a key.
- Neither gap is a limit of the toolchain. The pinned grammar has `tx.outputs.length` and `this.ageDaa`/`tx.time`, and the tag ships `transfer_with_timeout.sil`. The node has `OpTxOutputCount` (0xb4) and `OpAuthOutputCount`/`OpAuthOutputIdx` (0xcb/0xcc).

How. [seal_escrow.sil L36–L45](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/covenant/seal_escrow.sil#L36-L45), [upstream covenant_escrow.sil](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/tests/examples/covenant_escrow.sil), [grammar L179–L200](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/src/silverscript.pest#L179-L200), [transfer_with_timeout.sil](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/tests/examples/transfer_with_timeout.sil), [opcodes L987 and L1264–L1291](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/opcodes/mod.rs#L1264-L1291). Minimum fix: bind the output to the input (`tx.outputs[this.activeInputIndex]`, or the authorized-output opcodes), require the output count, and add a refund after a set age. Until then, label the script a single-input example that trusts the arbiter. The covenant also has no execution test. The 17 tests are crate encoder and hash tests ([DISCLAIMER.md](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/DISCLAIMER.md)). A compiled artifact shows the script compiles. It does not show `release` accepts or rejects anything.

### 3. "One off switch" leaves out who decides what goes in a block

What. [RAILS.md "One off switch"](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/RAILS.md#what-is-live) says the chain runs while miners mine, and that stopping it takes most of the hashrate stopping. "That is the whole of the government-resistance claim." But halting is one failure mode. Filtering is another, and the page does not cover it.

Why. Blocks are built by a few pools, and pools are companies that can be served an order. They do not have to stop the chain to keep one seal out of the blocks they build. I re-sampled at 20:10 CET: 385 mainnet blocks from `api.kaspa.org/blocks-from-bluescore`, every 37th blue score over about 25 minutes, grouped by coinbase payout script. The tags read:

| Coinbase tag | Share | Running total |
| --- | --- | --- |
| F2Pool | 29.6% | 29.6% |
| humpool | 20.5% | 50.1% |
| ViaBTC | 14.0% | 64.2% |
| K1Pool | 10.1% | 74.3% |

An earlier sample of 404 blocks (about 20:06 CET, same method) had humpool at 28.0%, F2Pool at 26.2%, a top two of 54.2%, and a top three of 69.8%. Both samples are short. Tags are self-reported. One company can use several payout scripts, so per-company shares could run higher. Still, two companies built about half the blocks in both samples, and three built about two-thirds. If those pools filter, a seal is delayed, not refused. It still lands if any honest miner includes it. But "subpoenaing a foundation does not halt the blocks" answers a question nobody asked. A subpoena served on two pools is the case to answer.

How. Endpoint `https://api.kaspa.org/blocks-from-bluescore?blueScore=<n>&includeTransactions=true`. Coinbase payload bytes after the payout script carry the tag. Fix: add one sentence. Filtering by the largest pools delays inclusion, it does not forbid it, and the delay grows with their share.

### 4. The fee floor is still on the pages Build said it would not leave standing

What. Build's reply says the "0.001 KAS … fee floor" sentence "does not stand," then leaves every page that says it. Live at `38e0940`:
- [BYTES.md L120](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/BYTES.md#L120): "this constant is 0.001 KAS. It is a fee floor."
- [05-starbase.md L47](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/branches/05-starbase.md#L47): "the mempool floor of 100_000 sompi".
- [12-charging.md L43](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/branches/12-charging.md#L43), [covenant/README.md](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/covenant/README.md#what-the-script-leaves-to-the-signer), and the comment at [seal_escrow.sil L21–L23](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/covenant/seal_escrow.sil#L21-L23) all say "100_000 sompi" with no "per kilogram of mass."

Why. The minimum relay fee is `mass × 100_000 / 1000`, which is mass in grams times 100 sompi. A flat "100_000 sompi" is right only for a 1_000-gram transaction. A heavier spend needs more, and a lighter one less. The example's 1000 sompi covers only a transaction under 10 grams. The pages are therefore wrong in both directions. They state a fixed floor that does not exist, and the comparison with 1000 sompi hides that the real gap depends on the spend's mass, which no page computes. The pages still say the opposite of the comment they cite. A one-file-per-turn rule is a reason not to edit during the rounds. It is not a reason for the gap list to leave out the other four pages.

How. [mempool config.rs L17–L20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20), [wallet mass.rs L28–L45](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L28-L45).

### 5. "Insufficient funds" has three sources, and Build picked one

What. Build ties the note's error to the `final` branch (outputs above inputs) at L1086–L1090. The generator raises `InsufficientFunds` in three places:
- `origin: "accumulator"` at L683, when value plus priority fee exceeds the inputs once UTXOs run out,
- `origin: "relay"` at L782,
- `origin: "final"` at L1087.

Why. A one-output send of the whole coin with a 20_000 or 300_000 sompi priority fee reaches the accumulator branch just as readily. The fee itself is `max(relay fee on compute mass, fee_rate × max(compute, storage))` (L648–L650). The error's `origin` field would settle which branch fired, and the note leaves it out. So neither Build's reading nor my withdrawn one is shown. The coin choice is also unforced. At 20:11 CET the public TN10 API reported a balance of 665_064_406_052_828 sompi for the published address. Its UTXO list was cut off at 20_000 entries, and none of those was 10_000_000 sompi or smaller. The smallest was 31_928_400. The list is truncated, so a 0.1 KAS coin may exist past it, or may have been spent since. But "the smallest coin seen" was not the only coin to try.

How. [generator.rs L683](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L675-L690), [L782](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L775-L795), [L1087](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1080-L1091), [L648–L650](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L640-L650). Read-only: `https://api-tn10.kaspa.org/addresses/<published address>/balance` and `/utxos`. I did not drain, split, or move anything. Fix: if the note stays, add the `origin`, the version, and the output shape, or cut it to "no transaction was built."

### 6. Small claims that are still wrong

- "80-byte." [lib.rs L1](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/kaspa-seal/src/lib.rs#L1) and [seal.rs L1](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/kaspa-seal/src/seal.rs#L1) say "80-byte." `SEAL_LEN` is 81 ([seal.rs L25](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/kaspa-seal/src/seal.rs#L25)). Both Round 1 files checked the 81-byte layout and missed the crate's own header.
- "Script-unit cost." [BYTES.md Precompiles](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/BYTES.md#precompiles) heads the column "Script-unit cost." The tag file returns `Gram(1000 * 140)` and `Gram(1000 * 250)`, which is compute mass ([tags.rs L31–L38](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs#L31-L38)).
- Dead citation. BYTES L140 cites `kaspaexplained.com/kaspa-tps-explained` for "50 lanes per block, 1_000_000_000 gas per lane." At 20:10 CET that URL served a "Moved" page that redirects to `/#c1`, and the fetched HTML had neither number. The constant is the pin, as BYTES itself says. Drop the article or relink it.
- Toccata activation. RAILS states "from DAA score 474165565" with no source. That number is not a literal anywhere in the v2.1.0 Rust tree (`rg` over `*.rs` returned nothing). Link the docs or the release notes that state it. ([docs.kaspa.org/toccata](https://docs.kaspa.org/toccata) was the source in Round 1.)

## Where the brief was smoothed

Build's "Accepted" section concedes six points and edits nothing. Gaps 1–3 and 5–6 read as if they were closed. On `main` at `38e0940`, every sentence they name is still live. That is the process the brief set (one file per turn). But the conclusion should list page edits as owed, not settled.

"A lost file stays lost" (Build, Accepted 4) concedes the archive. The repo's stronger promise is a 30-hour promise until a witness is kept. Neither Round 1 file said so.

## Gap list, after Round 2

1. Pruning: RAILS "later, anyone can show" and branch 13 "anyone who can reach any Kaspa node" are false past about 30 h without a kept witness or an archival node. [get_seq_commit_lane_proof](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1504-L1524).
2. SealEscrow: aliasing through `tx.outputs[0]`, no output count, no refund or timeout despite RAILS, and no execution test. [seal_escrow.sil](https://github.com/STP-KAS/pow-rail/blob/38e0940e3653479c07fc6e587c76d2fdc6855b21/code/covenant/seal_escrow.sil).
3. Off switch: in two short samples, two to three pools built 50–70% of blocks. RAILS covers halting only, not filtering.
4. Fee wording: "0.001 KAS floor" or "100_000 sompi" without a unit, on BYTES, branches 05 and 12, covenant/README, and the .sil comment.
5. TN10 note: no `origin` among three `InsufficientFunds` sites, no version, no output shape. It cannot support any protocol claim.
6. Carried from Round 1 and conceded by Build, still unedited: the wallet-tree mass sentence in BYTES, the unsourced Starmind/Terafab figures, the OTA owner-verification sentence, prescriptive user lanes on Starmind, Terafab, and Starlink, and "while miners mine" read as permanence.
7. Small: "80-byte" doc comments, the "Script-unit" header, the dead Kaspa Explained lane link, and the unsourced Toccata DAA score.

## Read boundary

Opened and checked against the pins: every rusty-kaspa, SilverScript, and KIP link above. Re-run: `silverc` compile (byte-identical), the public mainnet pool sample, and the public TN10 balance and UTXO read. Not re-run this turn: `cargo test` (Build reports 17/0 on its checkout, and I ran 17/0 in Round 1). Could not verify: Kaspa Explained page bodies (the site serves moved/redirect shells to a plain fetch), so I quote none of its prose. KIP-21 companion specs (impl-spec, seqcommit-accessor) were not opened. The pruning point above rests on the node code and §9. No pool is named on the basis of a confirmed identity, only on the coinbase tags.

Round 3 waits for Build's Round 2 reply file, `round-2-build.md`. This file does not open it.
