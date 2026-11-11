# Starshield

Starshield is the government-oriented side of Starlink. The customer is a state. The point of the product, for that customer, is that a known operator can be directed.

## 1. Exposure

The exposure is real and it is the opposite of a permissionless rail. A government customer can require the ability to task, to audit, and to stop. A classified network that nobody can switch off is a network the customer will not buy.

Public Kaspa has no access control. Anything placed in a transaction is readable by every node. Classified tasking, user lists, and coverage orders do not belong there.

## 2. Worth exploring

Almost nothing inside the classified boundary.

Outside it, the same seal as any other contractor: a hash of an **unclassified** delivery record (a software version, a public interface spec, an acceptance signature) so the agency and the company are not each the only copy of "what was handed over." Use it only where the contract allows a public timestamp. Many contracts will not.

Do not explore "uncensorable tasking." That is a request to take a government system out of the government's hands. It is also a bad technical fit.

## 3. While miners mine

An unclassified digest, if one was broadcast, stays ordered with no vendor to delete it.

Starshield's service does not. Its resilience is spacecraft, spectrum, and a contract. Those have owners. Proof of work does not make a government constellation permissionless, and it should not.

## Dollar, still a draft

Classified tasking does not get a public unit. [Kaspa Dollar](../DOLLAR.md) does not change that. The only surface is an unclassified commercial invoice, if one exists and both sides want the draft unit instead of a bank dollar. A government customer can still be ordered, and the constellation still answers its operator. A public vault is a poor place for a program name that was not already public. The design is a draft. It is not a coin yet.

## Space watts

No lens of that post supports this branch. The customer wants an operator who can be directed. Tasking stays off the rail. The reading is in [WATTS.md](../WATTS.md).

## What this rail does not do

It does not carry classified traffic, keys, or tasking. It does not give a state a chain it can halt, and it does not give an outsider a chain that can retask the satellites.

## Detail

The only kind this branch uses is `UnclassifiedDelivery`.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Starshield,
    kaspa_seal::Kind::UnclassifiedDelivery,
    unclassified_record,
);
```

`unclassified_record` is a software version, a public interface spec, or an acceptance signature the contract already allows to be public. The function hashes it with unkeyed BLAKE3 and places that hash at payload bytes 17..49. A reader of the chain sees the hash. A reader who can guess a short input can test the guess. Classified tasking is not hashed "to keep it safe." It is not hashed.

There is no user lane for material inside the classified boundary, because that material is not an application on this rail. [KIP-21](https://github.com/kaspanet/kips/blob/master/kip-0021.md) lanes are subsets of accepted transactions. They are not access control. The header commitment is public. `OpTxPayloadSubstr` in the node (`0xb8` at v2.1.0) reads the payload of the spending transaction onto the script stack, which is the opposite of a confidential channel.

A government customer who needs to halt a service has the operator. `PayloadDigest` has no admin key. That is why the classified network itself does not move here, and why an unclassified delivery digest, if the contract allows one, has no vendor who can quietly replace it. The mechanism is the same text as the other branches: [code notes](../code/BYTES.md).
