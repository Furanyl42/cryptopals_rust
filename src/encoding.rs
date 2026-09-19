#![allow(unused)]

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// Converts ASCII hex char to its numerical value
pub fn ascii_to_hex_digit(b: u8) -> Result<u8, &'static str> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err("Invalid hex char"),
    }
}

/// Decodes hex string to a vector of raw bytes
pub fn hex_to_bytes(s: &str) -> Result<Vec<u8>, &str> {
    if !s.len().is_multiple_of(2) {
        return Err("Hex string length must be even");
    }
    s.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let high = ascii_to_hex_digit(chunk[0])?;
            let low = ascii_to_hex_digit(chunk[1])?;
            Ok(high << 4 | low)
        })
        .collect()
}

/// Converts a byte to bits
pub fn byte_to_bits(b: u8) -> [u8; 8] {
    let mut bits = [0u8; 8];
    for i in 0..8 {
        bits[7 - i] = (b >> i) & 1;
    }
    bits
}

/// Recomposes a chunk of bits (6 bits) to integer value
pub fn bits_to_u8(bits: &[u8]) -> u8 {
    bits.iter().fold(0, |acc, &bit| (acc << 1) | bit)
}

/// Calculates result of XOR combination between 2 vectors of raw bytes
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

/// Encodes vector of raw bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        hex.push(HEX_CHARS[(b >> 4) as usize] as char);
        hex.push(HEX_CHARS[(b & 0x0F) as usize] as char);
    }
    hex
}

/// Converts a chunk of raw butes to a Base64 string
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let mut bits: Vec<u8> = bytes.iter().flat_map(|&b| byte_to_bits(b)).collect();
    let remainder = bits.len() % 6;
    if remainder != 0 {
        bits.resize(bits.len() + (6 - remainder), 0);
    }
    let mut result: String = bits
        .chunks(6)
        .map(|chunk| {
            let idx = bits_to_u8(chunk) as usize;
            BASE64_ALPHABET[idx] as char
        })
        .collect();
    match remainder {
        2 => result.push_str("=="),
        4 => result.push('='),
        _ => (),
    }
    result
}

/// Converts hex to base64
pub fn hex_to_base64(hex: &str) -> Result<String, &str> {
    let raw_bytes = hex_to_bytes(hex)?;
    Ok(bytes_to_base64(&raw_bytes))
}

/// Calculates the Bhattacharyya coefficient
pub fn bhattacharyya_coef(p: &[f64], q: &[f64]) -> f64 {
    p.iter().zip(q.iter()).map(|(&a, &b)| (a * b).sqrt()).sum()
}

/// Calculates the frequency of each letter
pub fn calc_freq(input: &[u8]) -> [f64; 26] {
    let mut freq_in_order = [0.0f64; 26];

    if input.is_empty() {
        return freq_in_order;
    }

    for &b in input {
        if b.is_ascii_alphabetic() {
            let idx = (b.to_ascii_lowercase() - b'a') as usize;
            freq_in_order[idx] += 1.0;
        }
    }

    let total = input.len() as f64;
    freq_in_order.iter_mut().for_each(|freq| *freq /= total);

    freq_in_order
}

/// Converts bytes to ascii
pub fn bytes_to_ascii(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| {
            if is_printable_ascii(b) {
                b as char
            } else {
                '.'
            }
        })
        .collect()
}

/// Checks whether a byte is printable ascii
pub fn is_printable_ascii(byte: u8) -> bool {
    (32..=126).contains(&byte)
}

/// Struct to store a decrypted message with its key and coefficient
pub struct Cracked {
    message: Vec<u8>,
    key: Vec<u8>,
    coef: f64,
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

/// Decrypts input message (single-bute XOR'ed)  and finds key based on character frequency using Bhattacharyya coefficient
pub fn single_byte_crack(input_bytes: &[u8]) -> Cracked {
    let english_freq: [f64; 26] = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
        0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
        0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
    ];
    let mut best = Cracked::default();
    let mut temp_buffer = vec![0u8; input_bytes.len()];
    for key in 0..=255u8 {
        for (i, &b) in input_bytes.iter().enumerate() {
            temp_buffer[i] = b ^ key;
        }
        let freq = calc_freq(&temp_buffer);
        let coef = bhattacharyya_coef(&freq, &english_freq);

        if coef > best.coef {
            best.coef = coef;
            best.key = vec![key];
            best.message = temp_buffer.clone();
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_1_hex_to_base64() {
        let input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let result = hex_to_base64(input_hex).expect("Hex decoding failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_challenge_2_fixed_xor() {
        let input1_hex = "1c0111001f010100061a024b53535009181c";
        let input2_hex = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let result = {
            let b1 = hex_to_bytes(input1_hex).expect("Hex decoding failed");
            let b2 = hex_to_bytes(input2_hex).expect("Hex decoding failed");

            //let mut out = vec![0u8; b1.len()];
            //fixed_xor(&b1, &b2, &mut out).expect("XOR failed");
            bytes_to_hex(&fixed_xor(&b1, &b2).unwrap())
        };
        assert_eq!(result, expected);
    }
    #[test]
    fn test_challenge_3_single_byte_xor_cipher() {
        let input_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let input_bytes = hex_to_bytes(input_hex).expect("Hex decoding failed");
        let result = single_byte_crack(&input_bytes);
        let plaintext = bytes_to_ascii(&result.message);

        println!("Clef: {:?}", result.key);
        println!("Nessage: {}", plaintext);
        println!("Score: {}", result.coef);

        assert_eq!(result.key, vec![b'X']);
        assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
    }
}
