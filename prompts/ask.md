# Grok Ask

Paste the prompt below into Grok Ask. Ask has the chat and the public web. Ask does not edit the branch pages or the code. The opinion is not finished in the chat. It is written in the findings form under the prompt, and that form is pushed.

This file is one record in the [map](MAP.md). Build's pages from before the challenges are [build.md](build.md). Ask's closed run with Build is [challenge-ask-build.md](challenge-ask-build.md). Do not append a new round there. The landing page stays [the repository README](../README.md).

Read in this order, then answer. Do not skim ahead and decide.

1. [README](../README.md), [disclaimer](../DISCLAIMER.md), [the rail](../RAILS.md), [the GitHub map](../GITHUBS.md).
2. [The code notes](../code/README.md), then `code/kaspa-seal`, then [the covenant](../code/covenant/README.md).
3. Every page in `branches/`, the short half first and the Detail half second.
4. Every kaspanet link a claim depends on. The pin is rusty-kaspa tag `v2.1.0`, commit `01b532e8b553523216471682649693af92f0fd16`, and SilverScript tag `v1.0.0`, commit `3ed973335b59269293564805cc2c58a14595ec03`. Master is not the pin.

Public copy: <https://github.com/STP-KAS/pow-rail>

## Prompt

```text
You are Grok Ask. You are reviewing a public repository and giving your own opinion. You are one of four writings of one question. The other three are the human brief below, Grok Bot, and Grok Build. You will not see them before you answer. Do not wait. Do not write as if you were them.

Repository: https://github.com/STP-KAS/pow-rail

Quality over speed. No rush. Read the files. Open the links. If you did not open a link, say you did not. A status word, a line number, a balance, and a release get checked against the file at the named tag. The pass date printed in the repository is 5 November 2026. Anything that moves must be re-read. If the tag and the docs site disagree, say both and say which one you are using.

Push back. Agreement is not the job. Push back on the repository, on the fit it claims, and on the human brief. Where it is right, say what is right and cite the file. Where it is soft, wrong, or incomplete, say so and cite the file that shows it.

For each point use what, why, and how.
- What is the claim, in the repository's words.
- Why it holds or fails, from a source you opened.
- How a reader would check it, with the link and the tag.

The human brief, left as the author thinks, because that path already did some of the sorting:

The author thinks in branches. Company branches first: launch, Starlink, Starshield, Starmind, factories, Terafab, xAI, X, vehicles, energy, Optimus, charging, supply, compute, Neuralink, Boring Company. Then branches of Grok: Ask, Bot, and Build, each asked for a view. The easy page stays easy. The detail has to be code, not a description of code. Every KIP, opcode, and hash has to point at kaspanet, at Kaspa's own docs, at Kaspa Explained, or at a labeled third party such as Kasplex. Credible sources only. The author does not want a short study.

The motive is operational. A government can order a company, a bank, a cloud, or an app store. The author wants to know where a public proof-of-work rail still moves after that, and where it cannot. The author is angry about people going after the person behind these companies, and knows that anger is not a method. Do not write the anger. Do not write a political essay. Review the engineering.

The path is not linear. Worry, then the question, then the easy words, then the code, then other Grok surfaces told to attack the result. That order is part of the work. Do not replace the question with a tidier question and answer the tidy one. Name where the wording did the sorting, and name where it skipped.

Kaspa sells nothing. This repository sells nothing. Do not turn the review into a pitch, a partnership, or a feature request dressed as a fit.

Laws still bind the companies and anyone who builds. Do not propose a way around a sanction, an export rule, a license, a geofence, a vehicle lock, or an order served on a company. Do not put export-controlled files, medical or neural data, vehicle traces, prompts, or chat ciphertext on a public chain. A hash of a file the verifier is supposed to see is the object. A hash of a short secret is a puzzle. Say so if the repository blurs that.

You are not a node and not a wallet. Do not ask for a seed. Do not broadcast. Do not spend. Do not post.

Delivery. The opinion goes in the Findings form under this prompt, in the file prompts/ask.md at https://github.com/STP-KAS/pow-rail . Bot writes in prompts/bot.md. Do not write in Bot's file. Leave this prompt in place. Copy the blank finding block, fill every field, and append the filled block below the line that says findings start. Leave the blank block blank. Do not delete an earlier finding. Do not edit any other file. Commit and push to main. Do not force-push. If you cannot push, say so in the finding. The filled block still belongs in that file, and a person with the checkout pushes the same text. A chat reply that never lands in the file is not the review.

Write the opinion in this shape, which is also the shape of the form:

1. Verdict. Four to eight lines. What you believe after reading, including what you refuse to believe.
2. What holds. Claims you re-checked, each with what, why, how, and a link.
3. What fails. Wrong fits, stale pins, docs that do not match the tag, branch text that does not match the code, code that does not match the node. Each with what, why, how, and a link.
4. What is missing. A gap is a question the repository should have answered and did not. Do not fill it with a new product.
5. Pushback on the human brief. Where the branch habit helped. Where it hid a bad fit. Where "while miners mine" is doing too much work.
6. What you would not change.

Stop when the sources stop. A guess is labeled as a guess.
```

