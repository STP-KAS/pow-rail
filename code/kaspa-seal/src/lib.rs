//! Encoder for an 81-byte document seal, and the payload digest Kaspa version-1
//! transaction ids commit to.
//!
//! Experimental. Not advice. Not Kaspa core. Not a node and not a wallet.
//! This crate does not broadcast, does not open a socket, and does not take a key.
//!
//! The digest matches rusty-kaspa tag `v2.1.0` (`01b532e8b553523216471682649693af92f0fd16`).
//! The reasoning and the file links are in `code/BYTES.md` at the root of this repository.

mod digest;
mod lane;
mod seal;

pub use digest::{document_hash, payload_digest, payload_digest_key, to_hex, EMPTY_PAYLOAD_DIGEST_HEX, PAYLOAD_DIGEST_DOMAIN};
pub use lane::{native_lane_id, user_lane_id, LaneError, LANE_ID_LEN};
pub use seal::{
    decode_seal, encode_seal, seal_file, Branch, Kind, Seal, SealError, AUX_OFFSET, MAGIC, PRIMARY_OFFSET, SEAL_LEN, VERSION,
};

/// Seal `file` and return the digest a version-1 transaction id commits to, once the
/// wallet puts these exact bytes in `Transaction.payload`.
///
/// The returned digest is not the transaction id. `id_v1` hashes this digest together
/// with a second digest of the transaction body. See `code/BYTES.md`.
pub fn committed_payload_digest(branch: Branch, kind: Kind, file: &[u8]) -> [u8; 32] {
    payload_digest(&seal_file(branch, kind, file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_digest_is_stable_for_one_file() {
        let first = committed_payload_digest(Branch::Launch, Kind::PublicFile, b"rev A");
        let second = committed_payload_digest(Branch::Launch, Kind::PublicFile, b"rev A");
        let other = committed_payload_digest(Branch::Launch, Kind::PublicFile, b"rev B");
        assert_eq!(first, second);
        assert_ne!(first, other);
        assert_eq!(first.len(), 32);
    }
}
