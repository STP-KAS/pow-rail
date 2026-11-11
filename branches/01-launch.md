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

## Dollar, still a draft

A supplier invoice priced in dollars is the surface [Kaspa Dollar](../DOLLAR.md) is aimed at. The draft unit would be what that invoice is denominated in, if both sides want it, with KAS locked in a vault at the ratios on that page. The seal of the note or the packing list stays the 81-byte clock. The escrow on this page locks KAS against an arbiter and a hash. The dollar vault locks KAS in order to mint the unit. A supplier who redeems receives KAS, not a bank deposit. The license and the vehicle do not move either way. The design is a draft. It is not a coin yet.

## Space watts

A public telemetry note can be sealed before flight. A ship that stays up can carry a pre-synced node. A booster flight of minutes does not get a node. The longer reading is [WATTS.md](../WATTS.md). Whether a node is the best tool on this branch is [NODE.md](../NODE.md). The license, the range, and the vehicle stay where they are.

## What this rail does not do

It does not store guidance software, ITAR packages, or range-safety keys. It does not replace a flight termination system. It does not pay a supplier who does not want KAS.

## Detail

The public file stays where it was published. The chain takes 81 bytes.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Launch,
    kaspa_seal::Kind::PublicFile,
    public_note,
);
let payload_half = kaspa_seal::payload_digest(&payload);
```

`public_note` is the telemetry snapshot, the mishap update, or the manual revision the sealer is willing to show a verifier. `payload_half` is the payload half of a version-1 transaction id, not the id itself. The node builds that half as `PayloadDigest`: BLAKE3 keyed with the 13-byte domain `PayloadDigest` and 19 zero bytes. `id_v1` then hashes that digest and the rest-of-transaction digest under the domain `TransactionV1Id`. Both steps are in `consensus/core/src/hashing/tx.rs` at [rusty-kaspa v2.1.0](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/hashing/tx.rs#L207-L223). The payload field they cover is `Transaction.payload` in [`tx.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/tx.rs#L254-L261).

The file hash inside the seal is unkeyed BLAKE3, at bytes 17..49. Passing the note through `PayloadDigest` directly would commit to different bytes than the seal the verifier recomputes. The crate tests that the two functions differ.

A supplier payment in KAS, when both sides want KAS, is the escrow in [`code/covenant/seal_escrow.sil`](../code/covenant/seal_escrow.sil). The script requires the spending payload to be a version-1 seal of the document hash it was constructed with, then sends this input's value, minus the fee the script names, to the buyer or the seller. That named fee is 1000 sompi. The relay rule is a mass rate, not that number. The script is a single-input example that trusts the arbiter. It checks `tx.outputs[0]` only and has no timeout. It reads `this.activeInputIndex` only. A launch contract written in dollars stays in dollars. [KCC-20](https://github.com/kaspanet/kccs/blob/main/kcc-0020.md) being Final does not create a dollar.

Guidance bytes are not an argument to `seal_file`. An 81-byte payload has no room for them, and a hash of a short controlled file is searchable. The license and the flight termination system are not opcodes. [KIP-16](https://github.com/kaspanet/kips/blob/master/kip-0016.md) `OpZkPrecompile` (`0xa6`) accepts tag `0x20` or `0x21` and rejects every other tag ([`tags.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/zk_precompiles/tags.rs)). Neither tag is a vehicle.

The native subnetwork is enough. A user lane, [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) section 2.1, is for a prover who will follow this stream alone. The encoder for that 20-byte id is `user_lane_id`. Forming the id does not submit it.

The full layout, the mass arithmetic, and the vector tests are in [the code notes](../code/BYTES.md).
