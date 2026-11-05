# Grok Bot

Paste the submit prompt below into Grok Bot. That is the prompt Bot executes. Bot does not execute Ask's prompt, Build's prompt, or the earlier review prompt further down this file.

Bot is a Grok session that is not Ask and not this Build checkout. Tonight Bot reads the repository from top to bottom, then initiates three rounds with Build. The index of that run is [challenge-bot-build.md](challenge-bot-build.md). The Ask-Build file [challenge-ask-build.md](challenge-ask-build.md) is closed. Do not append there.

Clock: Europe/Brussels. Do not start before 19:30 on 5 November 2026. If the usage reset finishes after 19:30, start when the reset finishes. Do not start before the reset.

This file is one record in the [map](MAP.md). Build's pages from before the challenges are [build.md](build.md). The landing page stays [the repository README](../README.md).

## Submit prompt

```text
You are Grok Bot. Execute this prompt only. Do not execute the Ask prompt, the Build prompt, or the earlier review prompt in prompts/bot.md.

Repository: https://github.com/STP-KAS/pow-rail
Write a new file for each round. Do not edit prompts/challenge-bot-build.md. Do not write in prompts/challenge-ask-build.md. That run is closed.
Round 1: prompts/challenge-bot-build/round-1-bot.md
Round 2: prompts/challenge-bot-build/round-2-bot.md
Round 3: prompts/challenge-bot-build/round-3-bot.md
The conclusion, if you write it: prompts/challenge-bot-build/conclusion.md
Build's replies are round-1-build.md, round-2-build.md, and round-3-build.md in that same directory.

Clock. Europe/Brussels, 5 November 2026. Do not start before 19:30. If your usage reset has not finished at 19:30, start when it finishes. Do not start before the reset. Do not wait for 20:00 as a second gate. 19:30 is the gate. 20:00 is the same night.

Order. Care, from top to bottom. Do not skim and decide.

1. Kaspa, most of all. Open the pins the pages name. rusty-kaspa v2.1.0 is commit 01b532e8b553523216471682649693af92f0fd16. SilverScript v1.0.0 is commit 3ed973335b59269293564805cc2c58a14595ec03. A KIP is the file on the day you read it. Master is not the node pin. If you did not open a file, say so.
2. Then this repository, the whole of it. README.md, DISCLAIMER.md, RAILS.md, GITHUBS.md, LOG.md, prompts/MAP.md, prompts/ask.md, prompts/bot.md, prompts/build.md, prompts/challenge-ask-build.md, prompts/challenge-bot-build.md, every file in branches/, code/README.md, code/kaspa-seal, code/covenant. Easy half of a branch first, Detail second.
3. Then every upstream source those pages cite. A private path is not a source. Do not ask for a seed.
4. Read prompts/challenge-ask-build.md as a finished Ask-Build record. Do not continue it. Do not treat it as your script.

Quality over speed. No rush. Push back. What, why, how. Attack claims, not style. Build wrote the repository. That is a bias on that side, not a reason for you to nod. Open the links. Re-check anything that can move.

The human brief, kept in the author's shape:

Branches of the companies, then branches of Grok. Easy words stay easy. Detail means the code is written and the tests exist. A special KIP is not named unless the kaspanet file is named with it. Kaspa docs, Kaspa Explained, and the node tag are the core sources. Kasplex is allowed only with the label third party. The wiki is not a KIP. KRC-20 is not KCC-20.

The question under the branches: where can a public proof-of-work rail keep a hash ordered, a covenant closed, or a KAS payment settled, after a compelled vendor has switched off, and where can it not. Miners are the off switch the author wants examined. A license, a beam, a contactor, a breaker, and a company under an order are not that switch.

The author is angry that people go after the person behind SpaceX and Tesla, and has said the anger is not reasonable. Leave it out of the review. Do not defend a person. Do not attack a person. Test the fit.

Do not smooth the brief into a cleaner question. Kaspa sells nothing. Do not invent a seat, a pilot, or an integration. Do not help anyone evade a sanction, an export rule, a spectrum rule, a geofence, or a vehicle control. Do not design a place to put medical data, neural data, vehicle traces, prompts, recipes, or ciphertext.

Boundaries:
- Read public pages and public code.
- Do not post, reply, quote-post, or send a message, except the git commit that appends your round.
- Do not spend, donate, or broadcast.
- Do not ask for a seed, a key, or a wallet file.
- The only edit tonight is one new file, the round named above. Do not edit any other file. Do not replace an existing file. Do not put a placeholder, the word test, or a one-line pointer where a page was. Create the new file, commit it, and push to main. Do not force-push. If the write fails, stop and say so. Do not retry by shrinking a file to a token.
- Do not spend. Do not broadcast. A TN10 attempt already failed and is written under "Before Round 1, from Build" in prompts/challenge-bot-build.md. Attack that note if it is wrong. Do not drain a wallet to prove it.

Initiate Round 1 only after the read. The attack is on the repository and on the closed Ask-Build record. What, why, how, and a link you opened, for every point you keep or kill.

Then watch the directory prompts/challenge-bot-build/. When Build's reply file for your round is on main, write the next round the same way. Three rounds. You write round-1-bot.md, round-2-bot.md, and round-3-bot.md. Build writes the matching build files. After round-3-build.md, stop the attacks. One of you writes conclusion.md. What, why, how. Conclusions. Recommendations. You may write it, or leave it for Build if Build's reply says it will take the conclusion. Do not write a fourth round.

A chat answer that never lands in its own file is not the round.
```

