# Kaspa Dollar, a draft

This is a reading of a design that is published and not yet a coin. The [how it works](https://kaspadollar.com/how-it-works) page and the [roadmap](https://kaspadollar.com/docs) were opened on 6 November 2026. The how-it-works page says the proofs will be in a whitepaper before the fundraiser ends. That paper was not on the site. The page also says the constants may change, and that KSD is a reference label, not the final ticker. Code for the vault was not in this repository's pins. This page does not ask anyone to send KAS.

The driver of the study is Nakamoto consensus. Miners order the blocks. It cannot phone a Kaspa office and have the chain switched off. There is no such office. The author called that cyberpunk, in that plain sense: proof of work, no company key that can freeze the unit after it has moved. A bank stablecoin fails that test because the issuer can freeze the token. Raw KAS passes it, and the unit moves with the coin. The draft is the attempt to keep the test and still name the unit as one dollar.

Ask and Bot did not review this page. The sentences they left standing are in [From here](FROM-HERE.md). Those sentences still limit the seal, the fee, and this repository's own covenant. They are not a review of this design.

## What the draft is

It is a lending position, not a dollar sitting in a bank. A vault locks KAS on the base layer. To mint, the locked KAS has to be worth at least 2.5 times the units minted. From that line, the price of KAS can fall 60 percent before the debt is larger than the collateral, even if nobody liquidates. From 250 percent down to 200 percent the vault cannot mint or withdraw, and it is not yet open to liquidation. The page treats a fall of 20 percent from a full vault as the step onto that 200 percent line. Below 200 percent, anyone can repay part of the debt, take the matching KAS, and take a penalty from the owner, and the position is brought back to 200 percent. Below 100 percent, part of the debt is unbacked.

Anyone can redeem one unit for one dollar of KAS, taken from a vault the holder chooses. If the unit trades under a dollar, the page's arbitrage is to buy it and redeem. If it trades over a dollar, the arbitrage is to lock KAS, mint, and sell. There is no company that posts the peg.

The loss is not spread. The page says a crash that gets past the collateral does not mint a new asset to cover it, and does not move the hole onto other vaults. Holders redeem against vaults that still have enough KAS. What is left trades as whatever the market will pay until someone repays it or the price of KAS recovers.

Three stages, as that page orders them. While votes are fresh and liquidators are awake, vaults are walked back to 200 percent and the unit stays backed. If the oracle or the liquidators are absent, minting, withdrawing, liquidating, and redeeming stop, and adding KAS, repaying, and closing do not. A full vault is still backed down to a 60 percent fall. If the fall goes past half while liquidation cannot run, vaults that were sitting on the 200 percent line are unbacked, and the shortfall stays on those units. If no valid price is posted for 30 days, anyone can settle at the last price. Settlement ends minting for that vault. Holders redeem at the settlement price with no fee. The owner withdraws KAS left above the debt.

## The oracle is the Nakamoto part

Every miner can put a price in the coinbase payload of a block it mines. The page says a small plugin reads public exchange prices, or a feed the miner chooses, and appends that price. The vote does not change the block reward and does not take extra hardware.

A window is the last ten minutes, and the page marks that length as preliminary. The votes in the window are sorted and the median is the price. An attacker with less than half of the votes cannot push the median outside the honest votes. Half of the votes is half of the hashrate only when every block votes. The window counts only if at least half of its blocks carried a vote. At that floor, half of the votes is a quarter of the hashrate. Below half of the blocks voting, the window gives no price. Stopping the oracle by refusing to vote, when everyone else votes, takes more than half the hashrate. When fewer vote, it takes less. The page's own chart is that trade. This study does not print a pool share, and it does not treat the oracle as safer than the chain.

Two prices are taken from the median, so a false print cannot empty a vault in one window. The mint price may rise by at most 10 percent an hour, and it may fall at once. The liquidation price may fall by at most 25 percent an hour, and it may rise at once. The page's reason for the downward cap is an attacker with half the votes posting a very low price and redeeming the KAS out for almost nothing. With the cap, that false price takes hours, and owners can repay and close without a fresh price.

Getting the median onto the chain is a separate act. Anyone may collect a valid window and post a zero-knowledge proof that the votes are the votes in those blocks, none added and none omitted, and that the median was computed from them. The page says the on-chain record checks that proof against the KIP-21 commitment. It does not name which proof tag. This repository's pin accepts Groth16 `0x20` and RISC0-Succinct `0x21` and rejects every other byte. A proof that is not one of those tags does not verify on v2.1.0. The draft does not say which tag it will use. KIP-16 is the opcode family. The roadmap describes Alexander Safstrom as its author and as the lead of this design. That is the page's sentence. It is not a statement that the dollar proof already verifies.

If no valid price has been posted for 30 minutes, every action that needs a price stops. Adding collateral, repaying, and closing continue. The oracle fee is a fixed amount of KAS. Miners whose votes landed within some band of the median, the band marked TBD, receive a fixed amount per block. The poster of the update receives one covenant transaction. Liquidators pay that fee. The page says nothing in the protocol slashes an attacker beyond the lost oracle reward. The costs it names are the hashrate, miners leaving a pool that attacks in public, and the price of KAS if the attack is seen.

A launch gate is still unnamed. The page says a ceremony will wait until some share of blocks is voting, and that the share is not chosen yet. After that, the 50 percent rule is the one above. Membership covenants, publisher software, and an indexer are named on the roadmap as things to build. The how-it-works page does not specify the membership rule. Liquidation is described as anyone acting, over a gossip layer the page calls Stroemnet in a parenthesis. The transport is not specified. This reading does not invent it.

Miner revenue is the part of the draft that ties the dollar back to the chain rather than to a foundation. A vote is optional work on top of the block the miner was already seeking. The fee is KAS. The chain's off switch stays the miners. A dollar built this way does not add an office that can halt blocks.

## What the branches are allowed to do with it

The fifteen branch pages each have a short draft reading. The pattern is the same, and the object is not.

The 81-byte seal remains the clock for a file a verifier can see. The draft unit is only a candidate for the invoice, the deposit, or the bounty, and only where both sides want that unit. [seal_escrow.sil](code/covenant/seal_escrow.sil) locks KAS until an arbiter signs, against a document hash. The dollar vault locks KAS in order to mint the unit. They are different scripts. This repository does not compile the vault, and it does not pretend the escrow spends the unit.

A supplier who wants dollars and does not want KAS is not automatically served. Redeeming the unit returns KAS. Selling that KAS for a bank dollar needs a market and a bank on the other side of the sale. The draft removes the issuer of the unit. It does not remove the price of KAS, and it does not remove the bank from the last step into a bank account.

## What it does not become

It does not keep a license, a beam, a contactor, a vehicle command, or a permit. It does not clear a sanction or an export denial. It does not warehouse a fab. It does not bid a national grid. It does not carry medical images, prompts, or vehicle traces. It is not USDT and not USDC. Those have an issuer. DAGKnight is not the oracle. KIP-2 is Proposed and is not in v2.1.0. If it lands, a client can treat a block as ordered on a measured delay. The price in this draft is a coinbase vote plus a proof against the lane commitment, which the rail page already marks Active.

The roadmap aims at a testnet in May 2027 and a mainnet in September 2027, after a fundraiser that closes on 1 January 2027 if the target is missed and contributions are refunded. On 6 November 2026 the front page showed a little over 10 percent of a 5,000,000 KAS target. That number moves. It is not evidence that the design works.
