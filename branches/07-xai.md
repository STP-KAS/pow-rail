# xAI and Grok

xAI is the model group. In 2026 reporting it sits inside SpaceX, and the GitHub org [xai-org](https://github.com/xai-org) presents itself as SpaceXSI. Public code includes [grok-1](https://github.com/xai-org/grok-1) (the 2024 Apache release), prompts, API SDKs, grok-build, and the For You code covered under [X](08-x.md). Later Grok models are hosted. They are not in `grok-1`.

## 1. Exposure

The API is an account on a company server. A government can order that company to close a customer's key, to refuse a country, or to log prompts. The company can comply, because the company runs the server. A proof-of-work network does not take that duty away, and it should not be used as a side door around an order served on xAI.

The second exposure is quieter. A weight file, an eval, or a system prompt can be replaced on the host. Users of a hosted model have the vendor's word for what ran. Users of a deleted GitHub repo have nothing, unless a digest was sealed somewhere the repo host does not control. GitHub can be compelled too. Miners are a different set of parties.

Cloud credits, GPU export rules, and power contracts for ground training clusters are real government-sized switches. They are not blockchain problems. See [compute](14-compute.md) and [Terafab](06-terafab.md).

## 2. Worth exploring

**Seal published weights.** `grok-1` is already public. Anyone can hash the files they downloaded and broadcast the digest. If the tag is moved or the repo is removed, the digest still has an order. People running their own copy can show which bytes they loaded. This requires no permission from xAI. That is the sense in which the tools are for anyone.

**Seal an eval the publisher chooses to stand behind.** When a benchmark transcript is published, seal it. A later PDF that says something else is a different hash. This is an improvement for a lab that is not under attack. It makes a quiet edit expensive.

**Receipts for self-hosted work.** Two people who run their own weights can pay each other in KAS and seal the result digest. The hosted API's account switch is untouched. A metered HTTP flow on Kaspa (the x402 experiments) was still a testnet release candidate in the September 2026 pin board. Do not describe it as a Grok checkout.

Prompts, completions, and API keys stay off the chain. A public ledger is a permanent prompt log.

## 3. While miners mine

A weight digest and a published eval digest are not unpublished by deleting a repository or by a court order aimed at a host. The transaction has no account to suspend.

The hosted Grok product still has an off switch, and that switch is in the company, where the law can reach it. Customers who need the hosted product remain customers. People who only needed a hash of a public file do not.

## What this rail does not do

It does not train Grok, serve the API, or smuggle a banned customer's traffic back into the hosted product. It does not make a later, unpublished model into an open model.

## Detail

`grok-1` is a public file. Anyone who downloaded it can hash what they downloaded.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Xai,
    kaspa_seal::Kind::WeightOrImage,
    downloaded_weight_bytes,
);
```

The seal commits to those bytes. If the tag moves or the repository disappears, `payload_digest` of this payload is still the half a version-1 id committed to, for anyone who kept the payload or the transaction. It does not prove that a hosted endpoint loaded the same bytes. A hosted model is the company's process. The hash is the file the sealer had.

A published eval uses `Kind::Eval`. Two files, the transcript and a table, use `aux`:

```rust
let payload = kaspa_seal::encode_seal(&kaspa_seal::Seal {
    branch: kaspa_seal::Branch::Xai,
    kind: kaspa_seal::Kind::Eval,
    primary: kaspa_seal::document_hash(transcript),
    aux: Some(kaspa_seal::document_hash(table)),
});
```

Prompts, completions, and API keys are not those arguments. A public payload is a permanent log. `OpTxPayloadSubstr` (`0xb8`) exists so a script can read the payload. It is not a secrecy feature.

The September 2026 pin board's x402 release candidate is not a Grok checkout. This crate does not speak HTTP and does not call an API. Self-hosted parties who want to pay each other in KAS can use the escrow against a result hash. The hosted account switch remains the company's. [KIP-16](https://github.com/kaspanet/kips/blob/master/kip-0016.md) verifies two proof tags. It does not serve the model. The encoder notes are [here](../code/README.md).
