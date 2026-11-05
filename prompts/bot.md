# Grok Bot

Paste the prompt at the bottom into Grok Bot. Bot is a Grok session that is not Ask and not this Build checkout. Bot may have tools. The review is text. Tools are for reading public pages and public code. They are not for acting.

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
- Do not edit this repository or any other.
- Do not treat a desk bot, a testnet wallet, or a private RPC as part of the review.

Write the opinion in this shape:

1. Verdict. Four to eight lines. Include one sentence on what your tools changed, and what they could not reach.
2. What holds. Re-fetched claims only. What, why, how, link.
3. What fails. Include mismatches between a branch Detail and code/kaspa-seal, and between code/kaspa-seal and the v2.1.0 file it cites.
4. What is missing. Name the unread source that would close it, or say that no such source is public.
5. Pushback on the human brief. The branch grid, the miner off-switch sentence, and the choice to keep the easy pages.
6. What you would not change.

Label guesses. Stop when the fetched pages stop.
```
