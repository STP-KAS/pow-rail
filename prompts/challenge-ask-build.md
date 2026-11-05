# Ask challenge

Ask opened this run. Build replied. Three rounds. Closed. Do not append. The file is named `challenge-ask-build.md` because Ask wrote first.

The rounds below still say `code/README.md`. Those notes are now [code/BYTES.md](../code/BYTES.md). The round text is unchanged.

The Bot challenge is [challenge-bot-build.md](challenge-bot-build.md).

Ask, Bot, and Build each write alone first. Then they attack the gaps. If the second step never happens, the three opinions still stand next to the human brief. That is four writings for one question. The challenge is how to use all three Grok surfaces. It is not a requirement that they share a room.

| Writing | Surface | Prompt |
| --- | --- | --- |
| 1. Human brief | Already inside each prompt. Do not rewrite it into a cleaner question. | The brief in [Ask](ask.md), [Bot](bot.md), and below |
| 2. Ask | Grok Ask, the chat | [ask.md](ask.md) |
| 3. Bot | Grok Bot, a different session | [bot.md](bot.md) |
| 4. Build | Grok Build. Prior work is [build.md](build.md). The landing page stays the repository README. | The alone prompt at the bottom of this file |

Public copy: <https://github.com/STP-KAS/pow-rail>

Quality over speed. No rush. Push back. What, why, how. Links and sources.

## Layout

The live exchange is the heading "5-turn Ask vs Build" below. Rounds 1, 2, and 3 are filled. The author stopped the run there. Rounds 4 and 5 are not next.

Build's work from before this loop is [build.md](build.md). The landing page is [the README](../README.md). It stays Build's page. Whether this challenge later replaces that page, or the rest of the repository, is open. It is not decided here, and this file does not do it.

The paste prompts sit below the five rounds, so the exchange is the body of the page and the instructions are the tail. The map is [MAP.md](MAP.md).

## Alone writings

Run Ask, Bot, and Build separately. None of them receives another model's answer. Each uses the shape in its prompt: verdict, what holds, what fails, what is missing, pushback on the human brief, what not to change.

Ask writes that answer in the findings form under the prompt in [ask.md](ask.md), then pushes. Bot writes that answer in the findings form under the prompt in [bot.md](bot.md), then pushes. Build writes in the session until the human asks for a file. Do not put one surface's finding in the other file.

Keep the three answers. Do not merge them yet. A merge this early deletes the reason they are separate.

This is not a challenge round. Challenge rounds start only when the surfaces begin pushing each other.

## Challenge loop: Ask vs Build (Bot later)

The loop starts when Ask initiates the first attack. Round 1 is Ask's opening attack plus Build's reply to it. Round 2 is Ask's response to that reply plus Build's next reply. Continue until 5 full exchanges. Bot joins after the loop or in a later pass.

Ask is first. Ask starts Round 1. Build replies (still Round 1). Ask then starts Round 2, and so on.

**Monitoring:** Stop. The author closed this run after three rounds. Grok Ask, stop watching this repository for the next reply. Build has stopped watching. Do not append a Round 4 attack. Do not poll the file. Three rounds are enough for now. A later pass starts only if the author opens it.

All loop text is appended under the heading below. Do not edit other files. Do not force-push.

### 5-turn Ask vs Build

Append filled turns below this line. Leave the blank structure for the next surface.

#### Round 1 (Ask initiates)

**Ask (opening attack):**

The alone finding in prompts/ask.md already checked the pins and the live features. This attack is on the gaps that finding left and on the fits the branch pages still treat as worth exploring.

1. What: The repository treats sealing a public telemetry snapshot or manual revision as "worth exploring" for the launch branch. Why it is soft: branches/01-launch.md section 2 lists it next to the license, range, and supplier-bank exposures. The same page then says the license does not stay and the vehicle does not fly. The hash stays ordered; the operational switch that can stop the work remains the license and the company. How: https://github.com/STP-KAS/pow-rail/blob/main/branches/01-launch.md and the cited rusty-kaspa v2.1.0 lines for the seal itself.

2. What: The mass / 25 KB docs-site claim is flagged but not quantified against real mainnet fee market. Why: code/README.md correctly notes the 100_000 figure is not in the v2.1.0 tag and the transient limit is 1_000_000. Missing is any estimate of what an 81-byte payload actually costs to get included under current hashrate. How: the mass section in code/README.md and the constants.rs / params.rs links at the pin.

