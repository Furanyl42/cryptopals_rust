// src/scoring.rc

#![allow(dead_code)]

pub const ENGLISH_FREQ_27: [f64; 27] = [
    0.07105, 0.01298, 0.02420, 0.03700, 0.11051, 0.01938, 0.01753, 0.05302, 0.06060, 0.00133,
    0.00672, 0.03502, 0.02093, 0.05872, 0.06531, 0.01678, 0.00083, 0.05209, 0.05504, 0.07879,
    0.02400, 0.00851, 0.02053, 0.00131, 0.01717, 0.00064, 0.13000,
];

/// Checks if buffer contains printable ASCII or formatting bytes
pub fn is_valid_text(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .all(|&b| matches!(b, 32..=126 | b'\n' | b'\r' | b'\t'))
}

/// Calculates frequency of letters and space
pub fn calc_freq(bytes: &[u8]) -> [f64; 27] {
    let mut counts = [0f64; 27];
    if bytes.is_empty() {
        return counts;
    }

    for &b in bytes {
        match b {
            b'a'..=b'z' => counts[(b - b'a') as usize] += 1.0,
            b'A'..=b'Z' => counts[(b - b'A') as usize] += 1.0,
            b' ' => counts[26] += 1.0,
            _ => {}
        }
    }

    let total = bytes.len() as f64;
    for c in counts.iter_mut() {
        *c /= total;
    }
    counts
}

/// Calculates Bhattacharyya coefficient
pub fn bhattacharyya_coef(p: &[f64; 27], q: &[f64; 27]) -> f64 {
    p.iter()
        .zip(q.iter())
        .map(|(&pi, &qi)| (pi * qi).sqrt())
        .sum()
}
