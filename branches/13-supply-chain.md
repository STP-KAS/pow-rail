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

A passport digest and a serial digest stay readable in countries where the vendor site is blocked, for anyone who can reach any Kaspa node. No passport company can freeze the digest. No ministry can edit it without producing a later, different transaction that everyone can see.

The manufacturer can still be ordered to stop shipping, to recall, or to hand over the underlying file. The rail does not hide the file and does not complete a crossing that customs refused.

## What this rail does not do

It does not replace a regulatory database, certify a mineral as conflict-free, or clear a sanctioned counterparty. It makes a hash hard to quietly withdraw.
