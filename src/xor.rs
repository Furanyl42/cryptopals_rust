// src/xor.rs

use crate::scoring::*;
use std::fmt;

/// Struct to store a decrypted message with its key and coefficient
#[derive(Debug)]
pub struct Cracked {
    pub message: Vec<u8>,
    pub key: Vec<u8>,
    pub coef: f64,
}

impl Default for Cracked {
    fn default() -> Self {
        Self {
            message: Vec::new(),
            key: Vec::new(),
            coef: -1.0,
        }
    }
}

impl fmt::Display for Cracked {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.message))
    }
}

/// Decrypts input message (single-byte XOR'ed)  and finds key based on character frequency using Bhattacharyya coefficient
pub fn single_byte_crack(input_bytes: &[u8]) -> Cracked {
    let mut best = Cracked::default();
    let mut temp_buffer = vec![0u8; input_bytes.len()];
    for key in 0..=255u8 {
        for (i, &b) in input_bytes.iter().enumerate() {
            temp_buffer[i] = b ^ key;
        }
        if !is_valid_text(&temp_buffer) {
            continue;
        }
        let freq = calc_freq(&temp_buffer);
        let coef = bhattacharyya_coef(&freq, &ENGLISH_FREQ_27);

        if coef > best.coef {
            best.coef = coef;
            best.key = vec![key];
            best.message = temp_buffer.clone();
        }
    }
    best
}
