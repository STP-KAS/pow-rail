# Tesla energy

Tesla Energy is Megapack, Powerwall, solar, and the trading and aggregation software around them, including Autobidder. [nempy](https://github.com/teslamotors/nempy) is a public model of how the Australian National Electricity Market dispatches. It is a model of a market an operator already runs. It is not that operator.

## 1. Exposure

A grid operator can disconnect a site. A utility can refuse an interconnection. A market operator can suspend a bidding entity. Those are physical and legal switches. A public chain cannot push a megawatt onto a wire, and it must not be described as a way around a disconnect order.

The software exposure is the **statement of what happened**. Meter readings, dispatch instructions, and settlement statements often live with an aggregator, an ISO portal, or Tesla's own cloud. If that tenant closes, or if a party in a dispute hosts the only copy, the electrons have already flowed and the record is captive.

Cross-border payment for a Megapack can sit in a correspondent bank. The hardware can be on the pad and the wire can be held. Same pattern as the factories.

Regulated markets will keep clearing in their own systems. Kaspa does not become AEMO, ERCOT, or a European TSO because it can order transactions.

## 2. Worth exploring

**Meter seals.** On an interval the parties already agree, hash the interval's reading (site id, timestamp, import, export) and seal the digest. The site owner, the aggregator, and the offtaker each keep the row. Any of them can show the row that was sealed. A government or a vendor who closes the aggregator's database does not obtain the power to rewrite the sealed intervals. They can still order the site offline going forward.

**Dispatch order, hashed.** The command "this site was told to discharge at this interval" can be sealed by whoever is allowed to utter it. The command itself travels on the utility's channel, which is faster and already authenticated. The chain answers a later question: which instruction was issued, in which order, when more than one company could benefit from editing the log.

**Settlement remains in fiat** for the market. Explore KAS only for a side agreement between two parties who want it, for example a delayed cross-border installment where the bank rail is the known failure. Do not propose replacing ISO settlement.

nempy shows the shape of a dispatch problem. It is not a candidate to "put on Kaspa." A toy sealed-bid commitment is something a student can build. A national market is not.

## 3. While miners mine

Sealed intervals stay in order if the aggregator is compelled to delete a tenant. There is no energy-company admin key on the chain.

The site can still be switched off at the breaker and at the interconnect. The money the ISO owes still clears under the ISO's rules. The rail protects the copy of the record, not the flow of power.

## What this rail does not do

It does not bid into a market, operate Autobidder, or keep a battery online in defiance of the grid operator. It does not turn a research model of the NEM into a second NEM.
