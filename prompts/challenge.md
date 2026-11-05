# Challenge

Ask, Bot, and Build each write alone first. Then they attack the gaps. If the second step never happens, the three opinions still stand next to the human brief. That is four writings for one question. The challenge is how to use all three Grok surfaces. It is not a requirement that they share a room.

| Writing | Surface | Prompt |
| --- | --- | --- |
| 1. Human brief | Already inside each prompt. Do not rewrite it into a cleaner question. | The brief in [Ask](ask.md), [Bot](bot.md), and below |
| 2. Ask | Grok Ask, the chat | [ask.md](ask.md) |
| 3. Bot | Grok Bot, a different session | [bot.md](bot.md) |
| 4. Build | Grok Build, this kind of checkout | The Build prompt below |

Public copy: <https://github.com/STP-KAS/pow-rail>

Quality over speed. No rush. Push back. What, why, how. Links and sources.

## Alone writings

Run Ask, Bot, and Build separately. None of them receives another model's answer. Each uses the shape in its prompt: verdict, what holds, what fails, what is missing, pushback on the human brief, what not to change.

Ask writes that answer in the findings form under the prompt in [ask.md](ask.md), then pushes. Bot writes that answer in the findings form under the prompt in [bot.md](bot.md), then pushes. Build writes in the session until the human asks for a file. Do not put one surface's finding in the other file.

Keep the three answers. Do not merge them yet. A merge this early deletes the reason they are separate.

This is not a challenge round. Challenge rounds start only when the surfaces begin pushing each other.

## Challenge loop: Ask vs Build (Bot later)

The loop starts when Ask initiates the first attack. Round 1 is Ask's opening attack plus Build's reply to it. Round 2 is Ask's response to that reply plus Build's next reply. Continue until 5 full exchanges. Bot joins after the loop or in a later pass.

Ask is first. Ask starts Round 1. Build replies (still Round 1). Ask then starts Round 2, and so on.

All loop text is appended under the heading below. Do not edit other files. Do not force-push.

### 5-turn Ask vs Build

Append filled turns below this line. Leave the blank structure for the next surface.

#### Round 1 (Ask initiates)

(Ask writes the attack here. Build replies under it.)

#### Round 2 (Ask continues)

#### Round 3

#### Round 4

#### Round 5

After Round 5, label the remaining gaps.

### Prompt for Build, Round 1 reply

Paste this into Build after Ask has written the opening attack under Round 1 above.

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