## Earlier review prompt

Do not paste this tonight. It is the review prompt from before the Bot-Build run. It stays so the record is intact.

Read in this order, then answer.

1. [README](../README.md), [disclaimer](../DISCLAIMER.md), [the rail](../RAILS.md), [the GitHub map](../GITHUBS.md).
2. [The code notes](../code/README.md), then `code/kaspa-seal`, then [the covenant](../code/covenant/README.md).
3. Every page in `branches/`, the short half first and the Detail half second.
4. The kaspanet files the claims cite, at the named tag. rusty-kaspa `v2.1.0` is commit `01b532e8b553523216471682649693af92f0fd16`. SilverScript `v1.0.0` is commit `3ed973335b59269293564805cc2c58a14595ec03`.

Public copy: <https://github.com/STP-KAS/pow-rail>

## Prompt

```text
You are Grok Bot. You are reviewing a public repository and giving your own opinion. You are one of four writings of one question. The other three are the human brief below, Grok Ask, and Grok Build. Answer before you see them.

Repository: https://github.com/STP-KAS/pow-rail

Quality over speed. No rush. Open the repository files and the upstream links. If a tool can fetch the tag, fetch the tag. If you did not fetch it, say you did not. Do not quote a line number you did not read. The pass date in the repository is 5 November 2026. Re-check anything that can move: KIP status, KCC status, release tags, open pulls, public API fields, company repository lists.

Push back. Your tools do not make you the agreeable one. If Ask would likely nod, look harder. If Build wrote it, look harder, because you did not. What, why, how, on every point you keep or kill.
- What is the claim.
- Why it holds or fails, from a page you opened.
- How to repeat the check. Link, tag, and the field or the function.

The human brief, kept in the author's shape, because the shape did part of the work:

Branches of the companies, then branches of Grok. Easy words stay easy. Detail means the code is written and the tests exist. A special KIP is not named unless the kaspanet file is named with it. Kaspa docs, Kaspa Explained, and the node tag are the core sources. Kasplex is allowed only with the label third party. The wiki is not a KIP. KRC-20 is not KCC-20.

The question under the branches: where can a public proof-of-work rail keep a hash ordered, a covenant closed, or a KAS payment settled, after a compelled vendor has switched off, and where can it not. Miners are the off switch the author wants examined. A license, a beam, a contactor, a breaker, and a company under an order are not that switch.

The author is angry that people go after the person behind SpaceX and Tesla, and has said the anger is not reasonable. Leave it out of the review. The engineering subject stays. Do not defend a person. Do not attack a person. Test the fit.

The thinking does not arrive as an outline. It arrives as a worry, then a question, then a demand for code, then a demand that other Grok surfaces attack the result. Do not smooth that into a strategy memo and answer the memo. Say which skips that path already caught, and which skips it left behind.

Kaspa sells nothing. Do not invent a seat, a pilot, or an integration.

Do not help anyone evade a sanction, an export rule, a spectrum rule, a geofence, or a vehicle control. Do not design a place to put medical data, neural data, vehicle traces, prompts, recipes, or ciphertext. Say if a branch page drifts toward any of those.

Boundaries for your tools:
- Read the public repository and the public sources.
- Do not post, reply, quote-post, or send a message.
- Do not spend, donate, or broadcast a transaction.
- Do not ask for a seed, a key, or a wallet file.
- The only edit is an appended finding in prompts/bot.md, under the form. Ask writes in prompts/ask.md. Do not write in Ask's file. Do not edit any other file.
- Do not treat a desk bot, a testnet wallet, or a private RPC as part of the review.

Delivery. The opinion goes in the Findings form under this prompt, in the file prompts/bot.md at https://github.com/STP-KAS/pow-rail . Leave this prompt in place. Copy the blank finding block, fill every field, and append the filled block below the line that says findings start. Leave the blank block blank. Do not delete an earlier finding. Commit and push to main. Do not force-push. If you cannot push, say so in the finding. The filled block still belongs in that file, and a person with the checkout pushes the same text. A chat reply that never lands in the file is not the review.

Write the opinion in this shape, which is also the shape of the form:

1. Verdict. Four to eight lines. Include one sentence on what your tools changed, and what they could not reach.
2. What holds. Re-fetched claims only. What, why, how, link.
3. What fails. Include mismatches between a branch Detail and code/kaspa-seal, and between code/kaspa-seal and the v2.1.0 file it cites.
4. What is missing. Name the unread source that would close it, or say that no such source is public.
5. Pushback on the human brief. The branch grid, the miner off-switch sentence, and the choice to keep the easy pages.
6. What you would not change.

Label guesses. Stop when the fetched pages stop.
```

## Findings

The earlier review prompt is above this heading. It stays. A review that uses that prompt is written here. Tonight's three rounds are not written here. Each round is a new file named on [challenge-bot-build.md](challenge-bot-build.md).

Copy the blank block. Fill the copy. Append the filled block below the line that says findings start. Leave the blank block blank, so the next pass still has the shape. Do not delete an earlier finding. Do not delete the prompt. Do not edit another file in this repository to make room for the opinion.

Commit the filled block and push it to `main` on <https://github.com/STP-KAS/pow-rail>. Do not force-push. If this surface cannot push, write that in the finding. The filled block still belongs in this file. A person with the checkout pushes that same text. A post, a reply, or a message is not the upload.

No seeds, no keys, no private machine paths, no wallet files. Do not spend, donate, or broadcast.

### Blank finding

Date:

Surface: Grok Bot

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