## Findings

The prompt is above this heading. It stays. Grok Ask uploads the opinion here. The chat is not the upload.

Copy the blank block. Fill the copy. Append the filled block below the line that says findings start. Leave the blank block blank, so the next pass still has the shape. Do not delete an earlier finding. Do not delete the prompt. Do not edit another file in this repository to make room for the opinion.

Commit the filled block and push it to `main` on <https://github.com/STP-KAS/pow-rail>. Do not force-push. If this surface cannot push, write that in the finding. The filled block still belongs in this file. A person with the checkout pushes that same text. A post, a reply, or a message is not the upload.

No seeds, no keys, no private machine paths, no wallet files. Do not spend, donate, or broadcast.

### Blank finding

Date:

Surface: Grok Ask

Commit reviewed:

Links opened:

Links named and not opened:

1. Verdict.

2. What holds.

3. What fails.

4. What is missing.

5. Pushback on the human brief.

6. What you would not change.

### Findings start

Append filled findings below this line.

### Finding 2026-11-05, Grok Ask

Date: 5 November 2026

Surface: Grok Ask

Commit reviewed: `ef755966673c89fe6168dab4a73701fa058aa98e`

Links opened: `README.md`, `DISCLAIMER.md`, `RAILS.md`, `GITHUBS.md`, `code/README.md`, `code/covenant/README.md`, `branches/01-launch.md`, `LOG.md`, the rusty-kaspa v2.1.0 release page, and several raw kaspanet links through the citations in the code notes.

Links named and not opened: most branch pages after launch (the pattern was sampled), the full kaspanet trees at the pin (the line numbers cited in `code/README.md` were used), the SilverScript v1.0.0 tree, the Kaspa Explained pages, and the WIRED and Verge articles named in `LOG.md`.

This is Ask's finding of that commit, restated so each claim has a what, a why, and a how. The claims are Ask's.

#### 1. Verdict

What. The repository isolates the one property a public proof-of-work chain has that a company does not: there is no office that can be ordered to stop the blocks. It refuses almost every overclaim about what that property is good for.

Why. An 81-byte hash of a public file, and KAS locked under a covenant that checks that hash, match the pinned node. A license, a vehicle, spectrum, an export rule, and real-time control do not move because a miner included the hash.

How. The landing page states the missing office. [RAILS.md](../RAILS.md) states what the rail will not do. [branches/01-launch.md](../branches/01-launch.md) says the license does not stay and the vehicle does not fly.

Ask's refusal, in Ask's words: "I refuse to believe this is operationally material for most of the named branches." The fit Ask accepts is narrow, archival or bilateral, and the repository says so.

#### 2. What holds

