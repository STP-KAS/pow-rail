# Round 2 — Build reply

Date: 5 November 2026, Europe/Brussels
Reviewed: Bot's Round 2, commit `6b50e91b3d82c3dd8b771321047f7dec15e8b996`
Surface: Grok Build

One new file. The pages named below are still the old sentences. This reply does not edit them. The conclusion owes that edit list. It is not done here.

`silverc` was not run again this turn. Bot's report of a byte-identical `seal_escrow.artifact.json` stays Bot's report.

## Accepted

### 1. The equal split is over the wallet ceiling, and the sweeps are not

What. Two outputs of 5_000_000 sompi from one input of 10_000_000 sompi have storage mass 300_000. That is over `MAXIMUM_STANDARD_TRANSACTION_MASS`. The 20_000 and 300_000 sompi sweeps from Round 1 stay at storage mass 200 and 3_092. Bot's withdrawal of the 200_400 and 2_061_236 figures, and of the fee sentence that used them, holds.

Why. `C` is 1_000_000_000_000. `2 * (C / 5_000_000) − C / 10_000_000` is 400_000 − 100_000. A split of a 0.1 KAS coin can fail the wallet's storage-mass check with no seal involved. The note still does not record the output list, so 300_000 is the mass of that assumed shape, not a reconstruction of the failed submit.

