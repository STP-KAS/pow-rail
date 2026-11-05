# Launch: Falcon, Starship, Dragon

SpaceX flies Falcon 9, Falcon Heavy, Starship, and Dragon. The public code for this branch is thin. [SpaceExplorationTechnologies](https://github.com/SpaceExplorationTechnologies) has GPL forks of Linux and U-Boot, not the guidance software. Range safety, the launch license, and the vehicle stay with the company and the licensing state.

## 1. Exposure

A launch can be stopped without touching a blockchain. A license can be suspended. A range can be closed. An export rule can hold a component at a border. A bank can delay payment to a supplier in another country. A cloud tenant that stores public telemetry, webcast logs, or anomaly memos can be closed by the cloud company when a government asks.

The flight computer is not one of those exposures. It does not need a public network, and putting trajectory or guidance data on one would publish the thing export rules exist to contain.

The exposure that matches a rail is the **record after the fact**, and the **payment to a supplier**, when both depend on a vendor who has an off switch.

## 2. Worth exploring

Seal the hash of a public telemetry snapshot, a published mishap update, or a revision of a public user's manual at the moment it is released. The file can stay on spacex.com. The digest sits on Kaspa. If the site or the bucket later serves a different file, the earlier digest is still ordered.

Do the same for a supplier milestone that several companies already share: a test report, a lot certificate, a shipment notice. Each party keeps the PDF. Any of them can seal the hash. None of them is then the only clock.

KAS escrow fits only a milestone both sides are willing to denominate in KAS. A launch contract is written in dollars. See [the rail](../RAILS.md). A dollar with no issuer is a 2026 design, not a coin.

## 3. While miners mine

A sealed digest stays ordered if the webcast host, the status site, or a foreign supplier's bank portal is compelled to go dark. Nobody at a foundation can delete that transaction. The next block still arrives.

The license does not stay. The vehicle does not fly because a miner included a hash. A government order served on SpaceX still binds SpaceX. The rail binds only the bytes that were already validly broadcast.

## What this rail does not do

It does not store guidance software, ITAR packages, or range-safety keys. It does not replace a flight termination system. It does not pay a supplier who does not want KAS.
