# Dojo, training, and on-site compute

Tesla has built training clusters, including the Dojo line, and a general compute appetite that Terafab and Starmind are also answers to. The public protocol in this branch is [TTPoE](https://github.com/teslamotors/ttpoe): Tesla Transport Protocol over Ethernet, a layer-2 fabric for very high bandwidth, published as a kernel module and a protocol disclosure. [liblithium](https://github.com/teslamotors/liblithium) is a small cryptography library. Neither is waiting on a consensus algorithm.

## 1. Exposure

Training clusters sit in a few buildings, on a few interconnects, under a few export regimes. A government can restrict accelerators, cap a power contract, or compel the company that owns the building. A public chain does not add a GPU and does not move a cluster out of the jurisdiction of the building.

The record exposure is the one that grows with regulation and with public argument about models. A lab can publish an eval, a data-cutoff date, or a system-card hash, and can later replace the PDF on its own site. Readers in another country may not reach the site at all. The training run itself stays private, as it should. The **claim about the run** is what gets edited.

TTPoE's threat model is a rack, not a ministry. Time sync on that fabric belongs on the wire, next to Precision Time Protocol, not in a block.

## 2. Worth exploring

**Seals of published claims.** When Tesla or xAI publishes a system card, an eval table, or a data statement, seal the hash. Researchers who cannot open the site can still be handed the PDF and check it. A replacement PDF does not inherit the old order.

**Chip and cluster acceptance.** A Terafab wafer lot, a delivered training rack, or a firmware image for a training NIC can use the same traveler seal as [Terafab](06-terafab.md). The secret is the design file. The public object is the digest of the acceptance record.

**Do not explore putting gradients, datasets, or TTPoE frames on Kaspa.** The throughput is absurd for a blockDAG, and the data is the asset.

vProgs, if it ever ships as a tagged release, could anchor a proved statement such as "this eval program, on this sealed input hash, produced this output." Today KIP-16 checks specific precompiles only. A training proof is a research bet, not a rail an operator can use. The hash of the published PDF is the piece that is usable now.

## 3. While miners mine

A system-card digest remains available if the corporate site is blocked, geofenced, or edited. The cluster does not keep training because of that. Power, export classification, and the owner of the building still decide whether the next run starts.

There is still no company to call to have the old digest removed. That is the property worth checking before a public controversy, not during one.

## Dollar, still a draft

A cluster reservation or a published-job bounty is the dollar surface. [Kaspa Dollar](../DOLLAR.md) would denominate that reservation in a unit with no bank key. The training run, the fabric, and the power contract stay where they are. A system card that was already published can still be sealed. Customer data cannot. The draft does not smuggle a restricted accelerator, and it does not pay the utility for the hall. The design is a draft. It is not a coin yet.

## What this rail does not do

It does not replace Dojo, TTPoE, or a data-center power contract. It does not smuggle restricted accelerators. It does not prove that a private training run matches a public card, unless a real proof system for that statement exists and the publisher runs it. The live piece is the seal of the card they chose to publish.

## Detail

The live piece, in code:

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Compute,
    kaspa_seal::Kind::Eval,
    system_card_pdf,
);
```

A replacement PDF hashes differently, so it does not inherit `payload_digest` of this payload. Researchers who are handed the PDF can check bytes 17..49. Researchers who are not handed the PDF learn only the hash. That is the publication the lab chose when it sealed the card.

[TTPoE](https://github.com/teslamotors/ttpoe) is a layer-2 Ethernet fabric. [liblithium](https://github.com/teslamotors/liblithium) is a small cryptography library. Neither import belongs in this crate. Time on that fabric is the wire's time. A block hash is a later receipt.

A training proof would be a [KIP-16](https://github.com/kaspanet/kips/blob/master/kip-0016.md) precompile only if the statement matched tag `0x20` or `0x21` and the node verified it. v2.1.0's `verify_zk` path charges the tag's script units and pushes true on success ([`opcodes/mod.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/crypto/txscript/src/opcodes/mod.rs#L758-L771)). The tests in `tags.rs` expect room for 3 Groth16 verifications or 2 succinct verifications in one block's compute mass. That is a few proofs per block, not a gradient step. The public inputs would be whatever the proof journal commits to. A private dataset that never enters the journal is not proved. Shipping that proof is future work with a tagged prover. The PDF hash is the call above.

Gradients, datasets, and TTPoE frames are not seal inputs. The transient limit that is actually in the mainnet struct is a quarter of a megabyte of body per block at the factor of 4. The code notes record the arithmetic and the open pull that would restate it: [code/BYTES.md](../code/BYTES.md).