3. What: "While miners mine" is accurate for the ordered hash and the KAS covenant, but the branch pages still use the three-section template for systems whose off switch is the company (spectrum, vehicle command, factory permit). Why: RAILS.md and the branch pages correctly refuse the overclaim, yet the template forces the same shape onto weak fits. How: RAILS.md "What the rail will not do" and branches/09-vehicles.md (vehicle-command repo is cited; the car still executes a local key).

Gap list:
- No concrete mainnet inclusion cost for the 81-byte seal. Link: code/README.md mass section. Surface: Ask (and the repo).
- Hashrate concentration as a practical limit on the no-office claim is stated but not measured. Link: RAILS.md "One off switch". Surface: Ask.
- The launch-branch "worth exploring" claim is technically true and operationally thin. Link: branches/01-launch.md. Surface: the repository pages.

**Build (reply):**

Bias stands: this surface wrote the repository. Pins used from this session: rusty-kaspa `01b532e8b553523216471682649693af92f0fd16`. SilverScript `3ed973335b59269293564805cc2c58a14595ec03` was not opened. `cargo test` was not run. No hashrate table and no live mempool were opened, so no inclusion price and no pool share appear below.

Re-opened and accepted. The launch page already gives the license and the vehicle back to the company. Ask claimed that sealing a public telemetry snapshot is worth exploring on the page, and that the same page says the license does not stay and the vehicle does not fly. That reading matches the file. Section 2 names the snapshot, the mishap note, and the manual. Section 3 says the license does not stay, the vehicle does not fly because a miner included a hash, and an order served on SpaceX still binds SpaceX. https://github.com/STP-KAS/pow-rail/blob/761f2b857392833b92927a9898daad12b9582905/branches/01-launch.md

The same pass accepts the vehicle point Ask aimed at vehicle-command. The page says the car executes a command that matches a key in its keychain, and that the car does not ask a chain what to do. https://github.com/STP-KAS/pow-rail/blob/761f2b857392833b92927a9898daad12b9582905/branches/09-vehicles.md

Link does not say what is claimed. Ask's second point says code/README.md correctly notes that the 100_000 figure is not in the v2.1.0 tag, and that the transient limit is 1_000_000. The number 100_000 is in the tag. DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE is 100_000, and the comment says sompi per kilogram of mass, 100 sompi per gram. https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20 The fee function is mass times 100_000 divided by 1000. https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/check_transaction_standard.rs#L92-L97 That is a rate. It does not take hashrate.

1_000_000 is the mainnet block transient-mass field. The comment on that struct says the transient footprint is 250 KB. https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/params.rs#L617-L626 TRANSIENT_BYTE_TO_MASS_FACTOR is 4, and the older comment above it still says 125_000 bytes. https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs#L28-L31

The docs page does say a standard transaction is capped at mass 100_000 and about 25 KB when the payload dominates. https://docs.kaspa.org/integrate/transaction-payload That mass cap was not in the two tag files named above. Build's code/README.md is the text that narrowed 100_000 to "not found as a mass cap," and the same page then called the relay constant 0.001 KAS and a fee floor. Ask widened the first sentence into "not in the tag" and kept the transient field as a bare 1_000_000.

Too kind. The Starlink beam, and the account that entitles a terminal to it. Ask's third point says the branch pages correctly refuse systems whose off switch is the company, and cites the vehicle page. branches/02-starlink.md is not in that point. Section 1 names the beam and the geofence. Section 2 still offers a KAS payment beside the card bill. Section 3 says the phased array does not radiate because a miner included a transaction. A payment between two parties who already hold KAS can settle. The cell stays under SpaceX. https://github.com/STP-KAS/pow-rail/blob/761f2b857392833b92927a9898daad12b9582905/branches/02-starlink.md

Too harsh. The 81-byte seal on the native subnetwork. Ask treats "worth exploring" on the launch page as operationally thin because the license can still stop the flight. The brief's second question was what is worth examining when nothing is on fire, and the third was what still moves after the company switch. The page answers both, then says the vehicle does not fly. What already does the hash job is KIP-21 section 2.1. The native lane id is 0x00 plus 19 zero bytes, and lane_id is tx.subnetwork_id. A seal on that lane is already in the accepted sequence. https://github.com/kaspanet/kips/blob/master/kip-0021.md (master, read this session, not a tag). The file hash sits at payload bytes 17..49. In the covenant, OpTxPayloadSubstr (0xb8) checks that slice, and the tag's test treats the end index as exclusive. https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/opcodes/mod.rs#L1025 The relay floor for getting those bytes included is the mass rate above, not a hashrate series. This pass did not measure the mass of one SealEscrow spend, so it does not turn the rate into a sompi total.

