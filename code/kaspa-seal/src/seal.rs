//! Fixed 80-byte seal carried in `Transaction::payload`.
//!
//! Layout, version 1. The magic is 13 bytes (`pow-rail/seal`), not 12.
//!
//! ```text
//! offset  len  field
//! 0       13   ASCII "pow-rail/seal"
//! 13      1    version = 1
//! 14      1    branch (1..=15)
//! 15      1    kind
//! 16      1    flags, bit 0 set when `aux` is present
//! 17      32   primary document hash (unkeyed BLAKE3)
//! 49      32   aux document hash, or 32 zero bytes
//! ```
//!
//! `code/covenant/seal_escrow.sil` requires length 81, the 13-byte magic,
//! version byte `0x01` at offset 13, and `payload[17..49]` equal to the
//! document hash. Those checks are `OpTxPayloadLen` and `OpTxPayloadSubstr`
//! (opcode `0xb8`). The end index is exclusive, as in rusty-kaspa v2.1.0.

use crate::digest::document_hash;

pub const MAGIC: &[u8; 13] = b"pow-rail/seal";
pub const VERSION: u8 = 1;
pub const SEAL_LEN: usize = 81;
pub const PRIMARY_OFFSET: usize = 17;
pub const AUX_OFFSET: usize = 49;
const AUX_FLAG: u8 = 0b0000_0001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Branch {
    Launch = 1,
    Starlink = 2,
    Starshield = 3,
    Starmind = 4,
    Starbase = 5,
    Terafab = 6,
    Xai = 7,
    X = 8,
    Vehicles = 9,
    Energy = 10,
    Optimus = 11,
    Charging = 12,
    Supply = 13,
    Compute = 14,
    Adjacent = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Kind {
    /// Hash of a file that is already public, or that the sealer is willing to hand to a verifier.
    PublicFile = 1,
    /// Hash of a receipt, a job acceptance, a fare, a session, or a meter row. Not the row itself.
    Receipt = 2,
    /// Hash of a published weight file, a firmware image, or a software build.
    WeightOrImage = 3,
    /// Hash of an inspection pack, a traveler, a calibration, or a shipment record.
    Inspection = 4,
    /// Hash of a published eval, system card, or notes run.
    Eval = 5,
    /// Hash of a battery or parts extract. Not a fungible token.
    PassportExtract = 6,
    /// Hash of an unclassified delivery record. Not a classified document.
    UnclassifiedDelivery = 7,
    /// Hash of an already-public status page or an already-public on/off notice.
    PublicStatus = 8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seal {
    pub branch: Branch,
    pub kind: Kind,
    /// Unkeyed BLAKE3 of the file the parties keep. Use [`document_hash`].
    pub primary: [u8; 32],
    /// Second file, when the claim names two hashes. `None` writes 32 zero bytes and clears the flag.
    pub aux: Option<[u8; 32]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SealError {
    Truncated { got: usize },
    BadMagic,
    BadVersion(u8),
    BadBranch(u8),
    BadKind(u8),
    /// Flag bit 0 was clear, but the aux field was not 32 zeros, or an unknown flag bit was set.
    AuxFlagMismatch,
}

impl std::fmt::Display for SealError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SealError::Truncated { got } => write!(f, "seal is {got} bytes, want {SEAL_LEN}"),
            SealError::BadMagic => write!(f, "seal magic is not pow-rail/seal"),
            SealError::BadVersion(v) => write!(f, "unsupported seal version {v}"),
            SealError::BadBranch(v) => write!(f, "unknown branch id {v}"),
            SealError::BadKind(v) => write!(f, "unknown kind id {v}"),
            SealError::AuxFlagMismatch => write!(f, "aux flag does not match the aux bytes"),
        }
    }
}

impl std::error::Error for SealError {}

impl Branch {
    pub fn from_u8(value: u8) -> Result<Self, SealError> {
        Ok(match value {
            1 => Branch::Launch,
            2 => Branch::Starlink,
            3 => Branch::Starshield,
            4 => Branch::Starmind,
            5 => Branch::Starbase,
            6 => Branch::Terafab,
            7 => Branch::Xai,
            8 => Branch::X,
            9 => Branch::Vehicles,
            10 => Branch::Energy,
            11 => Branch::Optimus,
            12 => Branch::Charging,
            13 => Branch::Supply,
            14 => Branch::Compute,
            15 => Branch::Adjacent,
            other => return Err(SealError::BadBranch(other)),
        })
    }
}

impl Kind {
    pub fn from_u8(value: u8) -> Result<Self, SealError> {
        Ok(match value {
            1 => Kind::PublicFile,
            2 => Kind::Receipt,
            3 => Kind::WeightOrImage,
            4 => Kind::Inspection,
            5 => Kind::Eval,
            6 => Kind::PassportExtract,
            7 => Kind::UnclassifiedDelivery,
            8 => Kind::PublicStatus,
            other => return Err(SealError::BadKind(other)),
        })
    }
}

pub fn encode_seal(seal: &Seal) -> [u8; SEAL_LEN] {
    let mut out = [0u8; SEAL_LEN];
    out[..MAGIC.len()].copy_from_slice(MAGIC);
    out[13] = VERSION;
    out[14] = seal.branch as u8;
    out[15] = seal.kind as u8;
    out[PRIMARY_OFFSET..AUX_OFFSET].copy_from_slice(&seal.primary);
    if let Some(aux) = seal.aux {
        out[16] = AUX_FLAG;
        out[AUX_OFFSET..].copy_from_slice(&aux);
    }
    out
}