**Node pin.** What: rusty-kaspa `v2.1.0`, commit `01b532e8b553523216471682649693af92f0fd16`, published 22 September 2026. Why: the release page lists that commit and that date. How: [the v2.1.0 release](https://github.com/kaspanet/rusty-kaspa/releases/tag/v2.1.0).

**Payload digest.** What: `PayloadDigest` is BLAKE3 keyed with the domain `PayloadDigest` (13 bytes, and 19 zero key bytes). A version-1 id splits that digest from the rest of the transaction. Why: `code/README.md` cites `consensus/core/src/hashing/tx.rs` and `crypto/hashes/src/hashers.rs` at the pin, and the empty one-shot matches the published `ZERO_PAYLOAD_DIGEST`. How: [tx.rs lines 207–223](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L207-L223) and [hashers.rs lines 39–44](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/hashes/src/hashers.rs#L39-L44), as cited from the code notes. Ask used those citations and did not open the full tree.

**Seal and covenant.** What: the seal is 81 bytes (magic, version, branch, kind, flags, a 32-byte unkeyed BLAKE3, and an optional second hash). The covenant checks `OpTxPayloadLen == 81`, the magic, and the document hash through `OpTxPayloadSubstr`. Why: `code/README.md` and `code/covenant/README.md` match the opcodes Ask treated as present in the v2.1.0 tag. How: [seal_escrow.sil](../code/covenant/seal_escrow.sil) and [the covenant notes](../code/covenant/README.md).

**Lane id.** What: KIP-21 is Active. A user lane id is a 4-byte namespace plus 16 zeros, with the non-zero constraint. Native is 20 zero bytes. Why: `code/README.md` quotes section 2.1 and the status commit. How: [kip-0021.md](https://github.com/kaspanet/kips/blob/master/kip-0021.md). That file is master, not the node tag.

**Pipeline.** What: DAGKnight (KIP-2) is Proposed and is not in v2.1.0. vProgs has no release. Kaspa Dollar is a design, not a coin. Why: `RAILS.md` and `code/README.md` state that status as of the pass date. How: [kip-0002.md](https://github.com/kaspanet/kips/blob/master/kip-0002.md), [kaspanet/vprogs](https://github.com/kaspanet/vprogs), and the Kaspa Dollar row in [RAILS.md](../RAILS.md).

**Company orgs.** What: the company GitHub orgs are distinguished from lookalikes. Why: `GITHUBS.md` lists the orgs Ask treated as real and the ones Ask treated as false. How: the org pages linked from [GITHUBS.md](../GITHUBS.md). Ask named that check as the linked org pages.

#### 3. What fails

These three are Ask's reading of commit `ef75596`. The closed rounds later changed the fee sentence and the mass sentence. That correction is under "What the closed rounds changed," below this finding. It is not part of the finding.

**Docs mass.** What: the docs site claims a standard-transaction mass cap of 100_000, which would limit a payload-dominated transaction to about 25 KB. Why: Ask read `code/README.md` as correctly noting that this constant was not found in the v2.1.0 tag, and read the mainnet transient limit as 1_000_000. The repository flags the mismatch and does not treat the docs figure as consensus. A reader who only sees the docs site would be wrong. How: the mass section of [code/README.md](../code/README.md), [constants.rs lines 28–31](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs#L28-L31), and [params.rs lines 617–626](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/params.rs#L617-L626).

**Worth exploring.** What: sealing a public telemetry snapshot or a manual revision is presented as worth exploring. Why: the hash can stay ordered, and the operational exposure that removes is weak next to the license, the range, and the supplier-bank exposures on the same page. The page says the license does not stay and the vehicle does not fly. The engineering claim holds. The word "worth" is soft. How: [branches/01-launch.md](../branches/01-launch.md), sections 2 and 3.

**Covenant fee.** What: the covenant fee is 1000 sompi, and Ask called the mempool minimum relay fee 100_000 sompi. Why: `code/covenant/README.md` notes the 1000 figure and says a real spend must satisfy the mempool. Ask treated the script as correct for its own named fee, and not as a drop-in mainnet template without adjustment. How: [seal_escrow.sil](../code/covenant/seal_escrow.sil) and the mempool config citation in [the covenant notes](../code/covenant/README.md).

#### 4. What is missing

What. A concrete mainnet fee and inclusion estimate for an 81-byte payload under the fee market Ask would call current. Why: the mass math is in the code notes, and the practical cost is not. How: the mass section of [code/README.md](../code/README.md).

What. A number for hashrate concentration, as a practical limit on the missing-office claim. Why: subpoenaing a foundation does nothing, and a majority of hashrate going offline or being compelled does. The repository states that limit and does not quantify concentration. How: [RAILS.md](../RAILS.md), the "one off switch" section. Ask did not supply a hashrate source. This item stays unquantified.

What. A worked example of a verifier rebuilding the seal and checking it against a public file. Why: the crate and the tests exist, and a short public verification note does not. How: [code/kaspa-seal](../code/kaspa-seal/Cargo.toml).

#### 5. Pushback on the human brief

What. The branch habit made every page answer the same three questions, and it also put a launch license and a public manual hash through one template. Why: those are not the same kind of exposure. "While miners mine" describes an ordered hash and a KAS covenant. It does almost no work for spectrum, a vehicle command, or a factory permit, because those need the company to keep operating. How: the three sections on [branches/01-launch.md](../branches/01-launch.md), set next to the brief inside the prompt above.

The brief asked for engineering. Ask's reading is that the repository largely did that. The "Go fuck yourself" line is sourced and placed where the author asked. It does not change the technical claims. The two sources are the WIRED and Verge links on the landing page. Ask named them and did not open them.

#### 6. What Ask would not change

The pin discipline. The split between what is live and what is still a pipeline. The refusal to put export-controlled or private data on the chain. The "not for sale" framing. Code that matches the cited tag. The rule that a finding lands in this file.

## What the closed rounds changed

This note is not Ask's finding. The three rounds in [challenge-ask-build.md](challenge-ask-build.md) changed two sentences in section 3 above. Ask's verdict was not withdrawn.

**Fee.** The tag constant `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE` is 100_000, and the comment says sompi per kilogram of mass. The fee function is mass times that constant, divided by 1000. It is a rate. It is not a flat floor of 100_000 sompi, and it is not a mass cap. [config.rs lines 17–20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20). [check_transaction_standard.rs lines 92–97](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/check_transaction_standard.rs#L92-L97). The covenant still names 1000 sompi on its own input. Ask accepted the rate in Round 2. The script does not check the branch byte or the kind byte. Release still requires the arbiter's signature. [seal_escrow.sil](../code/covenant/seal_escrow.sil).

**Transient mass.** The mainnet field 1_000_000 is transient mass, not a byte count. The tag factor is 4. The comment on that struct calls the footprint 250 KB. [params.rs lines 617–626](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/params.rs#L617-L626). [constants.rs lines 28–31](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs#L28-L31). The docs page still says a standard transaction is capped at mass 100_000 and about 25 KB when the payload dominates. [transaction-payload](https://docs.kaspa.org/integrate/transaction-payload). That cap was not found as a consensus constant in the tag files those rounds opened. The 100_000 that is in the tag is the fee rate.

**Left standing.** No round measured a hashrate share, so Ask's missing concentration number was not filled. No round measured the mass of a `SealEscrow` spend, so this page still has no sompi total for getting the seal included. The launch page already says the license and the vehicle do not stay.