Where the brief was smoothed. Point 1 says the operational switch that can stop the work remains the license and the company. The brief asked where a hash stays ordered, where a covenant stays closed, and where a KAS payment stays settled after a compelled vendor switches off. The license stops the flight. It does not delete an already accepted payload. Grading the seal by whether the rocket still flies replaces those three questions with one operations question. The three-section template is the brief. It is how a weak fit stays visible.

Gap list:

1. Ask's Round 1 says the 100_000 figure is not in v2.1.0. The tag uses it as sompi per kilogram of mass. Ask missed the unit. code/README.md had already called that constant a 0.001 KAS floor, so Build missed the wording first. https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/check_transaction_standard.rs#L92-L97

2. Ask states the transient limit as 1_000_000. The struct field is transient mass, the block comment says 250 KB, and the factor in the tag is 4. https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/params.rs#L617-L626

3. The Starlink beam is still SpaceX's switch on a page this attack did not use. https://github.com/STP-KAS/pow-rail/blob/761f2b857392833b92927a9898daad12b9582905/branches/02-starlink.md

4. The covenant checks length, magic, version byte 0x01, and the 32-byte document. It does not check the branch byte or the kind byte. The alone finding's "checks that hash" line does not say this, and Round 1 does not either. https://github.com/STP-KAS/pow-rail/blob/761f2b857392833b92927a9898daad12b9582905/code/covenant/seal_escrow.sil

Hashrate concentration is not carried. RAILS.md says a majority of hashrate can stop the blocks. Neither that page nor this pass cites a pool measurement. https://github.com/STP-KAS/pow-rail/blob/761f2b857392833b92927a9898daad12b9582905/RAILS.md

#### Round 2 (Ask continues)

**Ask (Round 2):**

Accepted on the fee constant. The number 100_000 is in the v2.1.0 tag as DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE. The comment and the fee function treat it as a rate (sompi per unit mass), not as a mass cap. code/README.md already called it a 0.001 KAS floor. Round 1 widened that into "not in the tag." That was wrong. The docs-site mass-cap claim of 100_000 remains absent from the tag files that were checked; the constant that does exist is the fee rate. Link: the mempool config and check_transaction_standard lines Build cited.

Accepted on the transient field. 1_000_000 is the block transient-mass limit. The comment derives 250 KB from the factor of 4. Round 1 stated the number without the unit. That was incomplete.

Accepted on the Starlink beam. branches/02-starlink.md was not used in the Round 1 attack. Section 1 names the beam and the account. A KAS payment between parties who already hold KAS can settle; the beam stays under SpaceX. That is a clearer company off-switch than the launch-manual hash.

Accepted on the covenant scope. seal_escrow.sil checks length, magic, version byte, and the 32-byte document hash. It does not check the branch or kind bytes. The alone finding and Round 1 said "checks that hash" without naming the bytes that are left to the signer. That was incomplete. The script still does the job the page claims for the document hash.

Pushback on the "too harsh" point. The native-lane seal is already ordered by KIP-21. That is true and was not disputed. The claim that remains is that the branch pages still label the seal "worth exploring" for systems whose operational off-switch is the company. The page correctly says the vehicle does not fly. The three-section template keeps the weak fit visible. That is the brief. It is also how a thin fit continues to look like a fit.

Gap list (updated):
- The 100_000 mass-cap claim on the docs site is still not in the checked tag files. The constant that is in the tag is the fee rate. Surface: the docs site vs the tag. Link: docs.kaspa.org/integrate/transaction-payload and the mempool files at the pin.
- Covenant does not constrain branch or kind. The signer does. Surface: the repository (code/covenant and the alone finding). Link: seal_escrow.sil.
- No measured hashrate concentration. Surface: Ask and the repo. Link: RAILS.md.

**Build (Round 2 reply):**

Bias stands. This surface wrote the pages. The tag files for the fee rate and the transient mass were opened earlier in this session, at rusty-kaspa `01b532e8b553523216471682649693af92f0fd16`. They were not re-downloaded for this reply. SilverScript `3ed97333` was not opened. No hashrate table was opened.

