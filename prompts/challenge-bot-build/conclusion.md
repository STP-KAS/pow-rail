# Bot-Build conclusion

Date: 5 November 2026, Europe/Brussels
Surface: Grok Build
Rounds: Bot `68c9382`, `6b50e91`, `b165f198ab919e4c02934b1e3de90c89ae2ce43d`. Build `38e0940`, `8d56e2a`, `8330151`.
There is no fourth attack. Bot does not write a second conclusion.

## What

The pages said a default node could show a sealed digest later, that locking KAS on this rail includes a timeout, and that the relay constant is a flat 100_000 sompi. The pins and `seal_escrow.sil` do not say that. The challenge files had already conceded the gaps. A concession in a challenge file does not change what a reader of the rail sees. This commit puts the agreed sentences on the pages.

## Why

Proof of work here is a public clock and a KAS lock. A reader who takes the Easy page as the product will plan a record they cannot fetch from a default node after the pruning point, or a payment that stays frozen when the arbiter never signs. Both are real limits. Neither is an office that can mint KAS or switch the chain off.

The fee wording was the same class of error. `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE` is sompi per kilogram of mass. Calling it 0.001 KAS, or a floor of 100_000 sompi, makes the example's 1000 sompi look like a shortfall against a number that is not a floor.

## How

The node pin is rusty-kaspa v2.1.0, commit `01b532e8b553523216471682649693af92f0fd16`. The compiler pin is SilverScript v1.0.0, commit `3ed973335b59269293564805cc2c58a14595ec03`. KIP files cited in the rounds are master, not that node tag.

Pruning: [`config/constants.rs` lines 69–73](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L69-L73) sets `FINALITY_DURATION` to 43_200 seconds and `PRUNING_DURATION` to 108_000 seconds. [`consensus/mod.rs` lines 1519–1524](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1519-L1524) returns `BlockTooDeep` from `get_seq_commit_lane_proof` when the block is not at or after the pruning point.

Fee: [`mining/src/mempool/config.rs` lines 17–20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20) and the wallet copy at [`mass.rs` lines 17–20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L17-L20). The product is mass times 100_000, divided by 1000. `MAXIMUM_STANDARD_TRANSACTION_MASS` is in [`mass.rs` lines 26–28](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L26-L28). It is a wallet standardness ceiling. It is not the block transient limit of 1_000_000.

Escrow: `seal_escrow.sil` still has one entry, `release`. It requires the arbiter signature and the document hash. It checks `tx.outputs[0]` only. It does not count outputs. It names 1000 sompi. It has no timeout. The script logic was not changed in this commit. The comment was.

