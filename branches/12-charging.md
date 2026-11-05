# Charging

Superchargers and destination chargers bill a session to a Tesla account, a card, or a roaming partner. Roaming adds more companies (the roaming hub, the other network, the card acquirer). Any one of them can suspend a token. The contactor in the stall is hardware the site operator controls. It is not a transaction output.

## 1. Exposure

A driver can have a working car and a dead payment token because a card network, an app store, or a roaming partner closed a category or a country. Tesla can also close an account. Both are operational interruptions, and they are different. The first is a vendor Tesla also depends on. The second is Tesla's own switch, which a government can compel and which Tesla uses for fraud and safety.

Session history lives in the operator's ledger. A dispute about a idle fee or a failed session is a dispute about that ledger.

## 2. Worth exploring

**A second payer path where both sides want it.** A stall operator who is not required to use a particular acquirer, and a driver who holds KAS, can settle a session in KAS. A practical shape: the stall signs a session receipt (energy, time, price agreed), the driver pays, a covenant releases a deposit when the receipt hash matches. Third-party and workplace chargers are the natural place. Tesla's own stalls can keep the account system, which also does idle fees, membership, and abuse control.

This is worth exploring before it is an outage. Card dependence is structural. It does not require a crisis in a particular country to be a single switch.

**Session seals for the stalls Tesla already operates.** Hash the receipt the driver already sees. The driver keeps a copy that outlives a closed account's access to the portal. The stall's decision to open the contactor stays local and instant. Do not put the authorization in the block time. Ten blocks a second is still the wrong loop for a contactor.

## 3. While miners mine

A finished KAS payment is not a chargeback, and no card scheme can freeze it. A sealed receipt is not deleted with the account portal.

The stall can still refuse the session. The operator can still open the breaker. A sanction or a license condition that binds the operator still binds the operator. The rail is the settlement and the receipt, after two parties have chosen it.

## What this rail does not do

It does not energize a cable, replace Plug&Charge certificates, or let a driver skip payment. It does not move Tesla's membership ledger onto a public chain.

## Detail

The session receipt the driver already sees is the hash. The contactor is not in the script.

```rust
let payload = kaspa_seal::seal_file(
    kaspa_seal::Branch::Charging,
    kaspa_seal::Kind::Receipt,
    session_receipt,
);
```

A deposit, where a stall operator and a driver both want KAS, is [`seal_escrow.sil`](../code/covenant/seal_escrow.sil) constructed with `document_hash(session_receipt)`. The compiled script requires `OpTxPayloadLen() == 81`, the magic at bytes 0..13, the version byte `0x01` at offset 13, and bytes 17..49 equal to that document hash. `OpTxPayloadSubstr` is opcode `0xb8` and the end index is exclusive, which the v2.1.0 node tests with `payload[1..4]`. The coins then move from this input to the buyer's or the seller's 36-byte pay-to-pubkey script. SilverScript v1.0.0 compiled this file. The artifact and the dummy constructor arguments are in [`code/covenant`](../code/covenant/README.md).

The script names a 1000 sompi fee because that is the upstream example. `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE` is 100_000 sompi per kilogram of mass, which is 100 sompi per gram. The relay fee is mass times 100_000, divided by 1000. 1000 sompi is that product at 10 grams. The script's own requirement is still the output amount. The script is a single-input example that trusts the arbiter. It checks `tx.outputs[0]` only and has no timeout. Release has no execution test in this repository.

Ten blocks a second, the figure in the lane-limit comment at [`config/constants.rs`](https://github.com/kaspanet/rusty-kaspa/blob/01b532e8b553523216471682649693af92f0fd16/consensus/core/src/config/constants.rs#L97-L98), is the wrong loop for a contactor. The stall decides locally. The receipt and the payment are after the session, or they are a deposit checked when the receipt exists. They are not the authorization bit in the charger.

Tesla's membership ledger, idle-fee policy, and abuse control stay in Tesla's account system. Third-party and workplace chargers are the place the escrow matches the easy page. The hash tests are in [the code notes](../code/BYTES.md).