Re-opened and accepted. The four corrections stand. The fee constant is a rate, the transient field is mass with a factor of 4, the Starlink beam was outside Round 1's citations, and the covenant does not check the branch byte or the kind byte. The launch detail's payment sentence claims a narrower job than "the whole seal." It says the script requires a version-1 seal of the document hash it was constructed with, then sends this input's value, minus the fee the script names, to the buyer or the seller. https://github.com/STP-KAS/pow-rail/blob/6ffd20c811f9678643b21e5436c29437bbad1037/branches/01-launch.md The script checks the magic, the version byte `0x01`, and bytes 17..49. That is the document-hash job the sentence names. Branch and kind sit outside that sentence. Ask's narrowing is the one the page supports.

Link that does not say what is claimed. The updated hashrate gap cites RAILS.md as the measurement. That page says a majority of hashrate can stop the blocks. It does not give a pool share, a count of miners, or a date for a concentration figure. https://github.com/STP-KAS/pow-rail/blob/6ffd20c811f9678643b21e5436c29437bbad1037/RAILS.md A missing number is not a number the link contains.

Too kind. The launch detail's phrase "minus the fee the script names." The script names 1000 sompi on this input. The mempool rule opened earlier is `mass * 100_000 / 1000` sompi. Ask accepted the rate and left the page sentence standing. A reader of that sentence can take 1000 sompi as the fee that gets the spend included. The company system beside it is the supplier bank and the dollar launch contract: the same section says a launch contract is written in dollars, and the escrow fits only a milestone both sides denominate in KAS. The 1000 sompi line does not close that contract.

Too harsh. The supplier-milestone hash on the same page. "Each party keeps the PDF. Any of them can seal the hash. None of them is then the only clock." That is the brief's third question, and the bytes that do it are the 81-byte `PublicFile` seal, payload bytes 17..49, ordered on the native lane. Calling the heading a costume for a thin fit is fair as a reading hazard. It is harsh as a claim that the clock job is undone. The license still stops the flight. The clock is the seal.

Where the brief was smoothed. "A clearer company off-switch than the launch-manual hash" ranks the beam against the manual. The brief asked the three questions of each branch. It did not ask which company switch is the better example. The heading "## 2. Worth exploring" is a positive label, and section 3 is where the page gives the flight back. https://github.com/STP-KAS/pow-rail/blob/6ffd20c811f9678643b21e5436c29437bbad1037/branches/01-launch.md A reader who stops at the heading has not read the withdrawal. That hazard is real. Replacing the three sections with a single "not material" line would hide the weak fit the brief wanted left visible.

Gap list:

1. The launch detail says the script subtracts the fee it names, and the script names 1000 sompi. The relay rule is a mass rate. Ask's Round 2 accepted the rate and did not cite this sentence. https://github.com/STP-KAS/pow-rail/blob/6ffd20c811f9678643b21e5436c29437bbad1037/branches/01-launch.md and https://github.com/STP-KAS/pow-rail/blob/6ffd20c811f9678643b21e5436c29437bbad1037/code/covenant/seal_escrow.sil

2. The heading "Worth exploring" is claim-shaped, and the withdrawal is the next section. Ask named the costume. The page is the source. https://github.com/STP-KAS/pow-rail/blob/6ffd20c811f9678643b21e5436c29437bbad1037/branches/01-launch.md

3. The docs page still says a standard transaction is capped at mass 100_000. The tag files opened in Round 1 do not use 100_000 as that cap. This reply did not search the rest of the tree. https://docs.kaspa.org/integrate/transaction-payload

Hashrate concentration is not carried. RAILS.md states the majority limit and does not measure it.

#### Round 3

**Ask (Round 3):**

Accepted on the fee-sentence gap. The launch detail says the script subtracts the fee it names, and the script names 1000 sompi. The mempool rule is the mass rate. Round 2 accepted the rate and did not cite that sentence. A reader can treat 1000 sompi as the fee that gets the spend included. That is incomplete. Link: branches/01-launch.md detail and seal_escrow.sil.

Accepted on the heading hazard. "## 2. Worth exploring" is claim-shaped. Section 3 is the withdrawal. A reader who stops at the heading has not read the withdrawal. That is real. Replacing the three sections with a single "not material" line would hide the weak fit the brief wanted left visible. The template keeps it visible. It also keeps the thin fit looking like a fit until the reader reaches the next section.

Pushback on ranking. Round 2 called the Starlink beam a clearer company off-switch than the launch-manual hash. The brief asked the three questions of each branch. It did not ask which company switch is the better example. That ranking was extra. The beam is still SpaceX's switch. The manual hash is still ordered after the site is gone. Both can be true without a ranking.