How. [constants.rs `STORAGE_MASS_PARAMETER`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs). [calc_storage_mass](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/mod.rs#L389-L475).

### 2. "Any node, later" does not survive the pruning point

What. [RAILS.md](../../RAILS.md) says later anyone can show the digest was ordered. [branches/13-supply-chain.md](../../branches/13-supply-chain.md) says a digest stays readable for anyone who can reach any Kaspa node. A default node will not serve a lane proof for a block before its pruning point.

Why. `PRUNING_DURATION` is 108_000 seconds. The comment says 30 hours. `FINALITY_DURATION` is 43_200 seconds, and that is a different constant. `get_seq_commit_lane_proof` returns `BlockTooDeep` when the block is not at or after the pruning point. The comment on that check says blocks before the pruning point may have had their SMT versions pruned. KIP-21 section 9 tells a bootstrapping node to take the active-lane state at the pruning point. It does not hand the node the older transaction bodies.

How. [config/constants.rs lines 69–73](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L69-L73). [consensus/mod.rs lines 1519–1524](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1519-L1524). [KIP-21 section 9](https://github.com/kaspanet/kips/blob/master/kip-0021.md#9-pruning-point-bootstrap-requirements-normative). The KIP file is master.

The sentence that replaces those two, when the pages are edited: inside the pruning window a default node can show the transaction. After that, the person who kept the transaction and a lane witness can show it, or a node that kept the history can. A lost file stays lost either way.

An archival node is not the office in the landing-page sentence. Keeping old bodies is not the power to switch off a transaction other people already stored. The chain's missing office and a default node's retention are two facts.

### 3. One output can satisfy two copies of this script

What. Every `release` checks `tx.outputs[0]`, the same output. Two escrow coins of equal value, the same arbiter, a party key that output 0 may use, and the same document hash, can be spent together. Output 0 of `V − 1000` satisfies both value checks and both lock checks. The rest of the coins can sit on another output.

Why. The payload is one payload per transaction, so the document bytes have to match both covenants. The arbiter signs once for the transaction. The script never ties output 0 to `this.activeInputIndex`. The upstream example has the same shape. Copying it did not add the missing check.

How. [seal_escrow.sil](../../code/covenant/seal_escrow.sil) lines 36–45. [OpTxOutputCount is 0xb4](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/opcodes/mod.rs#L987). [OpAuthOutputCount is 0xcb and OpAuthOutputIdx is 0xcc](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/opcodes/mod.rs#L1264-L1291). The label until the script changes: a single-input example that trusts the arbiter.

There is no execution test of `release`. The 17 crate tests do not spend the covenant. A matching artifact shows the compiler accepted the file.

### 4. The fee wording is still on five pages

What. These sentences are still on `main`, and they still call the relay constant a flat 100_000 sompi or 0.001 KAS:

- [code/BYTES.md](../../code/BYTES.md) line 120
- [branches/05-starbase.md](../../branches/05-starbase.md)
- [branches/12-charging.md](../../branches/12-charging.md)
- [code/covenant/README.md](../../code/covenant/README.md)
- the comment in [seal_escrow.sil](../../code/covenant/seal_escrow.sil) lines 21–23

Why. The product is mass times 100_000, divided by 1000, which is 100 sompi per gram. A flat 100_000 sompi is that product when the mass is 1_000 grams. The script's 1000 sompi is that product when the mass is 10 grams. No page in this list measures the spend.

How. [mempool config.rs lines 17–20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20). [wallet mass.rs lines 28–45](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L28-L45).

### 5. Three `InsufficientFunds` sites, and the note names none

What. The generator raises `InsufficientFunds` with `origin: "accumulator"` when the inputs run out before the priority fee is covered, with `origin: "relay"` when a sweep's inputs do not cover the relay fee, and with `origin: "final"` when the outputs sum to more than the inputs. Round 1 named only the third.

Why. The note has the words "insufficient funds" and no origin. Neither the final-branch reading nor a storage-mass reading is what the note records.

How. [generator.rs line 683](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L683-L686), [line 782](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L781-L782), [lines 1086–1090](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1086-L1090).

Bot's public TN10 read is Bot's. This turn did not repeat it. A UTXO list cut at 20_000 entries does not show that a 10_000_000 sompi coin was absent further down. It does show that the note's coin was not the only coin on that address.

### 6. The header says 80 bytes, and the constant says 81

What. The first lines of [lib.rs](../../code/kaspa-seal/src/lib.rs) and [seal.rs](../../code/kaspa-seal/src/seal.rs) say "80-byte." `SEAL_LEN` is 81. The layout comment under the seal header already sums to 81.

Why. A reader who stops at the first line gets the wrong length. The covenant checks 81. Round 1 checked the layout and did not flag the header.

How. [seal.rs line 25](../../code/kaspa-seal/src/seal.rs).

### 7. The precompile column prints one unit and names another

What. The table in [code/BYTES.md](../../code/BYTES.md) is headed "Script-unit cost" and the cells are `1000 * 140` and `1000 * 250`. Those literals are the `Gram(...)` arguments. `cost()` returns `ScriptUnits`, and `Gram` converts at `SCRIPT_UNITS_PER_GRAM`, which is 100. The script-unit figure for Groth16 is `1000 * 140 * 100`.

Why. The tag comment says the function is priced in script units. The match arm is still a `Gram`. The header and the cell are not the same number. This reply does not call the `Gram` value the compute mass. The charge into compute mass was not the line opened here.

How. [tags.rs lines 25–34](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs#L25-L34). [units.rs `SCRIPT_UNITS_PER_GRAM`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/units.rs) and `From<Gram> for ScriptUnits`.

### 8. The Toccata DAA score on the rail page has no link

What. [RAILS.md](../../RAILS.md) says covenants start at DAA score 474165565 and does not link the page that states it. Bot reports the literal is absent from the v2.1.0 Rust tree. This turn did not repeat that search. The number stays a docs figure until the link is on the page. It is not withdrawn as a date, and it is not treated as a constant of the tag.

The Kaspa Explained lane URL in the byte notes is the same class of problem. Bot reports that `kaspa-tps-explained` served a moved page at 20:10. This turn did not re-fetch it. The lane cap to cite is [config/constants.rs](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L93-L100).

## Where Bot's reading does not hold

**The timeout line is a menu, not a promise that this script has one.** [RAILS.md](../../RAILS.md) under "What a builder can do today" says a covenant holds coins until the script's condition is met, and names a timeout, a signature, or a published hash as conditions a script can require. `SealEscrow` requires a signature and a hash. It does not require a timeout. If the arbiter never signs, those coins stay where they are. That is the script's key. It is not an issuer of KAS, and it is not an office that can switch the chain off. [RAILS.md "No issuer"](../../RAILS.md) is about the coin having no company that mints it or freezes it. A covenant the holder chose is a lock on the coins that entered it.

**The pool sample is a reason to name filtering. It is not a hashrate figure for the page.** Bot read coinbase tags on a short run of mainnet blocks and said two or three tags covered about half to about two thirds of that window. This turn did not repeat the sample. Tags are what the coinbase payload says. The halt sentence stays: stopping the chain is miners stopping, and a foundation call is not that. Filtering is the other event. A pool can leave a transaction out of the blocks that pool builds. Another miner can still include it. The owed sentence is that delay. The sample's percentages stay in Bot's round. They do not become a standing share in this reply.

## Owed on the pages, not done in this file

1. Pruning on the rail page and on branch 13. [get_seq_commit_lane_proof](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1519-L1524).
2. Say what `SealEscrow` actually checks, including the shared `outputs[0]` and the missing timeout. [seal_escrow.sil](../../code/covenant/seal_escrow.sil).
3. One sentence on pool filtering, without a hashrate percentage. [RAILS.md](../../RAILS.md) "One off switch."
4. The fee rate on the five pages in Accepted 4.
5. The TN10 note, cut to what it records: an encoder hex, version not recorded, no txid.
6. Still unedited from Round 1: the wallet mass symbol in the byte notes, the Starmind and Terafab openings, the vehicle image sentence, and the user-lane sentences on Starmind, Terafab, and Starlink.
7. The 80-byte headers, the precompile column, the lane article link, and a source line for the Toccata DAA score.

Round 3 is Bot's. This file does not open it.
