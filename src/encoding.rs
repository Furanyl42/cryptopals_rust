// src/encoding.rs

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

pub fn base64_to_bytes(input: &str) -> Vec<u8> {
    let clean_bytes: Vec<u8> = input
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r' && b != b' ' && b != b'=')
        .collect();

    let mut result = Vec::with_capacity((clean_bytes.len() * 3) / 4);

    let (chunks, remainder) = clean_bytes.as_chunks::<4>();

    for chunk in chunks {
        let b0 = base64_char_to_bits(chunk[0]);
        let b1 = base64_char_to_bits(chunk[1]);
        let b2 = base64_char_to_bits(chunk[2]);
        let b3 = base64_char_to_bits(chunk[3]);

        let byte1 = (b0 << 2) | (b1 >> 4);
        let byte2 = ((b1 & 0x0F) << 4) | (b2 >> 2);
        let byte3 = ((b2 & 0x03) << 6) | b3;

        result.push(byte1);
        result.push(byte2);
        result.push(byte3);
    }

    if remainder.len() == 2 {
        let b0 = base64_char_to_bits(remainder[0]);
        let b1 = base64_char_to_bits(remainder[1]);
        result.push((b0 << 2) | (b1 >> 4));
    } else if remainder.len() == 3 {
        let b0 = base64_char_to_bits(remainder[0]);
        let b1 = base64_char_to_bits(remainder[1]);
        let b2 = base64_char_to_bits(remainder[2]);
        result.push((b0 << 2) | (b1 >> 4));
        result.push(((b1 & 0x0F) << 4) | (b2 >> 2));
    }

    result
}

pub fn base64_char_to_bits(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => 0,
    }
}

/// Converts hex to base64
pub fn hex_to_base64(hex: &str) -> Result<String, &str> {
    let raw_bytes = hex_to_bytes(hex)?;
    Ok(bytes_to_base64(&raw_bytes))
}

/// Converts bytes to ascii
pub fn bytes_to_ascii(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| {
            if is_printable_ascii(b) {
                b as char
            } else {
                '?'
            }
        })
        .collect()
}

/// Checks whether a byte is printable ascii
pub fn is_printable_ascii(byte: u8) -> bool {
    (32..=126).contains(&byte)
}
