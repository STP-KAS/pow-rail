# Seal escrow

`seal_escrow.sil` is a covenant in the language of [SilverScript v1.0.0](https://github.com/kaspanet/silverscript/releases/tag/v1.0.0) (commit `3ed973335b59269293564805cc2c58a14595ec03`).

The workspace version in that tag is `1.0.0`. The compiler's own `COMPILER_VERSION` string, in `silverscript-lang/src/compiler/mod.rs`, is `"0.1.0"`. The pragma on this file is `^0.1.0` because that is the version the compiler compares. The artifact field `compiler_version` is therefore `0.1.0` when the tag v1.0.0 compiler produces it.

## What was compiled

`silverc` from that tag accepted `seal_escrow.sil` with [`example-args.json`](example-args.json). The arguments are dummy bytes: 32 times `0x01` for the arbiter hash, a counting `0x00..0x1f` pubkey for both parties, and 32 times `0x11` for the document hash. They are not keys.

The compiler wrote [`seal_escrow.artifact.json`](seal_escrow.artifact.json). Its `template_hash` is

```text
21a73ab3317a05c68a646aea788380dfd29662f94f048ea7a20ed5765913bd37
```

In the bytecode, the magic check is the bytes `00 5d b8 0d` followed by `pow-rail/seal`. `0x00` is `Op0` (start index 0). `0x5d` is the small-integer opcode for 13, because `Op1` is `0x51` and the builder emits `(Op1 - 1) + n` for n in 1..=16 (`crypto/txscript/src/script_builder.rs`, `add_i64`, tag v2.1.0). `0xb8` is `OpTxPayloadSubstr`. `0x0d` is the data-push length of the 13-byte magic. The slice and the string are the same length.

## What the script requires

The spend entry `release(pubkey pk, sig s)` requires all of the following.

The revealed pubkey hashes to the arbiter byte string stored at construction, with `blake2b`, and the signature checks against that pubkey. This is the same shape as `silverscript-lang/tests/examples/covenant_escrow.sil` in the v1.0.0 tag.

`OpTxPayloadLen` is 81. `OpTxPayloadSubstr(0, 13)` is the ASCII magic. `OpTxPayloadSubstr(13, 14)` is the single byte `0x01` (seal version 1). `OpTxPayloadSubstr(17, 49)` equals the 32-byte document hash stored at construction.

The end index is exclusive. rusty-kaspa v2.1.0 tests `OpTxPayloadSubstr` with arguments `1` and `4` against `payload[1..4]` (`crypto/txscript/src/opcodes/mod.rs`, opcode `0xb8`).

The coin amount comes from `tx.inputs[this.activeInputIndex]`. The script requires that value to be greater than the named fee of 1000 sompi, then requires `tx.outputs[0].value` to equal the difference. The output lock is the 36-byte pay-to-pubkey script of the buyer or of the seller. The compiler builds that script as version bytes, `OpData32`, the 32-byte pubkey, and `OpCheckSig` (`silverscript-lang/src/compiler/compile/expression.rs` in the same tag). 2 + 1 + 32 + 1 = 36.

The script does not call `readInputState`. It does not read any other input's covenant state.

## What the script leaves to the signer

Branch and kind sit at payload offsets 14 and 15. The script does not compare them. The arbiter sees the whole payload before producing a signature, and can refuse a seal whose branch byte is wrong. The script's job is narrower: a signature cannot move the coins onto a payload that names a different document hash, a different magic, or a different seal version.

The 1000 sompi figure is the upstream example's fee. The v2.1.0 mempool constant `DEFAULT_MINIMUM_RELAY_TRANSACTION_FEE` is 100_000 sompi (`mining/src/mempool/config.rs`). A real spend has to satisfy whatever fee the mempool and the miners require. The covenant names its own output amount. It does not ask the compiler to invent one.

## Reproduce

From a checkout of `kaspanet/silverscript` at tag `v1.0.0`:

```bash
cargo run -p silverscript-lang --bin silverc -- \
  seal_escrow.sil --constructor-args example-args.json -o seal_escrow.artifact.json
```

The encoder that builds the 81 payload bytes is [`../kaspa-seal`](../kaspa-seal). The reasoning is in [`../README.md`](../README.md).
