# Tesla vehicles, FSD, robotaxi

Tesla sells cars, driver-assistance and supervised self-driving software, and is building a robotaxi service. The public protocols are [vehicle-command](https://github.com/teslamotors/vehicle-command) and [fleet-telemetry](https://github.com/teslamotors/fleet-telemetry). The car executes a command when the command matches a key in the vehicle keychain. Telemetry goes to a server the owner or the fleet operator chose. Both designs are already decentralized in the only way that matters for safety: the car does not ask a blockchain what to do.

## 1. Exposure

**The company account.** The app, the OTA server, and the signed-in features are Tesla's. A government can order the company to stop pushing updates in a country, to disable a feature, or to close a fleet's connectivity. The cars already sold keep the software they have. They lose the company's ongoing service. That is a worldwide operational risk because the same vendor serves many legal systems.

**The telemetry path.** Fleet telemetry is only as available as the server the operator picked, and as the network path to it. A local block on that host stops the stream. Copying the stream onto a public chain would publish where cars are and how they are driven. That is a worse failure than the outage.

**Robotaxi receipts.** A fare, a ban, or a dispute that lives only in one fleet database can be edited or closed with the database. Riders and cities who are not Tesla have no second copy.

**The keychain.** This is not an exposure Kaspa should "fix." End-to-end command authentication is the right design. Adding a global round trip would slow the command and publish it.

## 2. Worth exploring

**OTA release seals.** When a software build is published, seal the hash of the release notes and the hash of the image Tesla already lets owners verify. Owners, insurers, and a safety agency can show which build was offered on which date even if the download server later serves something else or goes dark in a region. The car still trusts Tesla's signature. The chain is the public clock, not the root of trust.

**Fare and incident receipts, without traces.** For a robotaxi trip, seal a digest of a receipt the rider already holds: time bucket, price, vehicle id the rider was shown, outcome. Leave the GPS trace off-chain. A city and a fleet can disagree about a policy and still agree which receipt was issued. A covenant in KAS can hold a deposit for a party who wants that. A dollar invoice remains a dollar invoice.

**Service bulletins.** Same pattern as OTA. A bulletin that is public, or that is shared with a defined set of owners, gets a digest. A later disappearance from the portal does not erase the fact of publication.

Do not design a proof that tries to unlock FSD, immobilizers, or paid upgrades. The opcode that exists checks specific precompiles. A bypass is not a use of the rail.

## 3. While miners mine

The release digest remains if Tesla's update host is blocked in a country or the cloud account is suspended. A receipt digest remains if the fleet database is ordered wiped, to the extent the digest was broadcast before the order. Miners do not know the owner's name.

The car does not keep receiving Tesla's live services. Autopilot and FSD behavior stays whatever the installed build does, under Tesla's keys and under the road rules of the place the car is. A chain cannot legally or technically re-enable a feature the manufacturer has stopped signing.

## What this rail does not do

It does not drive, steer, or authenticate a door command. It does not store dashcam video or location history. It does not jailbreak a vehicle. [vehicle-command](https://github.com/teslamotors/vehicle-command) already solved command authentication without miners.
