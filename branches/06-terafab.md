# Terafab

Terafab is the chip fab project described in 2026 as a build by SpaceX and Tesla, with xAI in the same story and Intel named as a process partner. Texas officials announced a Grimes County first phase on the order of $16.8 billion. Musk has said the companies will build and run the fab. Volume production has been described as a 2028 aspiration with wide error bars. There is no Terafab GitHub.

A fab's real system of record is a manufacturing execution system inside the building. Process recipes, yields, and tool logs are export-controlled or commercially secret. They do not go on a public chain.

## 1. Exposure

The project is multi-party before it is multi-jurisdiction. Tesla, SpaceX, a process partner, tool vendors (lithography, etch, metrology), construction firms, and a state incentive office will not share one database forever. The party who hosts the database can be compelled, hacked, or simply interested in a rewrite during a dispute.

Tool vendors are few. A government that can compel the tool vendor, or a government that can hold the tool at a port, can disturb the fab without passing a law about Tesla. The chain cannot unload a ship. It can make the **acceptance record** of what was delivered hard to rewrite after the fact.

Dollar clearing for a project of this size sits in banks. A KAS covenant cannot warehouse a phase-one budget. That is not a temporary gap. Collateral on the only live asset is KAS.

## 2. Worth exploring

**Lot-traveler hashes and calibration seals.** When a wafer lot, a tool qualification, or a shipment between the companies is accepted, seal the hash of the traveler. Counterparties verify the file against the digest. A later argument about which recipe revision or which calibration was current does not depend on the host of the portal staying friendly.

**One lane for the program.** KIP-21's point is that this stream of seals need not be proved together with every other application on Kaspa. The commitment is live. The fab software is not something Kaspa replaces.

**No recipes.** The digest is useful because it is public. The document is useful because it is not. Builders who invert that will harm the project.

KCC-20 being Final means a fungible covenant has a written layout. It does not create a settlement currency for tool invoices. Kaspa Dollar, if it ever matched its November 2026 design, would still be a coin borrowed against KAS, with no freeze key. A construction lender does not want an immutable coin whose collateral can gap. Explore seals now. Leave the capex in banks.

## 3. While miners mine

A qualification seal survives a falling-out between the partners and survives an order that closes one partner's cloud tenant. No company on the cap table can edit a mined digest. A government can still seize the building, hold the tools, and compel each company. The history of hashes already published remains, which is sometimes exactly the record a partner or an insurer needs, and sometimes a record the parties should not have made. Choose the hashes with that in mind.

## What this rail does not do

It does not run the process tools, store the recipe, move $16.8 billion, or substitute for an export license.

## Detail

The traveler is an inspection seal. The recipe is not in the function's argument list.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Terafab,
    kaspa_seal::Kind::Inspection,
    traveler_or_qualification,
);
```

Counterparties who hold the file recompute `document_hash` and compare bytes 17..49. The version-1 transaction id commits to the whole 81-byte payload through `PayloadDigest`, then `TransactionV1Id` ([`hashing/tx.rs` `id_v1`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L207-L218) at v2.1.0). A partner who closes a cloud tenant does not hold the key to that hash. There is no key.

One program lane is optional and only useful once a prover follows it. [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) section 2.1 is the byte rule `user_lane_id` enforces. The fab's manufacturing execution system stays in the building. The lane is a sequencing commitment, not a process tool.

[KCC-20](https://github.com/kaspanet/kccs/blob/main/kcc-0020.md) Final is a fungible-token covenant: state, transfer entrypoints, borrowed receive. A wafer lot is not fungible in that sense. [Kasplex](https://wiki.kaspa.org/en/Kasplex_KRC_20) is a third-party insertion protocol and indexer. Its KRC-20 convention is not KCC-20 and not a KIP. The [wiki FAQ](https://wiki.kaspa.org/faq) places that work outside the core team. Neither layout is a traveler.

A phase-one budget is not a covenant input. The only asset this escrow script moves is KAS, and only the coins on the input it spends. Collateral for a construction loan is a different conversation, and [Kaspa Dollar](../RAILS.md) was not a KIP and not code on this pass.

Recipes stay off both the hash input and the payload. [The code notes](../code/README.md) keep the mass and opcode citations in one place.
