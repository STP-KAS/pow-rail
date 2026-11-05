# Starbase and the factories

Starbase, the Hawthorne and other factories, and the supplier web around them are manufacturing branches. They buy parts, run tests, move hardware across borders, and keep inspection records for years. Almost none of that code is public. The exposure is the vendor in the middle of the record and the vendor in the middle of the payment.

## 1. Exposure

A customs authority can hold a crate. That is the state's border, and a chain will not open it.

A supplier's bank can be cut off from correspondent dollars. The part can be finished and the invoice stuck. That happens to manufacturers in contested payment corridors, and it does not require a special rule aimed at rockets.

An ERP tenant, a quality-system host, or a lab-report portal can be suspended. The inspection happened. The PDF lived in one vendor's account. If that account closes, the insurer, the customer, and the other factory are arguing from memory.

A single sign-on provider can lock the workforce out of those portals in one country. The tools on the floor still exist. The records do not open.

## 2. Worth exploring

Seal the hash of an inspection pack, a non-conformance report, a calibration certificate, or a shipment's packing list at the moment the parties accept it. Each company keeps the file in its own system. The digest is the common object. When a portal is down, any party with the file can show it is the file that was accepted, because the digest is on a network none of them operate.

Use one lane if the volume of seals ever justifies a proof. A factory does not need its own chain.

KAS escrow is a possible settlement for a small supplier who already wants KAS and cannot get a reliable dollar wire. It is a bad fit for a large purchase order priced in dollars. Do not pretend a volatile coin is a treasury policy. The improvement to explore is the **record**, first, and the **payment**, only where the bank rail is the thing that actually fails.

## 3. While miners mine

The accepted digest cannot be unpublished by the ERP vendor, the cloud region, or a ministry that compels that vendor. A later file is a new hash, and both orders are visible. That is the protection: not secrecy, but the inability of one office to rewrite the sequence.

The factory can still be fined, permitted, or closed. The rail does not employ the line.

## What this rail does not do

It does not move the crate, stamp the passport of an employee, or store drawings that are export-controlled. Hash in, file stays home.

## Detail

The common object is one inspection hash.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Starbase,
    kaspa_seal::Kind::Inspection,
    inspection_pack,
);
```

Each company keeps the PDF. Any of them can broadcast the 81-byte payload. The bytes 17..49 are the unkeyed BLAKE3 of the pack. A later pack is a later hash. Both can sit in the accepted sequence. Deleting one requires a history the header commitment does not offer an admin key for. The commitment is [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) `SeqCommit`, stored in `accepted_id_merkle_root`, over lane tips plus miner payloads. A factory does not need its own lane until someone proves that lane. The native subnetwork already orders the transaction. Mainnet's cap is 50 lanes per block, so a lane per building would be the wrong reading of [`DEFAULT_LANES_PER_BLOCK_LIMIT`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L97-L100).

A small supplier who already wants KAS can use [`seal_escrow.sil`](../code/covenant/seal_escrow.sil). The constructed document hash is the packing-list hash. The script will not release the coins onto a payload that carries a different hash. The amount is this input's value minus the fee written in the script. A purchase order priced in dollars is not rewritten into that script. The script's 1000 sompi fee is the upstream example's number, not the mempool floor of 100_000 sompi in v2.1.0 `mining/src/mempool/config.rs`.

Drawings that are export-controlled are not `inspection_pack`. The layout is too small for them on purpose. The rest of the arithmetic is in [the code notes](../code/BYTES.md).
