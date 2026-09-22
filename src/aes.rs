// src/aes.rs

use std::collections::HashSet;

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
