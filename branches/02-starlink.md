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

## Detail

Two seals, two kinds. A public manifest uses `Kind::PublicFile`. A public status page, or an already-public notice that a region was turned off or on, uses `Kind::PublicStatus`.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Starlink,
    kaspa_seal::Kind::PublicFile,
    manifest_bytes,
);
```

Anyone can run that on a file SpaceX already published. SpaceX does not have to call it. The same pattern is what [Kira-Ryan/ephemera](https://github.com/Kira-Ryan/ephemera) does into Bitcoin with OpenTimestamps. Kaspa's difference is the commitment the node already computes: version-1 `payload_digest` inside `id_v1` ([`hashing/tx.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L207-L223) at v2.1.0). [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) then folds that accepted transaction into the lane of its `subnetwork_id`, and the header stores one `SeqCommit`. A researcher who only wants a clock can stay on the native lane, twenty zero bytes, `native_lane_id()`. A prover who wants the manifest stream without the rest of the DAG builds a user lane with `user_lane_id`. The namespace rule is KIP-21 section 2.1: 4 bytes, 16 zero bytes, and a non-zero byte in `namespace[1..=3]`. Mainnet allows 50 such lanes in one block ([`config/constants.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L97-L100)).

A KAS payment beside cards is the escrow, not a second copy of the manifest. [`seal_escrow.sil`](../code/covenant/seal_escrow.sil) releases coins when the payload's document hash matches the hash the covenant was built with. The entitlement to the radio is still SpaceX's. Nothing in `OpZkPrecompile` addresses a phased array. The only tags in v2.1.0 are Groth16 `0x20` and RISC0-Succinct `0x21`.

Subscriber traffic, terminal locations, and card numbers are not inputs to `document_hash`. The payload is 81 bytes of magic, version, branch, kind, flag, and hashes. [The code notes](../code/BYTES.md) record why the docs-site "about 25 KB" mass cap was not treated as a constant of this tag, and why 81 bytes is inside the transient limit the mainnet struct does name.
