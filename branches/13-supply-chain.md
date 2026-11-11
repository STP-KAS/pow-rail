# Supply chain, service, and battery records

Tesla and SpaceX buy cells, metals, chips, and parts, and they service machines for years. European battery rules and similar regimes ask for a record that follows a battery through service, resale, and recycling. The public code does not implement that passport. The operational question is who can delete it, and who can counterfeit a serial number in the only database that matters.

## 1. Exposure

**One portal.** A passport vendor, an ERP module, or an OEM service site can be the only reader. If that company is compelled to close a region, or if it fails, a recycler in another country cannot show the record. The legal duty to keep the record stays on the manufacturer either way. The availability of the record is a vendor question.

**The border and the bank.** Customs can hold goods. A metal supplier's bank can lose dollar correspondent access. The factory did not do anything technical wrong. The payment corridor closed. This is the same exposure as [Starbase](05-starbase.md), concentrated on categories governments already police: critical minerals, sanctioned counterparties, and strategic chips.

**Counterfeit service parts.** Authenticity is "the OEM database says this serial is real." A buyer in a country where that site is blocked, or a buyer looking at a gray-market part, has no second check.

Putting the full passport, the supplier list, or a customer's service address on a public chain publishes commercial and personal data. The exposure is not fixed by more publication. It is fixed by a digest.

## 2. Worth exploring

**Passport seals.** When a passport extract is issued, seal its hash. The extract lives with the parties the law already requires. A recycler, a second owner, or a regulator who has the extract can check the hash against the rail even if the vendor portal is down or blocked. A changed extract is a new hash. The old one remains, so a silent edit is visible.

**Serial seals for parts.** The manufacturer seals serial-to-batch at ship time. A service center checks the seal. This works without Tesla operating a node for the service center. Anyone can read the header. That is the permissionless part, used as a publication, not as a marketplace.

**Payments only in the gap.** Where a lawful supplier cannot get a reliable wire, and both sides want KAS, a covenant can hold the payment against the sealed packing-list hash. Where the block is a sanction or an export denial, the rail is not a workaround. The company's compliance duty stays. Miners including a transaction do not make a forbidden shipment lawful.

KCC-20 Final is a layout for a fungible covenant. A battery is not fungible in the way that layout means. Do not represent a serialised pack as a fungible token and call it a passport.

## 3. While miners mine

A passport digest that was broadcast can be shown, while the block is still inside a default node's pruning window, by anyone who can reach such a node. After that window, showing it takes a kept copy of the transaction and a lane witness, or a node that kept the history. No passport company can rewrite an already accepted transaction into a different digest. A later transaction is a later digest.

The manufacturer can still be ordered to stop shipping, to recall, or to hand over the underlying file. The rail does not hide the file and does not complete a crossing that customs refused.

## Dollar, still a draft

The payment in the gap, where a lawful supplier cannot get a wire, is the dollar surface. [Kaspa Dollar](../DOLLAR.md) would let that payment be denominated in the draft unit instead of only in KAS, with the packing-list hash still the passport. The pack does not become a fungible token. A sanction or an export denial is still not a missing opcode. Redeeming the unit returns KAS. The design is a draft. It is not a coin yet.

## Space watts

The batteries in the post are cells and a record. A packing list the parties are already allowed to show can be sealed. The seal does not make the cell and does not close the winter gap. The reading is in [WATTS.md](../WATTS.md). Whether a node is the best tool on this branch is [NODE.md](../NODE.md).

## What this rail does not do

It does not replace a regulatory database, certify a mineral as conflict-free, or clear a sanctioned counterparty. It makes a hash hard to quietly withdraw.

## Detail

The kind is `PassportExtract` so a fungible layout is not what the bytes claim to be.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Supply,
    kaspa_seal::Kind::PassportExtract,
    passport_extract,
);
```

The extract stays with the parties the law already names. Bytes 17..49 are its unkeyed BLAKE3. A changed extract is a new payload and a new `payload_digest`. The old transaction stays in the lane it was accepted into. [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) commits lane tips. It does not provide an edit.

A serial at ship time is the same call. A service center recomputes the hash. It does not need the manufacturer to run a node for it. Reading a header is permissionless. That part is [the rail](../RAILS.md).

[KCC-20](https://github.com/kaspanet/kccs/blob/main/kcc-0020.md) was Final. Its title is a fungible token covenant. A serialised pack is one object. Representing it as a fungible balance would make two packs interchangeable, which is the failure mode a passport exists to prevent. [Kasplex](https://wiki.kaspa.org/en/Kasplex_KRC_20) is a third-party data-insertion protocol, indexer, and API, and KRC-20 is that project's token convention. The [wiki FAQ](https://wiki.kaspa.org/faq) treats it as outside the core team. This encoder does not produce a KRC-20 body.

A lawful supplier and a buyer who both want KAS can point [`seal_escrow.sil`](../code/covenant/seal_escrow.sil) at the packing-list hash. That file is a single-input example that trusts the arbiter. It checks `tx.outputs[0]` only and has no timeout. A sanction or an export denial is not a missing opcode. Miners including the transaction do not clear the shipment. Customs and the company's compliance duty are unchanged.

The mass story, including the tag's own disagreement between a 125_000-byte comment and a 250 KB mainnet transient limit, is written down in [the code notes](../code/BYTES.md) so this page does not invent a third number.
