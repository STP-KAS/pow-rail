# Build

Grok Build wrote the public repository before either challenge. This page is that record, in the order what, why, how. It does not move the pages, and it does not replace the landing page.

The two challenges are separate. [Ask and Build](challenge-ask-build.md) is closed after three rounds. [Bot and Build](challenge-bot-build.md) has not started. Whether either one later replaces the landing page is open. It is not decided here.

## What

Proof of work is the infrastructure. Miners make the blocks. Developers maintain the software. There is no vendor and no contract. A hash can stay ordered, a covenant can stay closed, and a KAS payment can stay settled after a compelled vendor switches off. A license, a beam, a contactor, and a vehicle command do not.

The sentence on the landing page stays there: it cannot phone a Kaspa office and have the chain switched off. There is no such office. The DealBook line is on that same page, with the two sources for 29 November 2023. This page does not copy them out.

## Why

A government order, a bank, a cloud, or an app store lands on a company, because that switch has an office. The chain has no such office. That is the property under test. It is not a claim that the company keeps operating.

The pages exist so each branch answers the same three questions: the exposure, what is worth examining when nothing is on fire, and what still moves after the company switch. A weak fit stays visible. The detail under each easy page is the code and the file it was checked against.

## How

The pins are rusty-kaspa `v2.1.0`, commit `01b532e8b553523216471682649693af92f0fd16`, and SilverScript `v1.0.0`, commit `3ed973335b59269293564805cc2c58a14595ec03`. The seal is 81 bytes. The file hash is unkeyed BLAKE3 at bytes 17..49. A version-1 id commits to that payload through `PayloadDigest`, domain `b"PayloadDigest"`, in [tx.rs](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L207-L223) and [hashers.rs](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/hashes/src/hashers.rs#L39-L44). The covenant checks length, magic, version byte `0x01`, and that document hash, and it reads only its own input. [seal_escrow.sil](../code/covenant/seal_escrow.sil). It does not check the branch byte or the kind byte. The release still requires the arbiter's signature.

These commits are Build's, on `main`, and they stop before Ask's first finding. Ask's finding is `321de41`, in [ask.md](ask.md).

| Commit | What Build put in the repository |
| --- | --- |
| [`7146de3`](https://github.com/STP-KAS/pow-rail/commit/7146de3) | Landing page, disclaimer, rail, GitHub map, fifteen branch pages, license. |
| [`d47050e`](https://github.com/STP-KAS/pow-rail/commit/d47050e) | The detail under each branch, the payload digest, the 81-byte seal, and the covenant. |
| [`068dcd4`](https://github.com/STP-KAS/pow-rail/commit/068dcd4) | The Ask, Bot, and challenge prompts. |
| [`089819a`](https://github.com/STP-KAS/pow-rail/commit/089819a) | The sharper office paragraph, the DealBook line, and [LOG.md](../LOG.md). |
| [`ef75596`](https://github.com/STP-KAS/pow-rail/commit/ef75596) | The findings forms. They were left blank. |

The path of those passes is the log. Entries there are kept.

## Pages

| Page | Role |
| --- | --- |
| [README.md](../README.md) | Landing page. |
| [DISCLAIMER.md](../DISCLAIMER.md) | Experimental. Not advice. Not Kaspa core. |
| [RAILS.md](../RAILS.md) | What the rail is, and what it will not do. |
| [GITHUBS.md](../GITHUBS.md) | Company code and lookalike orgs, as listed on 5 November 2026. |
| [LOG.md](../LOG.md) | The path. Ideas are the author's. Files, code, and citations are Build's. |
| [MAP.md](MAP.md) | The map of these records. |
| [branches/](../branches/01-launch.md) | Fifteen branches. Easy half, then Detail. |
| [code/README.md](../code/README.md) | The mechanism notes, checked against the v2.1.0 tag. |
| [code/kaspa-seal](../code/kaspa-seal/Cargo.toml) | The digest, the lane id, and the 81-byte seal. |
| [code/covenant](../code/covenant/seal_escrow.sil) | `SealEscrow`. |

The branch files are `01-launch`, `02-starlink`, `03-starshield`, `04-starmind`, `05-starbase`, `06-terafab`, `07-xai`, `08-x`, `09-vehicles`, `10-energy`, `11-optimus`, `12-charging`, `13-supply-chain`, `14-compute`, `15-adjacent`.

## What this page does not do

It does not open either challenge. It does not edit a branch page. It does not put a seed, a key, or a private path in the repository.
