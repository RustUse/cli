#![allow(dead_code)]

//! Deterministic content hashing helpers for `.github` consistency checks.
//!
//! These hashes are used for drift detection and report output only. They are
//! not intended for cryptographic use.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// FNV-1a 64-bit offset basis.
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

/// FNV-1a 64-bit prime.
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Returns a deterministic 16-character lowercase hex hash for the provided bytes.
///
/// This intentionally avoids adding a hashing dependency for simple report drift
/// detection. The output shape matches RustUse's short variant hashes.
#[must_use]
pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut hash = FNV_OFFSET_BASIS;

    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    format!("{hash:016x}")
}

/// Returns a deterministic 16-character lowercase hex hash for the provided text.
#[must_use]
pub fn hash_str(text: &str) -> String {
    hash_bytes(text.as_bytes())
}

/// Reads a file and returns its deterministic 16-character lowercase hex hash.
pub fn hash_file(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read file for hashing `{}`", path.display()))?;

    Ok(hash_bytes(&bytes))
}

/// Compares expected text contents against an existing file by hash.
pub fn file_matches_contents(path: impl AsRef<Path>, expected_contents: &str) -> Result<bool> {
    let actual_hash = hash_file(path)?;
    let expected_hash = hash_str(expected_contents);

    Ok(actual_hash == expected_hash)
}

#[cfg(test)]
mod tests {
    use super::{hash_bytes, hash_str};

    #[test]
    fn hash_bytes_is_deterministic() {
        assert_eq!(hash_bytes(b"rustuse"), hash_bytes(b"rustuse"));
    }

    #[test]
    fn hash_str_matches_hash_bytes() {
        assert_eq!(hash_str("rustuse"), hash_bytes(b"rustuse"));
    }

    #[test]
    fn hash_output_is_16_hex_chars() {
        let hash = hash_str("rustuse");

        assert_eq!(hash.len(), 16);
        assert!(hash.chars().all(|character| character.is_ascii_hexdigit()));
    }
}
