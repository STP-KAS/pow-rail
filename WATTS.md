# Space watts

On 11 November 2026 Elon Musk replied to a post about solar capacity on Earth. The reply is [this post](https://x.com/elonmusk/status/2120613240707420280). The words are his:

> 100GW in space is roughly equal to 800GW on Earth.
>
> Moreover, solar on Earth must be paired with massive batteries to continue powering compute at night and supplemented with combustion or nuclear power during winter.
>
> These are not the same at all.

The parent post stated that China installed 315 GW of solar in 2025. This page does not remeasure that figure, and it does not remeasure the eight-to-one ratio. Both sentences are filed as what was written. No company named in this repository is claimed to have adopted either sentence as an engineering baseline.

The question asked of the study is whether a Kaspa feature is interesting on any branch, once that post is in front of it. The lenses are financial, security, transparency, efficiency, durability, nodes in space, and miners in space.

## What is live, and what a watt is not

The live pieces are the ones [the rail](RAILS.md) already lists. Anyone who pays the fee can broadcast a valid transaction. KAS has no issuer who mints it or freezes coins that never entered a script. The chain runs while miners mine. A seal in this repository is 81 bytes. A default node can show the transaction inside the pruning window, 108_000 seconds in rusty-kaspa v2.1.0, and not by that call after it. The relay fee is 100 sompi per gram. The example covenant trusts an arbiter and can leave the coins that entered it frozen. KIP-16 checks two proof tags. It is not a general computer.

None of those move a watt, store a night, or reject heat in vacuum. The post is about energy. The rail is about order and about KAS.

## The seven lenses

**Financial.** A KAS payment is interesting only where both sides want KAS. It does not finance a gigawatt, pay a utility, or replace a battery contract. [Kaspa Dollar](DOLLAR.md) is still a draft, and a vault of KAS is the wrong instrument for a wholesale power market. The fee is a mass rate, not a cut of the energy.

**Security.** The chain orders a hash and enforces a script on coins that entered it. It does not authenticate a satellite command, a grid dispatch, or a battery contactor. Those stay on the channels that already carry them. A key that can freeze coins inside one script is not a grid operator.

**Transparency.** If a figure is already fit to publish, the 81-byte seal orders its hash. The file stays off the chain. A later edit of a web page does not inherit the old digest. The two sentences above can be sealed as a public note. Sealing them does not make the ratio true. Readers who are not handed the file learn only the hash.

**Efficiency.** The post's efficiency claim is physical: sun in orbit does not stop at night, and winter storage is a ground problem. Kaspa does not change a capacity factor. Ten blocks a second do not move a watt. A national meter stream does not fit in a block. An interval hash the parties already agreed to publish does.

**Durability.** While miners mine, a sealed hash stays ordered, with the pruning limit already stated. There is no office that can phone the chain off. There is also no office that can keep an array powered. Durability of a record is not durability of a satellite, a battery, or a training run.

**Nodes in space.** Anyone can run a node. Orbit does not change the consensus rules. A node in space still needs a radio, a license, and a computer that survives the environment. It verifies. It does not generate the 100 GW. This repository has no orbital node, and nothing in it was flown.

**Miners in space.** A fleet of miners in orbit is not a feature this study recommends. The chain already buys its security on the ground, and one pool can delay a transaction without stopping the chain. This page does not print a pool share. One node and one small miner, riding on a vehicle that is already going, is a different question. It is the next section.

## One node and one miner, along for the ride

The question is not a second Kaspa network in orbit, and it is not extra hashrate. It is whether one verifying node and one small miner, carried on hardware that is already being launched for Starmind, Starlink, or a Starship flight, would be useful to the people who fly that hardware. Nothing in this repository is for sale. SpaceX is not asked to fly it. This is a reading of the fit.

### What "does not weigh much" actually is

A Kaspa full node, in the Toccata guide shipped with rusty-kaspa, wants at least 8 CPU cores, 16 GB of memory, a 640 GB SSD, and on the order of 80 Mbit/s. The preferred box for a pool operator is larger. [That guide](https://github.com/kaspanet/rusty-kaspa/blob/master/docs/toccata-guide.md). The study's consensus pin stays rusty-kaspa v2.1.0. The guide is operational, and master can move.

That computer is kilograms, not grams. Next to a ship it is a small box. On a booster's mass budget it is still a line item, not a free sticker. It also wants a steady broadband path. A LEO terminal that is already up is the kind of path that can carry 80 Mbit/s. A deep-space radio usually is not, so a lunar or Martian node is a delayed copy, not a live peer.

A miner is a second object. Public maker-spec tables list small kHeavyHash units around 100 W and a few hundred GH/s, and current large units around 3 kW and tens of TH/s. One compilation is [this table](https://cloudminecrypto.com/learn/kaspa-mining), read 11 November 2026. This study did not open the manufacturer sheets. A third-party estimate the same day put the network near 341 PH/s ([MillionMiner](https://millionminer.com/kaspa-network-hashrate)). That figure moves and it is not a consensus measurement. Against a network of that order, a 400 GH/s unit is about one millionth of the hashrate. It does not secure the chain, and it does not need to. The large unit is a different payload: kilowatts, a real radiator, and still a rounding error beside the ground network.

### Heat

Vacuum does not blow the heat away. A surface rejects heat by radiation. Sunlight is the resource the post was about. The waste heat still needs a radiator that sees cold space, or a share of a radiator the vehicle already has.

A desk estimate, not a vehicle thermal analysis: a black surface near room temperature radiates on the order of a few hundred watts per square metre into cold space from one side. A 100 W load is then a fraction of a square metre, if the sun is not shining on that face. A 3 kW load is several square metres and a power allocation. [Starmind](branches/04-starmind.md) already names heat as a critical-path problem for orbital compute. That is the goal the passenger fits, in the small sense. A steady 100 W that turns electricity into heat is a calibrated load on a radiator the mission is already trying to prove. It is not the compute product. Inference, or whatever the customer bought, is a better use of a large radiator than hashing. The small miner is the instrument. The fleet is the mistake.

The chain does not know whether the watt came from a panel in sunlight or from a ground socket. Flying the miner does not make the eight-to-one sentence true.

### How late the radio is

Kaspa orders about ten blocks a second. One interval is a tenth of a second.

In low orbit the vacuum delay is a few milliseconds. The path through a relay is longer. Either way it is a slice of one block interval, not a stack of blocks. A LEO node can be a normal peer. A LEO miner can propose a block that ground miners are also proposing. GHOSTDAG keeps more than one block. Arriving late makes a block easier to leave out of the rewarded order. This page does not compute that penalty. One millionth of the hashrate means the machine will rarely find a block at all. A block it does find is a receipt that this computer, on this power bus, hashed. It is not a security contribution.

The Moon is a bit over a second of radio delay each way, on the order of ten block intervals. A miner there works on a header the ground has already moved past. Its block can still sit in the DAG when it arrives. It is a poor way to earn a reward, and it does not extend the Earth's chain in any way the ground miners need. A node on the Moon can hold a copy and check a seal locally, seconds behind the link. Covenants verify on the blocks it has. They do not confirm faster than the link.

Mars is minutes. That is thousands of blocks. A miner on the far side of that delay builds a private history. When the link returns, the heavier chain is the one Earth has been mining. The partitioned blocks lose. Do not mine across that gap. A node that only stores what the link delivers is a cache of Earth's chain, useful while the link is up, stale when it is not.

### Where the box would sit

**Starlink.** The constellation is already a radio network with a path to the ground internet. A node behind a terminal that is already flying is the best of these cases: broadband, short delay, no new spectrum. The node lets that vehicle check a recent seal without trusting a ground RPC. Starlink's own account, beam, and geofence stay SpaceX's. The node does not allocate a cell. A Starlink satellite is not a Kaspa node because it is in space. It becomes one only if someone runs the software on a computer that can keep the 640 GB and the bandwidth.

**Starmind.** Nothing is flying under this study's evidence. If an orbital computer is up, and it already has power, a radiator, and a link, a verifying node is a passenger on that bus. It can check a job-receipt seal when the ground gateway is the thing that failed. The receipt is still a transaction someone pays for on the public chain. The node does not run the customer's job. A 100 W miner on the same bus is the thermal instrument above. It should not be described as the workload.

**Starship, a flight that stays up.** A ship on orbit long enough to be a peer can carry the pre-synced node and use whatever broadband it already has. Sync from empty during a flight is the wrong plan. The guide's disk is the chain's recent history, and the pruning window is 108_000 seconds. A node that is partitioned longer than that, and then asked for an old proof, hits the same `BlockTooDeep` limit as a ground node. The file is still not in the 81 bytes.

**A booster, or any flight of minutes.** The computer cannot sync Kaspa in that window, and a miner cannot matter in it. The useful object on a short flight is the 81-byte seal made on the ground before launch and broadcast by a ground station. Carrying the box up and down adds a mass line and a thermal line for a chain that learned nothing. Ship the seal. Leave the miner.

**A station.** A long-lived pressurized or platform computer can run the node if the operators accept the power, the heat, and the safety process. Those are the station's switches. The chain does not bypass them. This page does not name a station payload that is already approved.

**The Moon, and anything past it.** Carry the node if the crew or the base wants to check a published hash without a ground RPC, and if the link can feed it often enough to be worth the disk. Expect the copy to be late. Do not carry the miner for hashrate. A covenant spent on Earth becomes visible there when the blocks arrive, not before.

### What the software would actually do out there

**The node** verifies the same rules as a ground node. That is the useful part. A published telemetry hash, a job receipt, or a meter row can be checked on the vehicle, from blocks the vehicle itself has, for as long as the radio has been feeding it.

**The miner** turns power into heat and, rarely, into a block. Keep it if the thermal test is the point. Drop it if the only hope was security or income.

**SilverScript and the covenant.** [`seal_escrow.sil`](code/covenant/seal_escrow.sil) already locks KAS against an arbiter signature and a document hash. A mission note that is already fit to publish can be that document. The script runs on every full node, including one in space, once the spending transaction is in a block that node has. Flying the node does not change the script. The script still has no timeout and no refund. If the arbiter never signs, the coins that entered it stay there. Do not put a launch rule, a beam command, or a contactor in the script. The example pays a key. It does not fly the ship.

**Argent.** [argent-lang/argent](https://github.com/argent-lang/argent) compiles a higher-level covenant language down to SilverScript. Its releases page showed no release on this pass. It is not what flies. An expert can still read the `.sil` it emits, the same narrow path this repository already uses for the escrow. A space node verifies that script. It does not compile it.

**vProgs.** [kaspanet/vprogs](https://github.com/kaspanet/vprogs) also showed no release on this pass. The pin on [the rail](RAILS.md) stays master `f9b84a863a`, 28 July 2026. If a tag lands, a node can check a proof that a job moved from one state to the next, and a space node can check it after the block arrives. The prover can stay on the ground or on the orbital computer. The Kaspa node is the checker. Until the tag, the 81-byte hash is the piece that exists. KIP-16 today checks two proof tags. It is not that prover.

**The dollar draft.** [DOLLAR.md](DOLLAR.md) stays a design, not a coin. A space node would verify the same vault transactions a ground node verifies, once the blocks arrive. The price votes are coinbase payloads from miners, almost all of them on the ground. A partitioned node does not mint, does not redeem, and does not keep a fresh median. If the draft one day matches its own page, a customer and a provider can settle a published data product in that unit wherever they can read the chain. The spacecraft does not become the issuer. No one is asked to send KAS.

### What would actually help SpaceX

Three uses survive. None of them is a product this repository sells, and none of them is a claim that the company wants them.

A node on a vehicle that already has broadband can check a seal from its own copy of the chain. That helps when the thing that failed is a ground website or a ground RPC, and the radio to the vehicle still works. It does not help when the radio is the thing that failed.

A 100 W miner on a bus whose owners are already trying to power a load and reject the heat is a small, steady, well-described instrument. The rare block is a souvenir of that test. The hashrate is not the point.

A covenant on the ground can hold KAS until a published mission hash matches, between parties who want KAS. The ship confirms the spend if it is running a node and the block has arrived. The flight computer does not wait on that confirmation. The dollar draft, if it ever exists, is the same shape in a unit that aims at a dollar. It is still KAS underneath, and it is still not live.

## Every branch

Each line uses only the live pieces above. The branch page stays the longer reading of that company.

**Launch.** Interesting as a seal of a public note made before flight. A ship that stays on orbit can carry the pre-synced node. A booster flight of minutes cannot. The license, the range, and the vehicle stay with the company and the state. A rocket is not a node unless the computer and the radio are actually on it.

**Starlink.** The interesting passenger is a node behind a terminal that is already up. Short delay, enough bandwidth, no new spectrum. The radio, the account, and the cell stay SpaceX's. A Starlink satellite is not a Kaspa node because it is in space.

**Starshield.** No lens of this post supports the branch. The customer is a state that wants a known operator who can be directed. A chain with no office is the product that customer is not buying. Tasking stays off the rail.

**Starmind.** This is the branch the post sits next to, and nothing is flying under this page's evidence. Power and heat are the critical path. A verifying node is a passenger on that bus. A 100 W miner is a thermal instrument, not the customer's job. Kaspa does not supply the watts. A published power budget can still be sealed. A job receipt in KAS is interesting only if both sides want KAS.

**Starbase and the factories.** Interesting as a seal of an inspection record the parties already keep, if the hardware for either architecture is what the factory is building. The chain does not weld, clear a crate, or test a radiator. Factory efficiency is not a block rate.

**Terafab.** Interesting as a seal of a public acceptance record. Recipes, yields, and tool logs stay off the chain. The eight-to-one sentence is not a process recipe. The fab figures already on that page are not treated as verified, and they are the wrong size for a KAS vault.

**xAI.** The night and winter power in the post is the ground-cluster problem. Interesting as a seal of a published system card or a published power statement. The API account remains a company switch. The cluster does not keep training because a hash was ordered. Prompts stay off the chain.

**X.** The post is already a public sentence on X. Sealing that text orders a copy of a sentence X already showed. It does not move a watt. X Money is a balance at a bank. It does not finance an array. Speech on X does not need this rail in order to be quoted.

**Vehicles.** No power plant sits in this branch. A published fleet energy figure can be sealed if it is already public. The car still trusts the key in the vehicle. A block does not steer, and it does not store the kilowatt-hour the post says the night requires.

**Energy.** This is the ground half of the post: solar that needs batteries, and a winter that needs another plant. The interesting feature is the one the energy page already has, a seal of an interval row the parties already store. The seal does not push a megawatt and does not replace the battery. Dispatch stays on the utility channel. An ISO still clears in the ISO's unit. A KAS side agreement stays a side agreement between parties who want KAS.

**Optimus.** No lens of this post adds a fit. A robot does not generate 100 GW. Safety logs and video stay off the chain. A published spec hash is the same narrow seal as on any other page, and this post is not that spec.

**Charging.** Interesting as a seal of a session receipt the driver and the site already have. The contactor stays in the stall. Night charging is the battery problem the post names, and the rail does not store the kilowatt-hour. Space solar does not close the contactor.

**Supply chain.** The batteries in the post are cells, metals, and a record that has to survive the vendor in the middle. Interesting as a seal of a packing list or a passport extract the parties are already allowed to show. The seal does not make the cell, and it does not close the winter gap. A sanction is still not a missing opcode.

**Compute.** "Powering compute at night" is this branch. Interesting as a seal of a published system card or a published power claim. The training run, the fabric, and the power contract stay in the building. Gradients do not go on the chain. Space solar for compute is the Starmind problem, a different building, not a Kaspa efficiency feature. Durability of an eval file is not durability of the next run.

**Neuralink and The Boring Company.** No feature of the rail supports this post here. Neuralink stays off the chain because the material is medical. A tunnel's inspection pack can be sealed for the reasons that page already gives, which are not space watts. The map includes both so the rail is not forced onto work it would harm.

## What this page does not do

It does not recommend a fleet of miners in orbit. One passenger node, and one small miner used as a heat load, are explored above and were not flown. It does not ask anyone to send KAS. It does not treat the post as a proof that 100 GW in space equals 800 GW on Earth. It does not move a license, a beam, a contactor, or a power contract.
