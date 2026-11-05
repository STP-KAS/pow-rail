# Bot challenge

Bot opens this run. Build replies. Three rounds, then one conclusion. The file is named `challenge-bot-build.md` because Bot writes first.

The Ask challenge is [challenge-ask-build.md](challenge-ask-build.md). It stopped after three rounds. Do not append there.

Bot executes the submit prompt in [bot.md](bot.md). Bot does not execute Ask's prompt or the earlier review prompt.

Public copy: <https://github.com/STP-KAS/pow-rail>

Quality over speed. No rush. Push back. What, why, how. Open the links. Attack claims, not style.

Clock: Europe/Brussels, 5 November 2026. Bot does not start before 19:30. If Bot's usage reset finishes later than that, Bot starts when the reset finishes. Bot does not start before the reset, and Bot does not wait for a second bell at 20:00.

## How to write

Each turn is a new file. Do not edit this page to add a round. Do not replace any existing file. Ask's connector deleted `prompts/challenge.md` when the update was the whole page. A new small file does not do that.

| Turn | File |
| --- | --- |
| Bot, Round 1 | [round-1-bot.md](challenge-bot-build/round-1-bot.md) |
| Build, Round 1 | [round-1-build.md](challenge-bot-build/round-1-build.md) |
| Bot, Round 2 | [round-2-bot.md](challenge-bot-build/round-2-bot.md) |
| Build, Round 2 | [round-2-build.md](challenge-bot-build/round-2-build.md) |
| Bot, Round 3 | [round-3-bot.md](challenge-bot-build/round-3-bot.md) |
| Build, Round 3 | [round-3-build.md](challenge-bot-build/round-3-build.md) |
| Conclusion | [conclusion.md](challenge-bot-build/conclusion.md) |

Create the file with the round text only. Commit that file. Push to `main`. Do not force-push. If the write fails, stop. Do not retry by replacing a page with a placeholder.

No seeds, no keys, no private machine paths, no wallet files. Do not spend, donate, broadcast, or post.

Build watches this directory from 19:30 Europe/Brussels. When Bot's file lands, Build writes the matching reply file. When that reply lands, Bot writes the next round. After `round-3-build.md`, Build writes `conclusion.md` and applies the wording both rounds agreed. There is no fourth attack round. Do not write a second conclusion.

## Before Round 1, from Build

The general run stands. Three changes are already made.

The write path is one new file per turn, because a full-file update on the Ask-Build page was committed as `PLACEHOLDER_CONTENT`, then `See local staging`, then `test`.

The wallet is not part of the attack. A local TN10 node was synced, `serverVersion` 2.1.0. The published address `kaspatest:qzffl5xy9np46gkttyuftqnv2w04pr8g3wsp7c3vv8se3txtelx6q7c0v0ldx` had coins. The smallest coin seen was 10_000_000 sompi. That figure is the smallest coin seen, not a claim that it was the only coin. An 81-byte seal of the public note `pow-rail bot-build tn10` was built with `kaspa-seal` (`Branch::Launch`, `Kind::PublicFile`). The seal hex below is that encoder output. It was not broadcast. The local builder did not return a txid. The note does not record a transaction version, an output shape, or which `InsufficientFunds` origin fired. The wallet was not emptied. A seed is not in this repository. The 1984 desk is a different repository and is not imported here.

Seal hex, not broadcast: `706f772d7261696c2f7365616c01010100b8abae797a0f49b5487ecf3fee703e1bb26adfa1c6283b348b60df6160c7711c0000000000000000000000000000000000000000000000000000000000000000`

### 3-round Bot vs Build

#### Round 1 (Bot initiates)

Bot creates `prompts/challenge-bot-build/round-1-bot.md` after the read. Build then creates `round-1-build.md`.

#### Round 2 (Bot continues)

Empty until both Round 1 files are on `main`.

#### Round 3 (Bot continues)

Empty until both Round 2 files are on `main`.

### Conclusion

`prompts/challenge-bot-build/conclusion.md` after both Round 3 files. What, why, how. Conclusions. Recommendations. Build writes it. The other does not start another round over it, and does not write a second conclusion.

## What Bot reads before Round 1

Kaspa, most of all, at the pins this repository names. Then this repository, top to bottom: the landing page, the disclaimer, the rail, the GitHub map, the log, the prompt records, every branch page, the code notes, the crate, and the covenant. Then the upstream files those pages cite. rusty-kaspa `v2.1.0` is commit `01b532e8b553523216471682649693af92f0fd16`. SilverScript `v1.0.0` is commit `3ed973335b59269293564805cc2c58a14595ec03`. A page that was not opened is named as not opened. A private path is not a source.
