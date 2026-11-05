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

## Round 1, alone

Run Ask, Bot, and Build separately. None of them receives another model's answer. Each uses the shape in its prompt: verdict, what holds, what fails, what is missing, pushback on the human brief, what not to change.

Ask writes that answer in the findings form under the prompt in [ask.md](ask.md), then pushes. Bot writes that answer in the findings form under the prompt in [bot.md](bot.md), then pushes. Build writes in the session until the human asks for a file. Do not put one surface's finding in the other file.

Keep the three answers. Do not merge them in this round. A merge this early deletes the reason they are separate.

## Round 2, the attack

Give each surface the other two answers. Ask's answer is the latest finding appended in [ask.md](ask.md). Bot's answer is the latest finding appended in [bot.md](bot.md). Then this instruction:

```text
You have two reviews you did not write. Find the gaps they left and the gaps they invented.

Quality over speed. No rush. Do not score style. Attack claims.

For each review, separate:
- A point you re-opened and now accept, with the link that changed your mind.
- A point that has no source, or the wrong tag, or a link that does not say what they claim.
- A fit they were too kind to. Name the company system that still has an off switch they treated as solved.
- A fit they were too harsh to. Name the byte, the opcode, or the KIP that already does the job.
- A place they smoothed the human brief into a different question.

What, why, how. If you did not open the page, you do not have a hit.

Do not propose sanctions evasion, a beam bypass, a vehicle jailbreak, or a new thing to put on-chain. Do not post, spend, or ask for a seed.

End with a gap list. A gap is one sentence, one link, and the surface that missed it. No gap without a source.
```

## Round 3, Build answers the gaps

Build wrote the repository, so Build is the biased surface. Build does not get the last word by default. Build gets the job of answering with the files.

```text
You are Grok Build. You have the gap list from Ask and from Bot. You also have your own Round 1 opinion. The repository is https://github.com/STP-KAS/pow-rail and the checkout you can read.

Quality over speed. No rush. You are pushing back on your own earlier writing as well as on theirs.

For each gap:
- What the gap says.
- Why it is real or not, from a file at the pinned tag. rusty-kaspa v2.1.0 is commit 01b532e8b553523216471682649693af92f0fd16. SilverScript v1.0.0 is commit 3ed973335b59269293564805cc2c58a14595ec03. A KIP status is whatever the KIP file says on the day you read it.
- How you would change the repository, or why you would not. A change is a specific file. "Clarify" is not a change.

Do not edit yet. The human decides which gaps become a commit. Do not push, post, spend, or broadcast while you answer.

Where the code and a branch page disagree, the cited upstream file wins, then the code, then the page. The page is the defect until the upstream file is wrong.

Where Ask and Bot disagree with each other, do not average them. Pick the one whose link you opened, or say both links fail.

The human brief stays in force while you answer. Branches, easy page then code, sources on every mechanism, nothing for sale, no political essay. The author's path is worry, then the question, then code, then this attack. That path did some sorting. It is not a shield. If the path skipped a bad fit, the gap stands.
```

## If they never meet

Stop after Round 1. Label the four writings:

1. Human brief.
2. Ask.
3. Bot.
4. Build.

A later reader can diff them without a new model in the room. The diff is the point of running all three. One surface, even this one, is one bias.

## Build, round 1 prompt

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
