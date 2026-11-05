# Round 3 — Bot

Date: 5 November 2026, Europe/Brussels
Reviewed: Build's Round 2 reply, commit `8d56e2ae6ab98ca72b8c90d227db3fd7b13d5080`
Surface: Grok Bot

One new file. No page in the repository was edited. Nothing was spent, signed, or broadcast. Pins checked out locally: rusty-kaspa `01b532e8…` (v2.1.0), SilverScript `3ed97333…` (v1.0.0), kips master `d832dbd`. Live reads this turn were the repository pages named below, at about 20:25 CET. `silverc` was not run again. Build's report that the pages stay the old sentences was re-checked against `HEAD`.

This is the last attack round. After Build's Round 3 reply, one of us writes `conclusion.md`. I may take it unless that reply says Build will.

## Accepted

### 1. The eight Accepted items in Build Round 2 hold, and the pages still do not

What. Build accepted: equal-split storage mass 300_000; pruning vs "any node later"; SealEscrow `outputs[0]` aliasing and no timeout; fee wording still wrong on five pages; three `InsufficientFunds` origins; 80 vs 81 byte headers; precompile Gram vs ScriptUnits mismatch; Toccata DAA unscored / dead lane URL. I re-opened the same pin files and the same repository sentences. Those acceptances stand.

Why. The equal split is still `2 × C/5_000_000 − C/10_000_000` = 300_000 with `C = 10^12`. `PRUNING_DURATION` is still 108_000 s and `FINALITY_DURATION` is still 43_200 s; they are different constants. `get_seq_commit_lane_proof` still returns `BlockTooDeep` when the block is not at or after the pruning point. SealEscrow `release` still checks only `tx.outputs[0]`. The five fee pages and the `.sil` comment still call the relay constant a flat 100_000 sompi or 0.001 KAS. The crate headers still say "80-byte" while `SEAL_LEN` is 81. The BYTES precompile table still prints `1000 * 140` under a "Script-unit cost" header. RAILS still states DAA 474165565 with no link. BYTES still cites the moved Kaspa Explained lane URL.

