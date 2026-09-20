// src/xor.rs

#![allow(dead_code)]

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

/// Calculates result of XOR operation between 2 vectors of raw bytes (same length)
pub fn fixed_xor(b1: &[u8], b2: &[u8]) -> Result<Vec<u8>, &'static str> {
    if b1.len() != b2.len() {
        return Err("Buffer lengths must match");
    }
    /*for ((&a, &b), target) in b1.iter().zip(b2).zip(out.iter_mut()) {
        *target = a ^ b;
    }
    Ok(())*/
    Ok(b1.iter().zip(b2.iter()).map(|(a, b)| a ^ b).collect())
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

/// Calculates result of XOR operation on input by repeating key
pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    /*let mut target = vec![0u8; input.len()];
    let key_len = key.len();
    for (i, &b) in input.iter().enumerate() {
        target[i] = b ^ key[i % key_len];
    }
    target*/
    input
        .iter()
        .zip(key.iter().cycle())
        .map(|(&a, &b)| a ^ b)
        .collect()
}