pub fn decode_seal(bytes: &[u8]) -> Result<Seal, SealError> {
    if bytes.len() != SEAL_LEN {
        return Err(SealError::Truncated { got: bytes.len() });
    }
    if &bytes[..MAGIC.len()] != MAGIC.as_slice() {
        return Err(SealError::BadMagic);
    }
    if bytes[13] != VERSION {
        return Err(SealError::BadVersion(bytes[13]));
    }
    let flags = bytes[16];
    if flags & !AUX_FLAG != 0 {
        return Err(SealError::AuxFlagMismatch);
    }
    let mut aux_bytes = [0u8; 32];
    aux_bytes.copy_from_slice(&bytes[AUX_OFFSET..]);
    let aux_flag = flags & AUX_FLAG != 0;
    if !aux_flag && aux_bytes != [0u8; 32] {
        return Err(SealError::AuxFlagMismatch);
    }
    let mut primary = [0u8; 32];
    primary.copy_from_slice(&bytes[PRIMARY_OFFSET..AUX_OFFSET]);
    Ok(Seal {
        branch: Branch::from_u8(bytes[14])?,
        kind: Kind::from_u8(bytes[15])?,
        primary,
        aux: if aux_flag { Some(aux_bytes) } else { None },
    })
}

/// Hash `file` with unkeyed BLAKE3 and wrap it as a seal with no second document.
pub fn seal_file(branch: Branch, kind: Kind, file: &[u8]) -> [u8; SEAL_LEN] {
    encode_seal(&Seal { branch, kind, primary: document_hash(file), aux: None })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digest::{payload_digest, to_hex};

    fn sample() -> Seal {
        Seal {
            branch: Branch::Launch,
            kind: Kind::PublicFile,
            primary: document_hash(b"public mishap note"),
            aux: None,
        }
    }

    #[test]
    fn round_trip() {
        let seal = sample();
        let bytes = encode_seal(&seal);
        assert_eq!(decode_seal(&bytes).unwrap(), seal);
    }

    #[test]
    fn layout_the_covenant_checks() {
        let primary = [0x11; 32];
        let aux = [0x22; 32];
        let payload = encode_seal(&Seal { branch: Branch::Charging, kind: Kind::Receipt, primary, aux: Some(aux) });
        assert_eq!(payload.len(), 81);
        assert_eq!(&payload[0..13], b"pow-rail/seal");
        assert_eq!(payload[13], 1);
        assert_eq!(payload[14], Branch::Charging as u8);
        assert_eq!(payload[15], Kind::Receipt as u8);
        assert_eq!(payload[16], 1);
        assert_eq!(&payload[17..49], &primary);
        assert_eq!(&payload[49..81], &aux);
    }

    #[test]
    fn absent_aux_is_zeros_and_flag_clear() {
        let payload = encode_seal(&sample());
        assert_eq!(payload[16], 0);
        assert_eq!(&payload[49..81], &[0u8; 32]);
    }

    #[test]
    fn payload_digest_changes_when_the_file_changes() {
        let a = seal_file(Branch::Starlink, Kind::PublicFile, b"manifest-a");
        let b = seal_file(Branch::Starlink, Kind::PublicFile, b"manifest-b");
        assert_ne!(payload_digest(&a), payload_digest(&b));
        assert_ne!(to_hex(&payload_digest(&a)), to_hex(&document_hash(b"manifest-a")));
    }

    #[test]
    fn every_branch_encodes() {
        let kinds = [
            (Branch::Launch, Kind::PublicFile),
            (Branch::Starlink, Kind::PublicStatus),
            (Branch::Starshield, Kind::UnclassifiedDelivery),
            (Branch::Starmind, Kind::WeightOrImage),
            (Branch::Starbase, Kind::Inspection),
            (Branch::Terafab, Kind::Inspection),
            (Branch::Xai, Kind::Eval),
            (Branch::X, Kind::Eval),
            (Branch::Vehicles, Kind::WeightOrImage),
            (Branch::Energy, Kind::Receipt),
            (Branch::Optimus, Kind::Inspection),
            (Branch::Charging, Kind::Receipt),
            (Branch::Supply, Kind::PassportExtract),
            (Branch::Compute, Kind::Eval),
            (Branch::Adjacent, Kind::PublicFile),
        ];
        for (branch, kind) in kinds {
            let payload = encode_seal(&Seal { branch, kind, primary: [9; 32], aux: None });
            let back = decode_seal(&payload).unwrap();
            assert_eq!(back.branch, branch);
            assert_eq!(back.kind, kind);
        }
    }

    #[test]
    fn rejects_a_short_buffer_bad_magic_and_a_dirty_aux() {
        assert!(matches!(decode_seal(&[0; 8]), Err(SealError::Truncated { got: 8 })));
        let mut bad = encode_seal(&sample());
        bad[0] = b'X';
        assert_eq!(decode_seal(&bad), Err(SealError::BadMagic));
        let mut dirty = encode_seal(&sample());
        dirty[49] = 1;
        assert_eq!(decode_seal(&dirty), Err(SealError::AuxFlagMismatch));
        dirty[16] = 0x02;
        dirty[49] = 0;
        assert_eq!(decode_seal(&dirty), Err(SealError::AuxFlagMismatch));
    }
}