Precompile: [`tags.rs` lines 25–38](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs#L25-L38). `cost()` returns `ScriptUnits`. The match arm is `Gram(1000 * 140)` and `Gram(1000 * 250)`. [`SCRIPT_UNITS_PER_GRAM`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/units.rs) is 100. The script-unit figures are 14_000_000 and 25_000_000.

Not re-run for this commit: `silverc`, `cargo test`, the pool sample, a TN10 balance read, a fetch of `docs.kaspa.org/toccata`, and a fetch of the moved Kaspa Explained lane URL. KIP-21 companion specs were not opened. Bot's Round 2 report that the compiled artifact matched stays Bot's report.

## Conclusions

What stands:

- The seal layout is 81 bytes. Version 1 is the hash split a seal wants. The native subnetwork already orders that payload. A user lane is for a prover who will follow that program.
- The local encoder tests were 17 passed and 0 failed when they were last run, in Round 1. They are hash tests. They are not an on-chain inclusion.
- Inside the pruning window a default node can show the transaction. After that, showing it takes a kept copy and a lane witness fetched in time, or a node that kept the history. A lost file stays lost. An archival node is not an issuer of KAS.
- Storage mass for the shapes both sides accepted: one output at a 20_000 sompi fee is 200; one output at a 300_000 sompi fee is 3_092; an assumed equal split into two outputs of 5_000_000 sompi is 300_000. The last figure is that shape, not a reconstruction of an unpublished split.
- The relay constant is 100 sompi per gram. 1000 sompi covers 10 grams. A flat 100_000 sompi is the product at 1_000 grams.
- `InsufficientFunds` has three origins in the generator: accumulator, relay, and final. The TN10 note names none of them. It records an encoder hex and no txid.
- `seal_escrow.sil` can satisfy two equal coins with one output 0. Without that alias the arbiter chooses buyer or seller. With it, one output of value minus 1000 can satisfy both scripts. If the arbiter never signs, those coins stay in the script. That is a freeze of the coins that entered it. It is not a mint, and it is not a freeze of coins that never entered the script.
- Filtering is not a halt. A pool can leave a transaction out of the blocks that pool builds. Another miner can still include it. This page does not print a pool share.
- The Groth16 script-unit figure is 14_000_000. The `Gram` literal is not that figure.

What was withdrawn:

- Bot's storage-mass figures 200_400 and 2_061_236, and the fee sentence of about 20_040 sompi.
- Calling the `Gram` literal compute mass.
- Reading the TN10 refusal as a version-1 protocol rejection.
- A flat fee floor of 100_000 sompi, or of 0.001 KAS.
- "Any node, later" as a description of a default node past its pruning point.

Limits of this commit:

- `seal_escrow.sil` was not given a timeout or a refund. The upstream tag ships `transfer_with_timeout.sil`. This repository did not use it. A new branch of the script without an execution test of `release` would be a second unverified claim.
- No execution test of `release` was added. The compile is not an execution.
- No pool share was printed. The short samples stay in Bot's Round 2.
- DAA score 474165565 is linked to the docs page both sides named. It is not stated as a literal of the v2.1.0 Rust tree. This commit did not re-fetch that page.
- The lane article was dropped as a source for the 50-lane and gas figures. The pin is `config/constants.rs`. This commit did not re-fetch the article.
- Starmind and Terafab openings are labeled as public reporting not linked on the page. The vehicle page no longer says Tesla already lets owners verify an image.
- No spend, no broadcast, no seed.

## Recommendations

Done in this commit:

- [RAILS.md](../../RAILS.md) states the pruning window, the missing refund on the example, one filtering sentence with no percentage, and a source line for the Toccata DAA score.
- [branches/13-supply-chain.md](../../branches/13-supply-chain.md) no longer says any node can show the digest after the window.
- The fee rate replaces the flat 100_000 sompi wording in [code/BYTES.md](../../code/BYTES.md), [branches/05-starbase.md](../../branches/05-starbase.md), [branches/12-charging.md](../../branches/12-charging.md), [code/covenant/README.md](../../code/covenant/README.md), and the comment in [seal_escrow.sil](../../code/covenant/seal_escrow.sil). [branches/01-launch.md](../../branches/01-launch.md) says the 1000 sompi figure is the fee the script names, and that the relay rule is a mass rate.
- The label "a single-input example that trusts the arbiter" is on the covenant note and on the branch pages that offer the script as a payment: launch, Starlink, Starbase, charging, and supply chain.
- The crate headers say 81 bytes. The precompile table names the `Gram` literal and the script units returned. The moved lane URL is not used for the lane caps. The wallet mass symbol is named where the byte notes had said it was absent.
- Starmind and Terafab no longer prescribe a user lane with no prover. Starlink keeps the native lane and drops the manifest-lane prescription. The vehicle image sentence is limited to a hash the owner is actually given.
- [DISCLAIMER.md](../../DISCLAIMER.md) says the compile is not an execution of `release`.
- The TN10 note in [challenge-bot-build.md](../challenge-bot-build.md) keeps the address and the encoder hex, says the hex was not broadcast, and records that there is no txid and no origin.

Left undone on purpose: a timeout entry, an execution test, a pool percentage, and a claim that 474165565 is a constant of the v2.1.0 Rust tree.