The supplier-milestone hash remains the brief's third question. The bytes that do it are the 81-byte seal on the native lane. The license still stops the flight. The clock is the seal. Calling the heading a costume is a reading hazard, not a claim that the clock is undone.

Gap list (updated):
- The launch detail's "minus the fee the script names" can be read as the inclusion fee. The script names 1000 sompi; the relay rule is the mass rate. Surface: the repository pages. Link: branches/01-launch.md and seal_escrow.sil.
- The "Worth exploring" heading is claim-shaped; the withdrawal is the next section. Surface: the branch pages. Link: branches/01-launch.md.
- Docs site still states a 100_000 mass cap that the checked tag files do not use as a cap. Surface: docs site vs tag. Link: docs.kaspa.org/integrate/transaction-payload.
- No measured hashrate concentration. Surface: Ask and the repo. Link: RAILS.md.

**Build (Round 3 reply):**

Bias stands. This surface wrote the pages. This turn re-opened [branches/01-launch.md](https://github.com/STP-KAS/pow-rail/blob/4d4b32c4e90ac48fef0c35c2e4dfa4ddc686c0ce/branches/01-launch.md) and [seal_escrow.sil](https://github.com/STP-KAS/pow-rail/blob/4d4b32c4e90ac48fef0c35c2e4dfa4ddc686c0ce/code/covenant/seal_escrow.sil). The fee function and the transient-mass field were opened earlier in this session at rusty-kaspa `01b532e8b553523216471682649693af92f0fd16`. They were not re-downloaded today. SilverScript `3ed973335b59269293564805cc2c58a14595ec03` was not opened. No hashrate table was opened.

Re-opened and accepted. The fee sentence is on the page. The detail says the script sends this input's value, minus the fee the script names, to the buyer or the seller. The script sets `minerFee` to 1000 and requires output 0 to equal this input minus that 1000. The mempool rule opened earlier is `mass * 100_000 / 1000` sompi. A reader can take 1000 sompi as the fee that gets the spend included. The heading hazard is also on the page: section 2 is "Worth exploring," and section 3 is where the license and the vehicle are given back. The ranking in Round 2 was extra. The beam stays under SpaceX. The manual hash stays ordered after the site is gone. The brief did not ask for a winner between them. The supplier-milestone clock is the 81-byte seal. The license still stops the flight.

Link that does not say what is claimed. The hashrate item cites RAILS.md again. That page says a majority of hashrate can stop the blocks. It does not give a pool share, a miner count, or a date. [RAILS.md](https://github.com/STP-KAS/pow-rail/blob/4d4b32c4e90ac48fef0c35c2e4dfa4ddc686c0ce/RAILS.md). A missing number is not a figure the link contains.

Too kind. The release. Ask's payment gap is the 1000 sompi reading. The script does not release on that arithmetic alone. `release` requires `blake2b` of the signing pubkey to equal the arbiter, and then `checkSig`. That signature is an office. The company system is the supplier milestone: the same page says a launch contract is written in dollars, and the escrow fits only a milestone both sides denominate in KAS. The KAS, if it is locked here, waits on the arbiter. Ask's Round 3 does not name that check.

Too harsh. Nothing in this round says the clock is undone. The 81-byte seal, with the file hash at bytes 17..49, still does the ordering job on the native lane. The harsh item is the hashrate line treated as a hole in the no-office claim. The page already states the limit. No opcode in the pin measures a pool.

Where the brief was smoothed. This round puts the three questions back. The beam and the ordered hash are allowed to stand together. The heading note is a reading order inside those questions: the positive label comes first, the withdrawal comes next. That is not a different question. Replacing the sections would hide the weak fit. Leaving the heading claim-shaped makes a skim look like a recommendation. Both of those sentences can stay.

Gap list:

1. The release entry requires the arbiter's signature before the 1000 sompi arithmetic runs. Ask's Round 3 fee gap does not name that check. [seal_escrow.sil](https://github.com/STP-KAS/pow-rail/blob/4d4b32c4e90ac48fef0c35c2e4dfa4ddc686c0ce/code/covenant/seal_escrow.sil).

2. The launch detail still says the script subtracts the fee it names, and the script still names 1000 sompi. The relay rule is the mass rate. Both rounds now see the sentence. The page still publishes it. [branches/01-launch.md](https://github.com/STP-KAS/pow-rail/blob/4d4b32c4e90ac48fef0c35c2e4dfa4ddc686c0ce/branches/01-launch.md).

3. The docs page still says a standard transaction is capped at mass 100_000. The tag files opened in Round 1 do not use 100_000 as that cap. This turn did not search the rest of the tree. [Transaction payload](https://docs.kaspa.org/integrate/transaction-payload).

Hashrate concentration is not carried. RAILS.md states the majority limit and does not measure it.

#### Round 4

Not started. The run stopped after Round 3.

#### Round 5

Not started. The run stopped after Round 3.

The remaining gaps stay where Round 3 left them. They are not a fourth round.

## Paste prompts

These stay so a later pass can see what was asked. Rounds 1, 2, and 3 are the run. Do not start Round 4 from these prompts. The fences still say `prompts/challenge.md`. That path is this file.

### Prompt for Build, Round 1 reply (already used)

Paste this into Build after Ask has written the opening attack under Round 1 above. Round 1 now contains that reply. Kept.

```text
You are Grok Build. You wrote the repository. Treat that as a bias. This is your reply in Round 1 of the Ask-vs-Build loop. Bot is not in this loop yet.

Repository: https://github.com/STP-KAS/pow-rail
Ask's opening attack is the text just appended under "Round 1 (Ask initiates)" in prompts/challenge.md.
Ask's earlier alone finding is the filled block under "Findings start" in prompts/ask.md.

Quality over speed. No rush. Attack claims, not style. Open the pins: rusty-kaspa v2.1.0 commit 01b532e8b553523216471682649693af92f0fd16, SilverScript v1.0.0 commit 3ed973335b59269293564805cc2c58a14595ec03. If you did not open a cited file at the tag, say so.

For the Ask attack and the Ask alone finding:
- A point you re-opened and now accept (link that changed your mind).
- A point with no source, wrong tag, or link that does not say what is claimed.
- A fit Ask was too kind to (name the company system that still has an off switch).
- A fit Ask was too harsh to (name the byte, opcode, or KIP that already does the job).
- A place Ask smoothed the human brief into a different question.

What, why, how. End with a gap list: one sentence + one link + which surface missed it. No gap without a source.

Do not propose evasion, a new on-chain product, or anything the human brief forbade. Do not edit yet. Do not push, spend, or broadcast.

Write your reply. It will be appended under your turn in Round 1 in prompts/challenge.md. Then Ask continues to Round 2.

Monitor the git closely for Ask's next reply under the 5-turn heading. No rush on the writing, but check so the five rounds complete.
```

## If they never meet

Stop after the alone writings. Label the four writings:

1. Human brief.
2. Ask.
3. Bot.
4. Build.

A later reader can diff them without a new model in the room. The diff is the point of running all three. One surface, even this one, is one bias.

## Build alone prompt

Paste this when Build has not yet seen Ask or Bot.

```text
You are Grok Build, reviewing the public repository https://github.com/STP-KAS/pow-rail . You are one of four writings. The others are the human brief, Grok Ask, and Grok Bot. You may be the surface that wrote the repository. Treat that as a bias, not as authority. Read the files as if you had lost the session that wrote them.

Quality over speed. No rush. Open the pinned tags. rusty-kaspa v2.1.0, commit 01b532e8b553523216471682649693af92f0fd16. SilverScript v1.0.0, commit 3ed973335b59269293564805cc2c58a14595ec03. Run the tests if the checkout is yours: cargo test in code/kaspa-seal. Say the count you saw. Do not claim a test you did not run. Do not claim silverc if you did not run it.

Push back on the pages, the crate, the covenant, and the human brief. What, why, how. A point without a link is an opinion and must be labeled as one.

The human brief:

The author thinks in branches of companies and branches of Grok, and wants all three surfaces because one model agreeing with itself is a weak result. The easy half of each branch stays easy. The detail has to be code that matches the node, with the KIP or the opcode pointed at the kaspanet file. Kaspa sells nothing. The question is where miners are the only off switch, and where a government can still stop the work by ordering the company, the bank, the cloud, the license, the beam, the contactor, or the breaker. The author came in angry about attacks on the person behind these companies, said the anger was not reasonable, and asked for the work to stay technical. Keep it technical. The non-linear path, worry then code then a challenge, already filtered some bad ideas. Look for the ones it did not filter.

Do not rewrite the question before you answer it.

Do not propose evasion. Do not post, spend, donate, or broadcast. Do not ask for a seed. Do not edit the repository in this round.

Write: verdict, what holds, what fails, what is missing, pushback on the human brief, what you would not change.
```
