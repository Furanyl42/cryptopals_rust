// src/aes.rs

use std::collections::HashSet;
/// Detects if bytes are encrypted with ECB mode
pub fn detect_ecb(bytes: &[u8]) -> bool {
    let (blocks, _) = bytes.as_chunks::<16>();
    let mut in_hash = HashSet::new();
    for b in blocks {
        if !in_hash.insert(b) {
            return true;
        }
    }
    false
}

/// Adjusts size to be multiple of blocksize
pub fn pkcs_7_padding(bytes: &[u8], blocksize: usize) -> Vec<u8> {
    let pad_len = blocksize - (bytes.len() % blocksize);
    let total = bytes.len() + pad_len;
    let mut padded = Vec::with_capacity(total);
    padded.extend_from_slice(bytes);
    padded.resize(total, pad_len as u8);

    padded
}
