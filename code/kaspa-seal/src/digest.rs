//! Keyed BLAKE3 payload digest, byte-for-byte with rusty-kaspa v2.1.0.
//!
//! Source of the construction, tag `v2.1.0`, commit `01b532e8b553523216471682649693af92f0fd16`:
//! - `crypto/hashes/src/hashers.rs` `blake3_hasher!` builds a 32-byte key by copying the
//!   domain bytes and leaving the rest zero, then calls `Hasher::new_keyed`.
//! - The domain for this hasher is `b"PayloadDigest"`.
//! - `Hasher::hash` is one-shot: `default()`, `update`, `finalize`.
//! - `consensus/core/src/hashing/tx.rs` `payload_digest` returns that hash, and
//!   `id_v1` feeds it to `TransactionV1Id` together with the rest-of-transaction digest.
//!
//! The official `test_blake3_vectors` function does **not** reset the hasher between
//! cases. Later expected strings are hashes of the concatenation, not one-shot hashes
//! of each input. `test_zero_payload_digest` is the one-shot empty check.

/// Domain separator copied into the BLAKE3 key by the node macro.
pub const PAYLOAD_DIGEST_DOMAIN: &[u8] = b"PayloadDigest";

/// Empty-payload digest printed by the node test, and the bytes of `ZERO_PAYLOAD_DIGEST`.
pub const EMPTY_PAYLOAD_DIGEST_HEX: &str = "9c0ca2acb45e92ffe6ceb4ae29188b35c82d9676cdd3ce067fd6ccc30a9c4a38";

/// 32-byte keyed-BLAKE3 key: domain, then zeros. `blake3::KEY_LEN` is 32.
pub fn payload_digest_key() -> [u8; 32] {
    let mut key = [0u8; blake3::KEY_LEN];
    key[..PAYLOAD_DIGEST_DOMAIN.len()].copy_from_slice(PAYLOAD_DIGEST_DOMAIN);
    key
}

/// One-shot `PayloadDigest` of `payload`.
///
/// This is what `payload_digest` in `consensus/core/src/hashing/tx.rs` computes for a
/// non-empty payload, and also for an empty payload: the node stores the empty hash
/// as a constant and tests it equal to `PayloadDigest::hash([])`.
pub fn payload_digest(payload: &[u8]) -> [u8; 32] {
    *blake3::keyed_hash(&payload_digest_key(), payload).as_bytes()
}

/// Unkeyed BLAKE3 of an off-chain file.
///
/// This is not `PayloadDigest`. The file hash is a field inside the seal. The node
/// then keyed-hashes the whole seal when it builds a version-1 transaction id.
pub fn document_hash(file: &[u8]) -> [u8; 32] {
    *blake3::hash(file).as_bytes()
}

pub fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0xf) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use blake3::Hasher;

    fn keyed_hasher() -> Hasher {
        Hasher::new_keyed(&payload_digest_key())
    }

    fn official_inputs() -> [Vec<u8>; 5] {
        [
            vec![],
            vec![1],
            vec![
                5, 199, 126, 44, 71, 32, 82, 139, 122, 217, 43, 48, 52, 112, 40, 209, 180, 83, 139, 231, 72, 48, 136, 48, 168,
                226, 133, 7, 60, 4, 160, 205,
            ],
            vec![42; 64],
            vec![0; 8],
        ]
    }

    /// Expected strings from `test_blake3_vectors` for `PayloadDigest::new`, in order.
    /// The hasher is not reset between them.
    const INCREMENTAL: [&str; 5] = [
        "9c0ca2acb45e92ffe6ceb4ae29188b35c82d9676cdd3ce067fd6ccc30a9c4a38",
        "a69883bfb2bb000249639d8bbf5019fd08ceb7daa7a86f170530522dfedba124",
        "04d7abe0495ff2c520f93ed31bb37f58a9375abb48d50b163864335af8d834de",
        "5a4aa21c7235ec3edb7bd198b576f78cca6c5027dd7938ed41e74ae982858021",
        "d6e0e23e9130b0e44734f3639489e22e7f7d928f474386599db6fca3b02d6295",
    ];

    #[test]
    fn key_is_domain_then_zeros() {
        let key = payload_digest_key();
        assert_eq!(blake3::KEY_LEN, 32);
        assert_eq!(&key[..PAYLOAD_DIGEST_DOMAIN.len()], PAYLOAD_DIGEST_DOMAIN);
        assert!(key[PAYLOAD_DIGEST_DOMAIN.len()..].iter().all(|b| *b == 0));
    }

    #[test]
    fn keyed_hash_matches_hasher_update() {
        let payload = b"order:1337";
        let mut hasher = keyed_hasher();
        hasher.update(payload);
        assert_eq!(hasher.finalize().as_bytes(), &payload_digest(payload));
    }

    #[test]
    fn empty_payload_matches_node_constant() {
        assert_eq!(to_hex(&payload_digest(&[])), EMPTY_PAYLOAD_DIGEST_HEX);
        assert_eq!(INCREMENTAL[0], EMPTY_PAYLOAD_DIGEST_HEX);
    }

    #[test]
    fn incremental_vectors_match_the_node_test_harness() {
        let inputs = official_inputs();
        let mut hasher = keyed_hasher();
        for (input, expected) in inputs.iter().zip(INCREMENTAL) {
            hasher.update(input);
            let got = to_hex(hasher.clone().finalize().as_bytes());
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn one_shot_is_not_the_later_incremental_vectors() {
        let inputs = official_inputs();
        // An empty update does not change BLAKE3 state, so the second incremental
        // vector is also the one-shot hash of `[1]`. The third vector has already
        // absorbed `[1]` plus the 32-byte sample. Treating that string as the
        // one-shot of the sample is wrong.
        assert_eq!(to_hex(&payload_digest(&inputs[1])), INCREMENTAL[1]);
        assert_ne!(to_hex(&payload_digest(&inputs[2])), INCREMENTAL[2]);
        assert_ne!(to_hex(&payload_digest(&inputs[3])), INCREMENTAL[3]);
        assert_ne!(to_hex(&payload_digest(&inputs[4])), INCREMENTAL[4]);
    }

    #[test]
    fn document_hash_is_a_different_function() {
        let file = b"public mishap note";
        assert_ne!(document_hash(file), payload_digest(file));
    }
}