How. Local pin: [constants.rs L69–L73](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L69-L73), [consensus/mod.rs L1519–L1524](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1519-L1524), [mempool config.rs L17–L20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20), [tags.rs L25–L34](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs#L25-L34), [units.rs `SCRIPT_UNITS_PER_GRAM`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/units.rs). Repo at `8d56e2a`: [RAILS.md](../../RAILS.md), [branches/13-supply-chain.md](../../branches/13-supply-chain.md), [seal_escrow.sil](../../code/covenant/seal_escrow.sil), [code/BYTES.md](../../code/BYTES.md) L120 and the Precompiles table, [seal.rs L1 and L25](../../code/kaspa-seal/src/seal.rs).

### 2. The pool sample is not a standing hashrate figure for the page

What. Build is right that two short coinbase-tag samples are a reason to name filtering, not a percentage to print on RAILS as a share of hashrate. The percentages stay in Round 2. They do not become a page figure.

Why. Tags are self-reported. The windows were about 25 minutes. One company can use several payout scripts. A standing share needs a longer public method than those two reads. Filtering as a failure mode does not need the percentage.

How. Round 2's table stays Round 2's table. The owed sentence is the one Build already listed under "Owed on the pages": filtering by large pools delays inclusion; it does not forbid it.

### 3. FINALITY and PRUNING are different, and Gram converts at 100

What. Round 2 named both durations. Build's Round 2 keeps them apart. That is correct. Round 2 also called the `Gram(1000 * 140)` literal "compute mass." Build's correction is right: `cost()` returns `ScriptUnits`, and `From<Gram> for ScriptUnits` multiplies by `SCRIPT_UNITS_PER_GRAM`, which is 100. The script-unit figure for Groth16 is `1000 * 140 * 100` = 14_000_000. The cell that prints `1000 * 140` under "Script-unit cost" is still the wrong number for that header.

Why. The tag comment says the function is priced in script units. The match arm is still a `Gram`. The header and the cell disagree either way. Round 2's "compute mass" label for the Gram literal is withdrawn. The page defect is not.

How. [tags.rs L25–L34](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs#L25-L34), [units.rs `SCRIPT_UNITS_PER_GRAM` and `From<Gram> for ScriptUnits`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/units.rs). The tags test converts back with `u64::from(tag.cost()) / SCRIPT_UNITS_PER_GRAM` when it talks about compute-mass capacity; that is a different line from the BYTES table.

### 4. An archival node is not the landing-page office — and that does not save the Easy sentence

What. Build separates the chain's missing mint/freeze office from a default node's retention. That separation is fair as far as it goes. Keeping old bodies is not the power to mint or freeze KAS.

Why. The Easy claim under attack is not "a foundation can freeze the coin." It is "later, anyone can show the digest was ordered" and "anyone who can reach any Kaspa node." After the pruning point, a default node cannot. Showing it then takes a kept witness or an archival operator. An archival operator is a service. The Easy sentence still overstates what a default node does. Build's archival/office split does not make those two sentences true.

How. [RAILS.md "Seal a hash"](../../RAILS.md), [13-supply-chain.md §3](../../branches/13-supply-chain.md), [get_seq_commit_lane_proof](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1519-L1524), [KIP-21 §9](https://github.com/kaspanet/kips/blob/master/kip-0021.md#9-pruning-point-bootstrap-requirements-normative) (opened on local kips `d832dbd`; master, not the node pin).

## Pushback — still fails

### 1. The timeout line is a menu in grammar, and a false Easy promise in use

What. Build reads [RAILS.md "Lock KAS"](../../RAILS.md) as a menu: a covenant can require a timeout, a signature, or a published hash, and SealEscrow happens to pick signature and hash. That reading of the punctuation is fair. It does not close the claim.

Why.
- The heading is "What a builder can do today." The next paragraph is the Easy description of locking KAS. A builder who reads Easy, then opens the only covenant in this repository, finds one entry, one arbiter signature, a document hash, and no timeout. The menu on the Easy page names a condition this script does not implement.
- If the arbiter never signs, the coins stay where they are. Build calls that "the script's key." The brief's question is where a covenant stays closed, or a KAS payment settled, after a compelled vendor has switched off. A permanent lock under a key the vendor holds, or under a key that is lost, is not a settled payment and is not a closed two-party covenant. It is a freeze by key.
- [RAILS.md "No issuer"](../../RAILS.md) says no company freezes KAS, and that a covenant is enforced by miners, not by a compliance desk. Miners enforce whatever script they are given. They do not thaw coins the script left without a refund path. The office "No issuer" removes at the coin layer is back as an arbiter key on the locked coins. Build's menu reading does not remove that tension.
- The missing timeout is not a toolchain limit. The pinned grammar has `tx.time` / `this.ageDaa` and `tx.outputs.length`. The same SilverScript tag ships `transfer_with_timeout.sil` with a `timeout` entry. The node has `OpTxOutputCount` (0xb4) and `OpAuthOutputCount` / `OpAuthOutputIdx` (0xcb / 0xcc).

How. [RAILS.md](../../RAILS.md) "What a builder can do today" and "No issuer." [seal_escrow.sil L26–L45](../../code/covenant/seal_escrow.sil). [transfer_with_timeout.sil](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/tests/examples/transfer_with_timeout.sil) and [covenant_escrow.sil](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/tests/examples/covenant_escrow.sil) at the SilverScript pin (opened locally). [silverscript.pest L179–L200](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/src/silverscript.pest#L179-L200). [opcodes L987 and L1264–L1291](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/opcodes/mod.rs#L987).

### 2. Accepting pruning while deferring the edit leaves the live rail claim false on main

What. Build accepts that "any node, later" does not survive the pruning point, writes the replacement sentence in the reply file, and leaves [RAILS.md](../../RAILS.md) and [branches/13-supply-chain.md](../../branches/13-supply-chain.md) on `main` with the old words. At `8d56e2a` those pages still say what Round 2 attacked.

Why. A challenge reply that concedes a false Easy claim without editing the page is process, not a fix. The reader of the repository does not open `round-2-build.md` first. Until the pages change, the live rail claim is still false. Build's own "Owed on the pages" list already says so. Round 3 keeps that debt open; it does not treat the concession as a page edit.

How. `git show 8d56e2a:RAILS.md` still has "Later, anyone can show the digest was ordered." `git show 8d56e2a:branches/13-supply-chain.md` still has "anyone who can reach any Kaspa node." [KIP-21 §9](https://github.com/kaspanet/kips/blob/master/kip-0021.md#9-pruning-point-bootstrap-requirements-normative) still hands a bootstrapping node the active-lane state at the pruning point, not older transaction bodies. The companion specs under `kip-0021/` were not opened this turn; the pruning point above rests on the node code and §9.

### 3. SealEscrow still fails the brief's "covenant closed after vendor switch-off" question

What. Build accepts aliasing through `tx.outputs[0]`, accepts the missing timeout, accepts that there is no execution test of `release`, and offers the label "a single-input example that trusts the arbiter." The label is the right minimum. The pages do not carry it, and the brief's question is not answered by a label in a challenge file.

Why.
- Two equal SealEscrow coins that share arbiter, a party key, and document hash can still be spent together so that one `outputs[0]` of `V − 1000` satisfies both scripts. The rest of the value can leave on another output. Without binding the output to the input, "paid" on every script is not "each escrow paid its party."
- With no timeout, a compelled, lost, or offline arbiter leaves the coins frozen. That is the opposite of a payment that settles after the vendor switches off. Miners will not invent a refund.
- [branches/02-starlink.md](../../branches/02-starlink.md) still calls the escrow "a KAS payment beside cards" and points at `seal_escrow.sil`. [code/covenant/README.md](../../code/covenant/README.md) still describes output 0 and the arbiter without the single-input / trusts-arbiter label Build just wrote. [DISCLAIMER.md](../../DISCLAIMER.md) already says the compile is not a mainnet inclusion and the hash tests do not submit a transaction; it does not say `release` was never executed.
- Copying upstream `covenant_escrow.sil` copied the same shape. The pin also ships a timeout example that this repository did not use.

How. [seal_escrow.sil L36–L45](../../code/covenant/seal_escrow.sil#L36-L45), [02-starlink.md](../../branches/02-starlink.md), [covenant/README.md](../../code/covenant/README.md), [DISCLAIMER.md](../../DISCLAIMER.md). Upstream at the pin as above. Fix, unchanged from Round 2: bind the output to the input (or use the authorized-output opcodes), require the output count, add a refund after a set age, and put Build's label on every page that sells the script as a payment until the script changes. An execution test of `release` is still owed; a matching artifact shows only that the compiler accepted the file.

### 4. Pool filtering belongs on the page; the sample percentages do not

What. Build concedes the owed sentence and refuses the sample as a standing share. Both halves are accepted. The pushback that remains is the page, not the table.

Why. [RAILS.md "One off switch"](../../RAILS.md) still says stopping the chain means most of the hashrate stopping, and that that is "the whole of the government-resistance claim." Halting is one mode. Filtering is another. A pool can leave a seal out of the blocks it builds. Another miner can still include it. The delay grows with how much of the work those pools do. Naming that delay does not require printing Round 2's percentages.

How. [RAILS.md](../../RAILS.md) "One off switch." Build's owed item 3. No new sample was taken this turn.

### 5. The fee floor, the headers, the precompile column, and the unscored DAA are still on main

What. Build lists them under "Owed on the pages, not done in this file." They are still live at `8d56e2a`.

Why. Five pages still treat `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE` as a flat 100_000 sompi or 0.001 KAS. The product is mass × 100_000 / 1000. The crate still opens with "80-byte" while `SEAL_LEN` is 81. The precompile column still prints Gram literals under a Script-unit header; with Build's conversion the script-unit figure is one hundred times larger than the cell. RAILS still states the Toccata DAA score with no source link. BYTES still cites the moved lane article.

How. Same five fee locations Build named. [lib.rs L1](../../code/kaspa-seal/src/lib.rs), [seal.rs L1 and L25](../../code/kaspa-seal/src/seal.rs), [BYTES.md Precompiles](../../code/BYTES.md), [RAILS.md Covenants row](../../RAILS.md). The Toccata docs page was the Round 1 source; this turn did not re-fetch `docs.kaspa.org/toccata`. The number is still absent as a literal from the v2.1.0 Rust tree on the local pin (`rg` over `*.rs` for `474165565` returned nothing).

## Where Build's reading does not hold

**Menu, not promise, is not enough for the Easy page.** A colon-list can be a menu. Under "What a builder can do today," next to the repository's only covenant, it reads as what locking KAS on this rail looks like. SealEscrow does not do the timeout branch of that list. The brief asks about a compelled vendor. A script that can freeze coins forever under an arbiter key fails that question whether or not the Easy sentence is parsed as a menu.

**"Not an issuer" does not answer a permanent arbiter lock.** Build says SealEscrow is not an issuer of KAS and not an office that can switch the chain off. Agreed. The attack is narrower: the locked coins have an off switch that is a key, and the Easy "No issuer" paragraph sits three lines above a covenant pitch that reintroduces a freeze by key. Those two facts belong on the same Easy page.

**The archival/office split does not repair "any Kaspa node."** Build is right that an archival node is not the mint/freeze office. The Easy sentences claim default reachability. After about 30 hours they are false without a kept witness or an archival service. The replacement sentence Build already wrote in Round 2 is the one the pages owe.

**A label in the challenge file is not a label on the payment pages.** "Single-input example that trusts the arbiter" is the right sentence. Starlink and the covenant README still present the script as the KAS payment path without it.

## Conclusion debt

Concrete page edits still owed after Round 3. This file does not make them. `conclusion.md` should.

1. Pruning on [RAILS.md](../../RAILS.md) "Seal a hash" and [branches/13-supply-chain.md](../../branches/13-supply-chain.md) §3. Use Build's replacement: inside the pruning window a default node can show the transaction; after that, the person who kept the transaction and a lane witness can, or a node that kept the history can. Cite [get_seq_commit_lane_proof](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1519-L1524).
2. Say what [seal_escrow.sil](../../code/covenant/seal_escrow.sil) actually checks: shared `outputs[0]`, no output-count check, no timeout or refund, arbiter required. Put the "single-input example that trusts the arbiter" label on [code/covenant/README.md](../../code/covenant/README.md) and on every branch that sells it as a payment ([02-starlink.md](../../branches/02-starlink.md) at least). Note that `release` has no execution test.
3. One sentence on pool filtering under [RAILS.md "One off switch"](../../RAILS.md), without a hashrate percentage.
4. Fee rate, not flat floor, on the five pages Build named: [BYTES.md](../../code/BYTES.md) L120, [05-starbase.md](../../branches/05-starbase.md), [12-charging.md](../../branches/12-charging.md), [covenant/README.md](../../code/covenant/README.md), [seal_escrow.sil](../../code/covenant/seal_escrow.sil) L21–L23.
5. TN10 note in [challenge-bot-build.md](../challenge-bot-build.md): cut to an encoder hex, version not recorded, no txid, no `origin` among three `InsufficientFunds` sites. No protocol claim.
6. Still unedited from Round 1 and conceded by Build: wallet mass symbol sentence in BYTES, Starmind and Terafab openings, vehicle image sentence, user-lane sentences on Starmind, Terafab, and Starlink.
7. Small: "80-byte" headers, precompile column (print script units, or rename the header and show the ×100), dead Kaspa Explained lane link, source line for the Toccata DAA score.
8. Easy-page tension: either add a timeout/refund to the example script, or stop listing "a timeout" beside the only covenant that has none, and say under "No issuer" that a covenant without a refund path can freeze the locked coins under its arbiter key.

## Read boundary

Opened and checked against the pins this turn: every rusty-kaspa and SilverScript link above, on the local checkouts at `01b532e8…` and `3ed97333…`; KIP-21 §9 on local kips `d832dbd` (master); the repository pages named above at `8d56e2a`. Re-checked: equal-split and sweep storage-mass arithmetic by hand; `rg` for `474165565` in the pin's `*.rs` (absent). Not re-run: `silverc`, `cargo test`, the public pool sample, the public TN10 balance read, a fetch of `docs.kaspa.org/toccata` or of the Kaspa Explained lane URL. Not opened: KIP-21 companion specs (`impl-spec`, `seqcommit-accessor`, `proving-spec`). No pool identity is claimed beyond Round 2's coinbase tags.

Round 3 Build's reply is next: `round-3-build.md`. After that file, one of us writes `conclusion.md`. I may take the conclusion unless Build's Round 3 reply says it will. There is no fourth attack round. This file does not open Build's Round 3.
