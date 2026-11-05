# Starlink

Starlink is the broadband constellation and the service around it. Public code: [enterprise-api](https://github.com/SpaceExplorationTechnologies/enterprise-api) and the GPL router tree [starlink-wifi](https://github.com/SpaceExplorationTechnologies/starlink-wifi). The radio, the account, and the billing relationship are SpaceX's.

## 1. Exposure

Three different switches get conflated. Only one of them is a Kaspa question.

**The beam.** SpaceX can geofence a cell. A government that licenses spectrum, or that can compel the operator, can demand that. The satellite and the user terminal obey the operator. A proof-of-work network cannot turn a cell on, and this note does not describe any attempt to. Service entitlement stays with SpaceX.

**The bill.** A subscription is collected through card networks, app stores, and local payment companies. Those companies have frozen merchants and whole categories when a government, or their own risk desk, told them to. A dish can be powered and the card rail can still be dead. That is a real interruption of operations, and it is worldwide in the sense that each country's acquirers are a different switch.

**The public catalog.** SpaceX publishes short-lived files of where each satellite will be. The next file replaces the last. A community archive, [Kira-Ryan/ephemera](https://github.com/Kira-Ryan/ephemera), already hashes those manifests and stamps them into Bitcoin with OpenTimestamps, because the history otherwise disappears. The job exists. Kaspa is another public clock for the same job, with blocks about ten times a second and a sequencing commitment ([KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md)) built for lane-scoped proofs.

The account database is a fourth switch: one system can mark a terminal unpaid. That database is the product. Replacing it with a public ledger would publish customer state. Do not.

## 2. Worth exploring

**Ephemeris seals.** Anyone, including a researcher who is not SpaceX, can seal each public manifest. The operator does not have to integrate anything. If a later dispute is about what the public catalog said on a given day, the digest does not depend on one archive staying online.

**A second way to pay, beside cards.** A subscriber who holds KAS, and a merchant who wants KAS, can settle on the rail. SpaceX's own checkout can keep using cards. A lawful local reseller who already has the right to sell service could offer KAS as one more way to pay that reseller. The entitlement to the radio still comes from SpaceX. Paying miners does not buy bandwidth.

**Outage and status history.** Hash the public status page, or the public order that turned a region off or on, when that order is already public. The operational act remains the company's. The historical claim becomes checkable if the status site is later edited or blocked in a country.

## 3. While miners mine

A KAS payment that has been included cannot be charged back by Visa, and it cannot be frozen by an issuer, because there is no issuer. A manifest digest cannot be quietly replaced. Both keep their order while miners mine.

The phased array does not. If SpaceX or a regulator turns a cell off, the chain can record that a public statement was made. It cannot radiate.

## What this rail does not do

It does not bypass geofencing, account suspension, or an export hold on a terminal. It does not put subscriber traffic, locations, or payment card numbers on a public ledger. The GPL router code does not need a consensus protocol.
