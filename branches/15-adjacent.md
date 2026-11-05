# Neuralink and The Boring Company

These companies share a founder with SpaceX and Tesla. They are not folders in those companies' GitHub orgs. They are here so the map is complete, and so the rail is not forced onto work it would harm.

## Neuralink

Public repos: [tsui](https://github.com/neuralink/tsui) (a terminal UI for Tailscale), [DicomPreview](https://github.com/neuralink/DicomPreview) (Quick Look for medical images), plus two quiet forks. The product is a medical device. Trials, firmware, and neural recordings are regulated health data.

**Exposure.** A device company can be ordered to stop a trial, to recall, or to patch. That order lands on the company. A chain does not keep an unapproved device on the market, and it must not.

**Worth exploring.** Nothing that touches patient data. A hash of a published, already-public firmware release note is the same seal as any other software version, and it is optional. DICOM files, spike trains, and participant records are not candidates.

**While miners mine.** A digest of a public note would remain. The clinical system would not, and should not, move onto the rail.

**Do not.** Put images or neural data in a transaction. Do not build a "permissionless" command path to an implant.

## The Boring Company

Public code is [boringcompany/app](https://github.com/boringcompany/app), last pushed in March 2019. The work is tunnels and machines. There is no living software surface to integrate.

**Exposure.** Construction has the same bank, permit, and inspection-portal switches as [Starbase](05-starbase.md). A city can stop a project. A chain cannot dig.

**Worth exploring.** Hashes of inspection packs and shipment certificates, if the parties want a copy that the project portal does not exclusively hold. Only if a real project is producing those documents and a single vendor holds them. The empty GitHub repo is not a reason to invent a workflow.

**While miners mine.** A sealed inspection digest would remain. The permit would not.

## Everyone else

A supplier, a city, a university, or an owner can use the same three questions without a relationship to these companies. The node is public. The fee is KAS. No one in this repository sells the seat.

## Detail

Neuralink's public firmware note, if one exists and is already public, is the same call as any other software version.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Adjacent,
    kaspa_seal::Kind::PublicFile,
    already_public_firmware_note,
);
```

DICOM files, spike trains, and participant records are not a kind. They are not passed to `document_hash`. A hash of a medical file is still a commitment to that file, and the payload that carries the hash is readable by every node. `OpTxPayloadSubstr` (`0xb8` in v2.1.0) is how a script reads it. There is no cipher in the seal layout. There is also no command entrypoint. An implant is not a covenant output.

The Boring Company's inspection case, if a project is actually producing packs and one portal is the only copy, uses `Kind::Inspection` on this same branch id. The 2019 repository is not that project. The permit is not `encode_seal`.

Everyone else's use of the encoder is the same 81 bytes and the same `PayloadDigest`. The fee is whatever KAS the transaction pays. The crate does not invoice it. Tests recompute the node's published vectors. They do not submit. The write-up of those vectors, of KIP-21's 20-byte lane id, and of the two precompile tags is [the code notes](../code/BYTES.md).
