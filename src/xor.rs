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
            best.message = input_bytes.iter().map(|&b| b ^ key).collect();
        }
    }
    best
}

pub fn find_best_single_byte_key(input_bytes: &[u8]) -> (u8, f64) {
    let mut best_key = 0u8;
    let mut best_coef = -1.0;

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
        if coef > best_coef {
            best_coef = coef;
            best_key = key;
        }
    }

    (best_key, best_coef)
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

pub fn repeating_key_crack(input_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut score_list: Vec<(usize, f64)> = Vec::new();

    for key_size in 2..=40usize {
        let mut total_distance = 0u32;
        let mut count = 0u32;
        for pair in input_bytes.chunks_exact(key_size * 2) {
            let block1 = &pair[..key_size];
            let block2 = &pair[(key_size)..(key_size * 2)];

            total_distance += hamming(block1, block2);
            count += 1;
        }
        if count > 0 {
            let normalized_score = (total_distance as f64) / (count as f64 * key_size as f64);
            score_list.push((key_size, normalized_score));
        }
    }
    score_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let (key_size, _score) = score_list[0];
    let mut key = Vec::with_capacity(key_size);

    let mut column_buffer = Vec::new();

    for i in 0..key_size {
        column_buffer.clear();
        column_buffer.extend(input_bytes[i..].iter().copied().step_by(key_size));
        let (best_byte, _score) = find_best_single_byte_key(&column_buffer);
        key.push(best_byte);
    }
    let message = repeating_key_xor(input_bytes, &key);
    (message, key)
}

pub fn hamming(b1: &[u8], b2: &[u8]) -> u32 {
    //bits_to_u8(&b1.iter().zip(b2.iter()).map(|(a, b)| a ^ b).collect())
    //let in_bytes: Vec<u8> = b1.iter().zip(b2.iter()).map(|(a, b)| a ^ b).collect();
    //bits_to_u8(&in_bytes).count_ones()
    b1.iter()
        .zip(b2.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum()
}
