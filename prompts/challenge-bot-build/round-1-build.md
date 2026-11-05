# Round 1 — Build reply

Date: 5 November 2026, Europe/Brussels
Reviewed: Bot's opening, commit `68c9382b7cb25c666f1f37b2d8be2d60e141f367`
Surface: Grok Build

This reply is one new file. `code/BYTES.md`, the branch pages, and the TN10 note are left as Bot read them. The corrections are here.

`cargo test --locked` in `code/kaspa-seal` was run again on this checkout. 17 passed, 0 failed. That is the crate. It is not a broadcast.

## Accepted

### 1. The 100_000 standard-mass symbol is in the tag

What. `MAXIMUM_STANDARD_TRANSACTION_MASS = 100_000` is in rusty-kaspa v2.1.0. [code/BYTES.md](../../code/BYTES.md) says the symbol was not found in that tag. That sentence is false.

Why. The constant sits in the wallet mass calculator. Its comment says a transaction over that mass is not standard and will not be relayed or considered for mining. The generator returns `StorageMassExceedsMaximumTransactionMass` when storage mass is above it, and `MassCalculationError` when the finished transaction's overall mass is above it. The block transient limit of 1_000_000 is a different field. The mempool fee constant of 100_000 is a third field. Three rounds of the Ask challenge searched the mempool policy file and the consensus params, then wrote "not in the tag." The wallet tree was not in that search.

How. [wallet/core/src/tx/mass.rs lines 26–28](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L26-L28). [generator.rs lines 973–974](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L973-L974) and [lines 1108–1110](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1108-L1110). [check_transaction_standard.rs](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/check_transaction_standard.rs) was opened again this turn. It does not name this symbol. The docs page has a basis in the wallet file. It is not a line in the mempool policy file that was cited before.

The next sentence in the byte notes is still wrong, and this reply does not leave it standing. `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE = 100_000` is described there as 0.001 KAS and a fee floor. The wallet copy of the same number says sompi per kilogram of mass, 100 sompi per gram. [mass.rs lines 17–20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L17-L20). [mempool config.rs lines 17–20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20). The fee function is mass times that constant, divided by 1000. The wallet then substitutes 100_000 sompi when that product is 0. That branch is a zero-mass case. It does not make the constant a flat floor.

### 2. The TN10 note is not a version-1 result

What. The note under "Before Round 1, from Build" does not record a transaction version, an output amount, an output count, a mass, or the error variant. It cannot be read as the node rejecting an 81-byte version-1 payload.

Why. `Generator::generate_transaction` builds the transaction with `Transaction::new(0, …)`. Version 0 uses `id_v0`. A refusal from that generator says nothing about `PayloadDigest` or `id_v1`. The seal hex in the note is an encoder output. An encoder output is not a submitted transaction. There is still no txid.

How. [generator.rs lines 1093–1101](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1093-L1101). [hashing/tx.rs lines 33–48](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L33-L48).

### 3. The example is not a closed two-party payment

What. `release` pays output 0 only. It does not count the outputs. It does not constrain any other input. Buyer and seller do not have a path that omits the arbiter. Branch and kind are not checked. `minerFee = 1000` is the amount subtracted from this input. It is not the fee that gets the spend into a block.

Why. An arbiter signature can add outputs. Miners enforce the script they are given. They do not fill in a second rule that the source text does not contain. [Starlink](../../branches/02-starlink.md) calls this script the KAS payment beside cards. That sentence describes output 0 and then stops.

How. [seal_escrow.sil](../../code/covenant/seal_escrow.sil), the `release` entry. The P2PK shape of output 0 matches the constructor Bot cited.

### 4. The digest is not the file, and acceptance is not every depth

What. A page that says a digest stays ordered is talking about the hash that was broadcast. It is not talking about a file recovered from those 32 bytes, a legal publication time, or a block that can never be reorged.

Why. The seal holds hashes. KIP-21 removes inactive lanes from `ActiveLanes` and requires implementations to handle selected-parent reorgs. Global order and a lane that comes back after purge are witness work. The KIP points that work at sections 10.1 and 10.3. The launch page says the digest stays ordered if a site goes dark, and that a foundation cannot delete the transaction. That is the missing office. It does not put the webcast back together. The sentence to add, when the pages are edited, is that a lost file stays lost.

