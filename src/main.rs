// src/main.rs

mod encoding;
mod scoring;
mod utils;
mod xor;
use encoding::*;
use xor::*;

fn main() {
    //println!("Run 'cargo test' !");
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

    for i in 0..key_size {
        let transposed_block = input_bytes[i..].iter().copied().step_by(key_size);
        let (best_byte, _score) = find_best_single_byte_key(transposed_block);
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
