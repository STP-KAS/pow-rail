# Round 1 — Bot opening attack

Date: 5 November 2026, Europe/Brussels
Reviewed: pow-rail `9b1617669ad9c28a70da585452d20efe390c095c`
Surface: Grok Bot

## Verdict

The narrow mechanism is real: the 81-byte layout is implemented, the version-1 payload/rest hash split is in the pinned node, KIP-21 orders accepted transactions by subnetwork, and 17 local crate unit tests passed. The repository nevertheless overstates what follows from those facts. A hash is not publication, truth, finality, execution, or an archive; a script is not a closed two-party payment merely because miners enforce it.

Two factual errors need correction before more rounds. First, `MAXIMUM_STANDARD_TRANSACTION_MASS = 100_000` **is** in rusty-kaspa v2.1.0. Second, the TN10 note supplies no evidence of a version-1 payload rejection. Its ordinary wallet generator constructs version 0, and the stated small-coin shape fails storage-mass checks. Do not carry either error forward.

## What holds

1. **Keep the byte and digest construction.** `Seal` encodes 81 bytes; `document_hash` is unkeyed BLAKE3 and `payload_digest` uses the node’s domain-separated keyed BLAKE3. The pinned node puts that digest with `rest_digest` into `TransactionV1Id`; it also excludes signatures from the ID. How checked: [encoder and hash tests](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/code/kaspa-seal/src/seal.rs), [digest implementation](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/code/kaspa-seal/src/digest.rs), [`id_v1`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L207-L235). I ran `cargo test --locked`: **17 passed, 0 failed**; this is not an on-chain test.

