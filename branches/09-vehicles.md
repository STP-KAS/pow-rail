# Tesla vehicles, FSD, robotaxi

Tesla sells cars, driver-assistance and supervised self-driving software, and is building a robotaxi service. The public protocols are [vehicle-command](https://github.com/teslamotors/vehicle-command) and [fleet-telemetry](https://github.com/teslamotors/fleet-telemetry). The car executes a command when the command matches a key in the vehicle keychain. Telemetry goes to a server the owner or the fleet operator chose. Both designs are already decentralized in the only way that matters for safety: the car does not ask a blockchain what to do.

## 1. Exposure

**The company account.** The app, the OTA server, and the signed-in features are Tesla's. A government can order the company to stop pushing updates in a country, to disable a feature, or to close a fleet's connectivity. The cars already sold keep the software they have. They lose the company's ongoing service. That is a worldwide operational risk because the same vendor serves many legal systems.

**The telemetry path.** Fleet telemetry is only as available as the server the operator picked, and as the network path to it. A local block on that host stops the stream. Copying the stream onto a public chain would publish where cars are and how they are driven. That is a worse failure than the outage.

**Robotaxi receipts.** A fare, a ban, or a dispute that lives only in one fleet database can be edited or closed with the database. Riders and cities who are not Tesla have no second copy.

**The keychain.** This is not an exposure Kaspa should "fix." End-to-end command authentication is the right design. Adding a global round trip would slow the command and publish it.

## 2. Worth exploring

**OTA release seals.** When a software build is published, seal the hash of the release notes and, if the owner is given an image hash, that hash. This page does not claim that Tesla already lets owners verify an image. Owners, insurers, and a safety agency can show which digest was ordered, to the extent it was broadcast, even if the download server later serves something else or goes dark in a region. The car still trusts Tesla's signature. The chain is the public clock, not the root of trust.

**Fare and incident receipts, without traces.** For a robotaxi trip, seal a digest of a receipt the rider already holds: time bucket, price, vehicle id the rider was shown, outcome. Leave the GPS trace off-chain. A city and a fleet can disagree about a policy and still agree which receipt was issued. A covenant in KAS can hold a deposit for a party who wants that. A dollar invoice remains a dollar invoice.

**Service bulletins.** Same pattern as OTA. A bulletin that is public, or that is shared with a defined set of owners, gets a digest. A later disappearance from the portal does not erase the fact of publication.

Do not design a proof that tries to unlock FSD, immobilizers, or paid upgrades. The opcode that exists checks specific precompiles. A bypass is not a use of the rail.

## 3. While miners mine

A release digest that was broadcast can be shown while a default node still has the block at or after its pruning point, and after that from a kept copy, if Tesla's update host is blocked in a country or the cloud account is suspended. A receipt digest remains in that same sense if the fleet database is ordered wiped, to the extent the digest was broadcast before the order. It does not remain on every node forever. Miners do not know the owner's name.

The car does not keep receiving Tesla's live services. Autopilot and FSD behavior stays whatever the installed build does, under Tesla's keys and under the road rules of the place the car is. A chain cannot legally or technically re-enable a feature the manufacturer has stopped signing.

## What this rail does not do

It does not drive, steer, or authenticate a door command. It does not store dashcam video or location history. It does not jailbreak a vehicle. [vehicle-command](https://github.com/teslamotors/vehicle-command) already solved command authentication without miners.

## Detail

An OTA seal names the image and the release notes. The car keeps trusting Tesla's signature. The chain is the clock.

```rust
let payload = kaspa_seal::encode_seal(&kaspa_seal::Seal {
    branch: kaspa_seal::Branch::Vehicles,
    kind: kaspa_seal::Kind::WeightOrImage,
    primary: kaspa_seal::document_hash(image_bytes_owners_can_verify),
    aux: Some(kaspa_seal::document_hash(release_notes)),
});
```

A robotaxi receipt the rider already holds is `Kind::Receipt` over that receipt, with the GPS trace left out of the hashed bytes. If the trace is inside the file, the hash is a commitment to the trace, and a later leak of the file plus the public hash confirms it. Hash the receipt the rider can show. Do not hash the trace to "anchor" it.

The precompile is the wrong tool for a command. `OpZkPrecompile` is `0xa6`. The tag match in v2.1.0 [`tags.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs) is `0x20` Groth16 and `0x21` RISC0-Succinct. Every other byte is `UnknownTag`. There is no tag whose success path unlocks a door, an immobilizer, or a paid feature. [vehicle-command](https://github.com/teslamotors/vehicle-command) checks a key in the vehicle. A block round trip would publish the command and miss the deadline.

[KIP-17](https://github.com/kaspanet/kips/blob/master/kip-0017.md) introspection can require a payload hash, which is what the escrow does for a KAS deposit. It cannot sign a firmware image. Service bulletins use the same `WeightOrImage` or `PublicFile` call as the OTA notes.

Dashcam bytes and location history do not enter `document_hash`. The layout is 81 bytes so they cannot enter the payload either. Citations for the id split and the mass limits are in [the code notes](../code/BYTES.md).
