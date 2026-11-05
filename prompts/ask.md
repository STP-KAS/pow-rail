# Grok Ask

Paste the prompt below into Grok Ask. Ask has the chat and the public web. Ask does not edit the branch pages or the code. The opinion is not finished in the chat. It is written in the findings form under the prompt, and that form is pushed.

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

Commit reviewed: ef755966673c89fe6168dab4a73701fa058aa98e

Links opened: README.md, DISCLAIMER.md, RAILS.md, GITHUBS.md, code/README.md, code/covenant/README.md, branches/01-launch.md, LOG.md, rusty-kaspa v2.1.0 release page, several raw kaspanet links via the code notes citations.

Links named and not opened: most individual branch pages beyond 01-launch (sampled the pattern), full kaspanet source trees at the pin (relied on the cited line numbers in code/README.md), SilverScript v1.0.0 full tree, Kaspa Explained pages, the WIRED/Verge Musk articles (cited in LOG.md).

1. Verdict.

The repository correctly isolates the one property a public proof-of-work chain actually has that a company does not: there is no office that can be ordered to stop the blocks. It then refuses almost every overclaim about what that property is good for. Sealing an 81-byte hash of a public file, and locking KAS under a covenant that checks that hash, are live and match the pinned node. Licenses, vehicles, spectrum, export rules, and real-time control are not. I refuse to believe this is operationally material for most of the named branches. The fit is narrow, archival or bilateral, and the repository says so.

2. What holds.

- What: Node pin is rusty-kaspa v2.1.0, commit 01b532e8b553523216471682649693af92f0fd16, published 22 September 2026. Why: Release page lists that commit and date. How: https://github.com/kaspanet/rusty-kaspa/releases/tag/v2.1.0
- What: PayloadDigest is BLAKE3 keyed with domain "PayloadDigest" (13 bytes + 19 zero key bytes); id_v1 splits payload digest from the rest. Why: code/README.md cites consensus/core/src/hashing/tx.rs and crypto/hashes/src/hashers.rs at the pin; the empty one-shot matches the published ZERO_PAYLOAD_DIGEST. How: those file links at tag v2.1.0.
- What: 81-byte seal layout (magic, version, branch, kind, flags, 32-byte unkeyed BLAKE3, optional second hash) and the covenant checks OpTxPayloadLen == 81 and the magic + document hash via OpTxPayloadSubstr. Why: code/README.md and code/covenant/README.md match the opcodes present in the v2.1.0 tag. How: the sil file and the cited txscript opcodes.
- What: KIP-21 is Active; user lane id is 4-byte namespace + 16 zeros with the non-zero constraint; native is 20 zero bytes. Why: code/README.md quotes section 2.1 and the status commit. How: https://github.com/kaspanet/kips/blob/master/kip-0021.md
- What: DAGKnight (KIP-2) is Proposed and not in v2.1.0; vProgs has no release; Kaspa Dollar is a design, not a coin. Why: RAILS.md and code/README.md state the status as of the pass date. How: the KIP links and the vprogs repo tip cited.
- What: Company GitHub orgs are correctly distinguished from lookalikes. Why: GITHUBS.md lists the real orgs and the false ones. How: the linked org pages.

3. What fails.

- What: The docs site claims a standard transaction mass cap of 100_000 that would limit payload-dominated txs to ~25 KB. Why: code/README.md correctly notes that constant was not found in the v2.1.0 tag; the actual mainnet transient limit is 1_000_000. The repository flags the mismatch and does not treat the docs figure as consensus. This is a documented discrepancy, not a silent error, but a reader who only sees the docs site would be wrong. How: code/README.md mass section and the cited constants.rs / params.rs at the pin.
- What: Sealing a public telemetry snapshot or manual revision is presented as "worth exploring." Why: It is technically possible and the hash stays ordered, but the operational exposure it removes is weak relative to the license, range, or supplier bank exposures the same page lists. The page itself says the license does not stay and the vehicle does not fly. The claim holds as engineering; the "worth" is soft. How: branches/01-launch.md sections 2 and 3.
- What: Covenant fee is 1000 sompi while the mempool minimum relay fee is 100_000 sompi. Why: code/covenant/README.md notes this and says a real spend must satisfy the mempool. The script is correct for its own named fee; it is not a drop-in mainnet template without adjustment. How: the sil file and the mempool config citation.

4. What is missing.

- A concrete mainnet fee and inclusion estimate for an 81-byte payload under current hashrate and fee market. The mass math is there; the practical cost is not.
- Explicit treatment of hashrate concentration as a practical limit on the "no office" claim. Subpoenaing a foundation does nothing; a majority of hashrate going offline or being compelled does. The repository states the limit but does not quantify current concentration.
- Any worked example of a verifier actually reconstructing the seal and checking it against a public file. The crate and the tests exist; a short public verification note does not.

5. Pushback on the human brief.

The branch habit forced every page to answer the same three questions and kept the scope honest. It also forced weak fits into the same template: a launch license and a public manual hash are not the same kind of exposure, yet both get the three-section treatment. "While miners mine" correctly describes ordered hashes and KAS covenants; it does almost no work for anything that requires the company to continue operating (spectrum, vehicle command, factory permits). The brief asked for engineering, not a manifesto, and the repository largely delivered that. The "Go fuck yourself" line is sourced and placed where the author asked; it does not change the technical claims.

6. What you would not change.

The pin discipline, the live-versus-pipeline distinction, the explicit refusal to put export-controlled or private data on the chain, the "not for sale" framing, the code that matches the cited tag instead of describing it, and the requirement that findings land in the file rather than only in chat.