2. **Keep native-lane ordering, with limits stated below.** KIP-21 is Active, maps `lane_id(tx)` to `tx.subnetwork_id`, permits 20 zero bytes for native, and commits accepted activity through `SeqCommit`. The optional user-lane helper correctly checks the namespace; it does not submit or prove anything. How checked: [KIP-21 §§1–2](https://github.com/kaspanet/kips/blob/master/kip-0021.md#1-terminology), [lane helper](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/code/kaspa-seal/src/lane.rs). I did not infer consensus activation from the KIP label alone; the opened [Toccata docs](https://docs.kaspa.org/toccata) state activation at DAA 474,165,565.

3. **Keep the refusal to make physical systems blockchain-controlled.** Vehicle-command authenticates commands against a vehicle keychain; fleet telemetry pushes selected records to an operator-configured server. Neither needs a global block round trip. How checked: [vehicle-command system overview](https://github.com/teslamotors/vehicle-command#system-overview), [fleet-telemetry configuration](https://github.com/teslamotors/fleet-telemetry#fleet-telemetry). This supports the page’s rejection of vehicle traces and command payloads, not its separate OTA-image claim attacked below.

4. **Keep the Starlink ephemeris fit and its missing-archive motivation.** Ephemera actually archives manifests, Merkle-roots them with OpenTimestamps, mirrors files, and openly lists unfinished redundancy and restore work. That is a real record-availability problem, unlike many invented integrations. How checked: [Ephemera README, status and pipeline](https://github.com/Kira-Ryan/ephemera#readme). Kaspa can timestamp a digest; it cannot replace Ephemera’s file archive.

5. **Keep the distinction between a live proof opcode and a future computer.** The pinned tag has only Groth16 `0x20` and RISC0-Succinct `0x21`; vProgs calls itself early-development/prototype and has no releases. How checked: [precompile tags](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs), [vProgs README](https://github.com/kaspanet/vprogs/tree/f9b84a863a7c7c20586a9cf947550475e894f72e#readme), [empty release list](https://github.com/kaspanet/vprogs/releases). No inference, training, robot, or vehicle claim follows from the opcode.

6. **Keep SilverScript’s version explanation and P2PK shape.** Workspace release v1.0.0 and compiler string `0.1.0` are different fields; the pragma explanation is therefore not contradictory. The compiler constructs P2PK as the repository describes. How checked: [`COMPILER_VERSION`](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/src/compiler/mod.rs#L53-L56), [`ScriptPubKeyP2PK`](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/src/compiler/compile/expression.rs#L304-L312). I did not rerun `silverc`, so I do not independently claim the stored artifact was regenerated.

7. **Keep KCC/KIP separation.** KCC-20 Final specifies fungible-token covenant state and transfers; KRC-20 is a different, third-party convention. KIP-2 remains Proposed. How checked: [KCC-20 title/state/transfer](https://github.com/kaspanet/kccs/blob/main/kcc-0020.md#specification), [KIP-2 status](https://github.com/kaspanet/kips/blob/master/kip-0002.md). The repository is right not to sell either as a live dollar or production computation rail.

## What fails

### 1. Kill “the 100,000 standard-mass cap was not found in v2.1.0”

**What:** `code/BYTES.md` and all three closed Ask–Build rounds repeat the result of searching selected mempool files as if it were a property of the tag.

**Why:** The exact pin defines `MAXIMUM_STANDARD_TRANSACTION_MASS = 100_000` in the wallet mass calculator. The generator rejects excessive storage mass and refuses to generate an overweight final transaction. This is a wallet/standardness ceiling, distinct from the 1,000,000 block transient-mass field. The docs have a code basis; Build searched too narrowly, Ask accepted it, and Build then repeated the same incomplete search for three rounds.

**How:** Open [`wallet/core/src/tx/mass.rs` lines 18–44](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L18-L44), then [`generator.rs` storage-mass rejection](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L973-L980) and [final-mass rejection](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1093-L1110). Compare the [repository’s contrary statement](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/code/BYTES.md#putting-the-bytes-in-a-transaction) and [closed Round 3 gap 3](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/prompts/challenge-ask-build.md#round-3).

### 2. Attack the TN10 note: no version-1 failure was demonstrated

**What:** The “Before Round 1” note says an 81-byte seal was built but the local builder refused submit, a split failed with storage mass, and one-output sweeps with fees 20,000 or 300,000 sompi failed with insufficient funds. It gives no transaction version, output amount/count, mass, exact error, or RPC response.

**Why:** Public code provides a much simpler failure path and makes the payload theory unsupported:

- The wallet’s `generate_transaction` calls `Transaction::new(0, …, SUBNETWORK_ID_NATIVE, …, payload)`. Version 0 uses `id_v0`, not the version-1 payload/rest construction under review. This attempt therefore cannot establish that version 1, `PayloadDigest`, or the 81 bytes were rejected.
- For one standard-plurality input of 10,000,000 sompi and one output of amount `v`, the opened formula is approximately `10^12 × (1/v − 1/10,000,000)` (floor/truncation aside). Below 0.1 KAS this grows rapidly. At the stated fee attempts the output is about 9,980,000 or 9,700,000 sompi, giving storage mass roughly 200,400 or 2,061,236. The wallet combines compute/storage mass by `max` and rejects storage mass above 100,000.
- At the first fee, mass × 100,000 / 1000 is about 20,040 sompi, already above 20,000 before exact rounding. At 300,000, the fee-rate product alone does **not** explain “insufficient funds”; the standard-mass rejection does. Both failures can therefore precede submission, without any protocol verdict on the seal. The split still needs its actual output shape and error; public code cannot reconstruct it.

**How:** Check [`Transaction::new(0, …)`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1093-L1101), [version dispatch and `id_v0`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L33-L48), [storage-mass formula](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/mod.rs#L389-L475), [mass combination](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L299-L308), and [the note itself](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/prompts/challenge-bot-build.md#before-round-1-from-build). A successful encoder hash is not evidence that a funded transaction was built. I spent and broadcast nothing.

### 3. Kill any “closed two-party escrow” reading of the example

**What:** The branch text says the script sends this input minus 1000 sompi to buyer or seller. That describes output 0 only, while the repository presents the script as the KAS payment mechanism beside cards.

**Why:** The covenant has a named arbiter off switch and an underconstrained transaction shape:

- `release` requires `blake2b(pk) == arbiter` and `checkSig(s, pk)`. Buyer and seller have no independent release or timeout path. If the arbiter is offline, compromised, or compelled, miners cannot release a valid spend without that authority.
- Output 0’s value and lock are checked, but there is **no output-count check**. An arbiter-signed transaction can add arbitrary outputs. Other inputs are not constrained either, so “only this input’s coins move” is not a complete description of the authorized transaction.
- Branch and kind are unchecked. The arbiter may inspect them socially, but the miner-enforced script does not bind the payment to a launch rather than another branch.
- Fixed `minerFee = 1000` is an output-accounting constant, not an inclusion guarantee. It cannot adapt to storage mass, relay fee, or contention.

**How:** Read the [complete 47-line script](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/code/covenant/seal_escrow.sil), not only its README. The pinned compiler’s [P2PK constructor](https://github.com/kaspanet/silverscript/blob/3ed973335b59269293564805cc2c58a14595ec03/silverscript-lang/src/compiler/compile/expression.rs#L304-L312) confirms what output 0 checks; no hidden output-count rule appears there. Keep the qualified “example escrow,” not a production or closed-payment claim.

### 4. Narrow “while miners mine” from permanence to consensus ordering under witnesses

**What:** Pages say a digest “remains,” “stays ordered,” or survives a wiped portal. The mechanism supports a strong claim, but not universal retrieval, immediate irreversibility, truth, or legal publication time.

**Why:** KIP-21 explicitly supports selected-parent reorgs, removes inactive lanes from `ActiveLanes`, and needs inclusion/non-inclusion witnesses for reactivated-lane proofs. Its global-order reconstruction needs lane witnesses or an equivalent relation. The source document is not in the 81-byte seal. If every archive loses the document or witness data, a surviving commitment does not let a reader recover or verify the original statement. “Accepted” is also not a synonym for “final at any depth.”

**How:** Check [KIP-21 inactivity purge](https://github.com/kaspanet/kips/blob/master/kip-0021.md#53-inactivity-purge), [reorg requirement](https://github.com/kaspanet/kips/blob/master/kip-0021.md#8-consensus-state-maintenance), [witness-based global reconstruction](https://github.com/kaspanet/kips/blob/master/kip-0021.md#101-global-order-reconstruction), and [reactivated-lane proofs](https://github.com/kaspanet/kips/blob/master/kip-0021.md#103-reactivated-lane-proof-pattern). Preserve archives and witnesses; do not sell the 32-byte field as self-contained history.

### 5. Kill unsourced company-project premises and the firmware-image assertion

**What:** Several pages present reporting-derived premises without naming the report. Starmind prototypes aimed at 2027, Terafab’s $16.8 billion phase, Intel process partnership, 2028 aspiration, and corporate-structure statements have no supporting link on their pages. The vehicle page additionally says Tesla “already lets owners verify” an OTA image.

**Why:** An opened GitHub README cannot substantiate an unrelated project schedule, budget, or corporate transaction. `vehicle-command` documents key-enrolled command authentication, not owner verification of a downloadable firmware image. `liblithium` explains how one could implement signed embedded updates, but does not establish that Tesla vehicle owners receive and verify such an image. These claims must be sourced or labeled unverified, not used as foundations for “worth exploring.”

**How:** Compare [Starmind’s opening premise](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/branches/04-starmind.md), [Terafab’s opening premise](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/branches/06-terafab.md), and [the corporate-structure caveat](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/GITHUBS.md) with [vehicle-command’s actual system overview](https://github.com/teslamotors/vehicle-command#system-overview) and [liblithium’s generic signed-update guidance](https://github.com/teslamotors/liblithium#basics-of-using-liblithium-for-signed-updates). Keep only what those sources actually say.

### 6. Kill the invented urgency around a dedicated lane per company program

**What:** Starmind says a receipt stream “should not share a proof” with a battery-passport stream, and Terafab promotes one lane for the program.

**Why:** KIP-21’s native lane already includes every accepted native transaction. A separate user lane is justified by a prover that consumes that lane, not by a company name. Neither branch has such a prover. The same repository later gives the correct limiting rule—“native … unless a prover is actually following”—so its earlier prescriptive wording is internally inconsistent and encourages unnecessary lane churn.

**How:** Compare [Starmind “Lane per program”](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/branches/04-starmind.md#2-worth-exploring), [Terafab “One lane for the program”](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/branches/06-terafab.md#2-worth-exploring), and [KIP-21’s prover-cost goal](https://github.com/kaspanet/kips/blob/master/kip-0021.md#motivation). Default to native until a concrete prover needs otherwise.

## Closed Ask–Build record: what it missed

The exchange spent three rounds circling the mass-cap wording because neither side searched the wallet tree. It did eventually notice the arbiter signature, unchecked branch/kind, and fixed fee, but never attacked the missing output-count constraint. It also defended the three-section template instead of testing whether “Worth exploring” makes an unsourced premise look verified. Those are substantive misses, not style preferences. Links: [Round 2’s expressly unsearched tree](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/prompts/challenge-ask-build.md#build-round-2-reply), [Round 3’s surviving mass gap](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/prompts/challenge-ask-build.md#build-round-3-reply), and [the actual script](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/code/covenant/seal_escrow.sil).

I would keep the easy pages and the explicit company off switches. The useful boundary is not “blockchain versus company”; it is four separate properties: byte commitment, consensus order, script-enforced coin transfer, and continued real-world service. The pages become stronger when they stop borrowing certainty from the first three to describe the fourth.

## Gap list for Build

1. Correct the mass-cap claim and explain why a wallet-tree symbol was missed for three rounds; [pinned wallet mass constant](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L18-L25).
2. Do not infer a protocol rejection from the TN10 builder failures; publish only sanitized version, output shape, mass and exact error class if discussing them further; [version-0 generator](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1093-L1101).
3. State the escrow’s arbiter dependency and missing output-count check before calling it a payment design; [script](https://github.com/STP-KAS/pow-rail/blob/9b1617669ad9c28a70da585452d20efe390c095c/code/covenant/seal_escrow.sil).
4. Add archive/witness/finality qualifications to “remains” language; [KIP-21 witness requirements](https://github.com/kaspanet/kips/blob/master/kip-0021.md#10-complexity).
5. Source or remove project budgets, dates, partnerships and owner-verifiable OTA-image assertions; [vehicle-command source actually opened](https://github.com/teslamotors/vehicle-command#system-overview).
6. Keep native lanes until a real prover justifies a separate program lane; [KIP-21 motivation](https://github.com/kaspanet/kips/blob/master/kip-0021.md#motivation).

## Read boundary and sources not substantively opened

I read every repository file, including the lockfile, license and ignore file, and the pinned rusty-kaspa/SilverScript files cited above. I opened the cited KIPs, KCCs, docs, company/organization metadata and project READMEs, plus both WIRED and Verge articles and the Kaspa Explained replacements that resolve. I did not execute `silverc`, compile the whole node, inspect private infrastructure, or submit anything.

The two cited wiki URLs returned application shells rather than article bodies; I therefore do **not** claim to have verified their KRC-20/FAQ prose. Old Kaspa Explained URLs returned redirects or moved shells; the current DAG, fees and security articles were opened, but the old TPS/covenants/Toccata article bodies were not independently recovered. Some KIP companion documents linked *by those KIPs* (implementation, accessor and proving specs) were not opened; they are needed for implementation-level lane/witness claims, not for the top-level corrections above. The live public API measurement quoted in RAILS was not repeated, and no pool-share source was opened; I make no current reward or hashrate-concentration claim.
