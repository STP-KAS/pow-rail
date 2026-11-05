//! 20-byte subnetwork ids, the shape KIP-21 uses as a lane id.
//!
//! Normative text: KIP-21 section 2.1, read from `kaspanet/kips` master on 5 November 2026.
//! `lane_id(tx) = tx.subnetwork_id`, 20 bytes.
//!
//! - Native: one `0x00` followed by 19 zero bytes.
//! - Coinbase: one `0x01` followed by 19 zero bytes. This crate does not build that id.
//! - User lane: 4-byte namespace, then 16 zero bytes, and at least one of `namespace[1..=3]`
//!   is non-zero. Any other id whose last 19 bytes are zero is reserved and invalid.
//!
//! The node field is `Transaction::subnetwork_id` in `consensus/core/src/tx.rs`.
//! SilverScript v1.0.0 types the introspection opcode `OpTxSubnetId` as `byte[20]`
//! (`silverscript-lang/tests/compiler_tests.rs`, the subnet case).
//!
//! Forming the bytes is not the same as the node accepting a transaction. A user lane
//! also needs transaction version 1 (`TX_VERSION_TOCCATA` in `consensus/core/src/constants.rs`).

/// KIP-21 lane id width.
pub const LANE_ID_LEN: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaneError {
    pub namespace: [u8; 4],
}

impl std::fmt::Display for LaneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "namespace {:02x}{:02x}{:02x}{:02x} is native, coinbase, or reserved: KIP-21 requires a non-zero byte in namespace[1..=3]",
            self.namespace[0], self.namespace[1], self.namespace[2], self.namespace[3]
        )
    }
}

impl std::error::Error for LaneError {}

/// Native subnetwork. Twenty zero bytes. A seal that does not need its own prover uses this.
pub fn native_lane_id() -> [u8; LANE_ID_LEN] {
    [0u8; LANE_ID_LEN]
}

/// User-lane id. Fails when `namespace[1..=3]` are all zero.
pub fn user_lane_id(namespace: [u8; 4]) -> Result<[u8; LANE_ID_LEN], LaneError> {
    if namespace[1] == 0 && namespace[2] == 0 && namespace[3] == 0 {
        return Err(LaneError { namespace });
    }
    let mut id = [0u8; LANE_ID_LEN];
    id[..4].copy_from_slice(&namespace);
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_is_twenty_zeros() {
        let id = native_lane_id();
        assert_eq!(id.len(), 20);
        assert!(id.iter().all(|b| *b == 0));
    }

    #[test]
    fn user_lane_keeps_a_16_byte_zero_suffix() {
        let id = user_lane_id([b'P', b'R', 0x01, 0x00]).unwrap();
        assert_eq!(&id[..4], b"PR\x01\x00");
        assert!(id[4..].iter().all(|b| *b == 0));
    }

    #[test]
    fn zero_tail_namespaces_are_rejected() {
        assert!(user_lane_id([0, 0, 0, 0]).is_err());
        assert!(user_lane_id([1, 0, 0, 0]).is_err());
        assert!(user_lane_id([0x50, 0, 0, 0]).is_err());
    }

    #[test]
    fn a_nonzero_byte_in_the_last_three_is_enough() {
        assert!(user_lane_id([0x00, 0x00, 0x00, 0x01]).is_ok());
        assert!(user_lane_id([0xff, 0x00, 0x01, 0x00]).is_ok());
    }
}
