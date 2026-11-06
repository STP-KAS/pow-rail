# The rail

Proof of work. No office. The name of the network is in the table below, because the table cites files. The property under test is the missing office.

Three properties matter more than any feature list.

**Permissionless.** Anyone who pays the fee can broadcast a valid transaction. No account at Kaspa exists to approve or to close.

**No issuer.** KAS was fair-launched in November 2021. No company mints it, freezes it, or redeems it. A covenant can lock KAS under script rules. The script is enforced by miners, not by a compliance desk. A covenant with no refund path can freeze the coins that entered it, under the key the script requires. That key is not an issuer of KAS. It cannot mint the coin, and it cannot freeze coins that never entered the script.

**One off switch.** The chain runs while miners mine. Stopping it means a majority of the hashrate stopping, or the internet paths to those miners failing. Subpoenaing a foundation does not halt the blocks. A pool can leave a transaction out of the blocks that pool builds. Another miner can still include it. That delay is not the same event as the chain stopping. This page does not print a pool share. Halting the chain is the government-resistance claim. Filtering is a separate limit on how soon a transaction is included. The companies themselves can still be compelled, even while the chain continues.

## What is live

Checked against public sources on 5 November 2026. Details and the method are in the pin encyclopedia [STP-KAS/kaspa-master-file](https://github.com/STP-KAS/kaspa-master-file). This page is not that board.

| Piece | Live fact |
| --- | --- |
| Consensus | GHOSTDAG, about 10 blocks a second since Crescendo. [KIP-2](https://github.com/kaspanet/kips/blob/master/kip-0002.md) (DAGKnight) is **Proposed**. |
| Node | [rusty-kaspa v2.1.0](https://github.com/kaspanet/rusty-kaspa/releases/tag/v2.1.0), published 22 September 2026. A public API node that day-range reported `serverVersion` 2.1.0 and a block reward of 1.94454365 KAS. |
| Order | [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) is **Active**. One header commitment over application lanes, plus miner payloads. A prover for one lane follows that lane's activity. |
| Covenants | Toccata, from DAA score 474165565, as stated on [docs.kaspa.org/toccata](https://docs.kaspa.org/toccata). That score is not a literal in the rusty-kaspa v2.1.0 Rust tree. Scripts can lock KAS and check state on their own continuation. |
| Proof check | [KIP-16](https://github.com/kaspanet/kips/blob/master/kip-0016.md) is **Active**. Opcode `OpZkPrecompile` (`0xa6`) checks a **predefined** proof tag. It is not a general computer. |
| Compiler | [SilverScript v1.0.0](https://github.com/kaspanet/silverscript/releases/tag/v1.0.0). Master has moved past the tag. The tag is the pin. |
| Token layout | [KCC-1](https://github.com/kaspanet/kccs/blob/main/kcc-0001.md), [KCC-2](https://github.com/kaspanet/kccs/blob/main/kcc-0002.md), and [KCC-20](https://github.com/kaspanet/kccs/blob/main/kcc-0020.md) say **Final**. A Final convention is a shared layout. It is not a dollar. |
| Money on the rail | **KAS.** It moves. It is not a peg. |

Anyone can run a node from [kaspanet/rusty-kaspa](https://github.com/kaspanet/rusty-kaspa). Anyone can read a header. Anyone can broadcast. SpaceX and Tesla do not have to run infrastructure for someone else to seal a hash of a public document.

## What a builder can do today

**Seal a hash.** Put a digest in a transaction or a miner payload. KIP-21 commits accepted transactions by subnetwork. A default node can show that transaction while the block is still at or after its pruning point. `PRUNING_DURATION` in rusty-kaspa v2.1.0 is 108_000 seconds. `FINALITY_DURATION` is 43_200 seconds. They are different constants. After the pruning point, `get_seq_commit_lane_proof` returns `BlockTooDeep` ([consensus/mod.rs lines 1519–1524](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/src/consensus/mod.rs#L1519-L1524), [`config/constants.rs` lines 69–73](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L69-L73)). Showing the transaction then takes a copy that was kept, with a lane witness fetched in time, or a node that kept the history. The file the hash was taken from is not in the 81 bytes. If every copy of the file is gone, the hash does not restore it.

**Lock KAS.** A covenant holds coins until the script's own condition is met. A script can require a timeout, a signature, or a published hash. The example in this repository, [`seal_escrow.sil`](code/covenant/seal_escrow.sil), requires an arbiter signature and a document hash. It does not require a timeout. The amount has to be required in the script. The compiler does not lock the amount by itself.

**Pay KAS.** Two parties who both want KAS can settle without an acquirer. Fees are KAS too.

**Check a precompile proof.** If the statement matches a tag the node actually implements, miners verify it. This pass did not treat "any proof" as live. Read the opcode before designing a Tesla or SpaceX proof around it.

## What is not live

Do not tell an operator these are switches they can flip.

| Name | State on this date |
| --- | --- |
| DAGKnight | KIP-2 Proposed. Branch `dagknight` tip `ad45e241`, 8 September 2026. Not in the v2.1.0 release. |
| vProgs | [kaspanet/vprogs](https://github.com/kaspanet/vprogs) master `f9b84a863a`, 28 July 2026. No release, no tag, dozens of open pulls. |
| Kaspa Dollar | A published design and a fundraiser, not a coin and not a KIP. Mechanism: [how it works](https://kaspadollar.com/how-it-works), read 6 November 2026. That page says the whitepaper, with proofs, is still to come before the fundraiser ends. The [roadmap](https://kaspadollar.com/docs) names Alexander Safstrom and aims at a May 2027 testnet and a September 2027 mainnet. Code was not in this repository's pins. This page does not ask anyone to send KAS. |
| 100 blocks a second | Not an activation. |

If DAGKnight lands, clients can accept a seal sooner, tracking real delay instead of a fixed parameter. If vProgs lands, a lane can anchor a proved state machine, not only a hash. If a dollar with no issuer ever lands, the unit in a covenant can aim at one dollar. The draft of that unit is [DOLLAR.md](DOLLAR.md). None of that changes the off switch. The off switch remains the miners. None of that flies a rocket, energizes a coil, or turns a phased array.

## What the rail will not do

- It will not keep a launch license, a spectrum grant, or a factory permit.
- It will not override an order served on SpaceX or Tesla. The company can still be forced to stop a service it operates.
- It will not hide data. A public chain is a poor place for export-controlled designs, customer prompts, neural recordings, and vehicle traces. Seal a hash. Keep the file elsewhere.
- It will not clear a national power market, route Starlink, or authenticate a command the car must execute in milliseconds. Those systems already have a faster, private mechanism. See [teslamotors/vehicle-command](https://github.com/teslamotors/vehicle-command) and [teslamotors/ttpoe](https://github.com/teslamotors/ttpoe).

The encoder, the vector tests, and the covenant are in [the code notes](code/BYTES.md).