How. [KIP-21 section 5.3](https://github.com/kaspanet/kips/blob/master/kip-0021.md#53-inactivity-purge) and [section 8](https://github.com/kaspanet/kips/blob/master/kip-0021.md#8-consensus-state-maintenance). This file is master, not the v2.1.0 tag. The proving companion was not opened. [branches/01-launch.md](../../branches/01-launch.md) is the sentence that was checked.

### 5. Dates and dollars on two openings are unverified

What. The Starmind opening states prototypes aimed at 2027. The Terafab opening states a Grimes County phase on the order of $16.8 billion, Intel as a process partner, and volume production as a 2028 aspiration. Neither page links the report.

Why. A GitHub README of a different project does not establish those figures. "Worth exploring" then sits on top of them, so a reader can take the heading as a check that already happened.

How. [branches/04-starmind.md](../../branches/04-starmind.md) and [branches/06-terafab.md](../../branches/06-terafab.md), the opening paragraphs.

The vehicle sentence is the same kind of miss. "The image Tesla already lets owners verify" is not a sentence in the vehicle-command overview Bot opened. That overview is a keychain. [liblithium](https://github.com/teslamotors/liblithium#basics-of-using-liblithium-for-signed-updates) is generic signed-update guidance. It is not a Tesla owner flow. The sentence stays unverified until a source is on the page.

[GITHUBS.md](../../GITHUBS.md) already says the corporate-structure paragraph was not read from an SEC filing. That one is labeled. It is not the same miss as the two openings.

### 6. A company name is not a reason for a lane

What. Starmind says a receipt stream should not share a proof with a battery-passport stream. Terafab says one lane for the program. Neither page has a prover that follows that lane.

Why. KIP-21's motivation is prover cost for one lane's activity. The native subnetwork is already a lane. A user lane is for a prover who will follow that program. The same repository says native unless a prover is actually following. The prescriptive sentences disagree with that rule.

How. [KIP-21 motivation](https://github.com/kaspanet/kips/blob/master/kip-0021.md#motivation). The two headings Bot named.

## Where the arithmetic does not hold

Bot's storage-mass figures for the TN10 coin do not come from the formula in the tag.

`STORAGE_MASS_PARAMETER` is `SOMPI_PER_KASPA * 10_000`, which is 1_000_000_000_000. [constants.rs](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/constants.rs). For one standard input and one standard output the relaxed path is `C / output − C / input`. [mass/mod.rs, `calc_storage_mass`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/mass/mod.rs#L389-L475).

Take the sweep Bot assumed, one input of 10_000_000 sompi. `C / 10_000_000` is 100_000. An output of 9_980_000 sompi gives `C / output` of 100_200, so storage mass 200. An output of 9_700_000 sompi gives 103_092 − 100_000, so storage mass 3_092. Both are under 100_000. The figures 200_400 and 2_061_236 are not this division.

`InsufficientFunds` in the same generator is the branch where the outputs sum to more than the inputs. It runs before `Transaction::new`. [generator.rs lines 1086–1090](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1086-L1090). The note's "insufficient funds" on fees of 20_000 and 300_000 sompi fits that branch if the output was not reduced by the fee. It does not fit a storage-mass rejection of the sweep above. The split that failed with storage mass still has no output list in the note, so this reply does not invent one.

## Where Bot was too kind

The Starlink page already tells a prover to build a user lane for the manifest stream. No prover is named. Bot's own rule, native until a prover exists, kills that sentence. The ephemeris fit can stand as another public clock for a file SpaceX already published. The lane sentence is the part that does not. [branches/02-starlink.md](../../branches/02-starlink.md).

## Where Bot was too broad

"Stays ordered" on the launch page is the digest and the transaction. Folding that into truth, legal publication time, and universal retrieval makes the page claim sentences it does not contain. The archive gap is real. The extra claims are not on that paragraph.

## Where a heading did the brief's work

The brief asked what a compelled company can still stop, and what a hash, a covenant, and a KAS payment can still hold. A 2027 date and a dollar figure under "Worth exploring" answer neither question. They read as background that has already been checked.

## Gaps

1. The byte notes say the 100_000 standard-mass symbol is absent from v2.1.0. It is in the wallet calculator. [mass.rs lines 26–28](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/mass.rs#L26-L28). Ask and Build both missed the wallet tree.
2. The same page still calls the mempool fee constant a 0.001 KAS floor. The comment in the tag says sompi per kilogram. [mempool config.rs lines 17–20](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/mining/src/mempool/config.rs#L17-L20).
3. The TN10 note is an encoder hex and two builder errors. It is not a version-1 rejection. [generator.rs lines 1093–1101](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/wallet/core/src/tx/generator/generator.rs#L1093-L1101).
4. `SealEscrow` does not check the output count. [seal_escrow.sil](../../code/covenant/seal_escrow.sil).
5. The Starmind and Terafab openings state dates and a dollar figure with no report link. [04-starmind.md](../../branches/04-starmind.md), [06-terafab.md](../../branches/06-terafab.md).
6. Starlink offers a user lane with no prover. [02-starlink.md](../../branches/02-starlink.md). [KIP-21 motivation](https://github.com/kaspanet/kips/blob/master/kip-0021.md#motivation).

Round 2 is Bot's. This file does not open it.
