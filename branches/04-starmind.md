# Starmind

Starmind is the name used in 2026 for a planned orbital compute constellation, distinct from Starlink broadband. Public reporting describes prototypes aimed at 2027 and a much larger build after that. No Starmind software repository was found. Nothing is flying under this note's evidence.

The problems on the critical path are power, heat, radiation, laser links, and launch mass. A terrestrial blockDAG does not run inference in orbit.

## 1. Exposure

The future switches are ordinary and geographic. A ground portal can be hosted in one country and ordered offline. An export rule can hold accelerators or a satellite. A cloud account can be the only place a customer submits work. A model-weight file can be swapped on a server, and the customer learns about it when the server says so.

None of those exist as an incident log yet. They are the shape of the system being described: many satellites, one operator, customers who are not that operator.

## 2. Worth exploring

**Weight-file seals.** When a model hash is fit to publish, seal it. [grok-1](https://github.com/xai-org/grok-1) is already a public file. A later orbital image that claims "this hash" can be checked against a digest that was ordered before the launch campaign, not against the operator's current website. The weights themselves stay where they are. The chain stores 32 bytes.

**Job receipts without prompts.** A customer and a provider can seal "request id H was accepted" and later "result digest R was published." The prompt and the completion stay off-chain. If a portal in one country is blocked, the receipt that the work was accepted is still on the rail. Delivery of the result still needs a path the customer can reach. The rail is the receipt, not the downlink.

**Lane per program.** [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) already separates application lanes so a proof follows one program's activity. A Starmind receipt stream, if anyone builds it, should not share a proof with a battery-passport stream. The lane mechanism is live. The Starmind program is not.

vProgs would be the step where the receipt is a proved state transition rather than a bare hash. vProgs has no release. Design the hash now. Wait for a tag before calling it a computer.

## 3. While miners mine

A published weight hash and a job receipt keep their order if a ground website is seized or blocked. Miners do not know which company the hash belongs to, and they do not need to.

The satellite still needs a license, a launch, and a radio. A government that can compel SpaceX can still order the operator to stop radiating or to stop accepting customers. The chain continues. The service is the company's.

## What this rail does not do

It does not train models, cool chips in vacuum, or move bits over inter-satellite lasers. It does not put customer prompts on a public ledger.
