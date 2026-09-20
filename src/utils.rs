// src/utils.rs

#![allow(dead_code)]

use crate::encoding::hex_to_bytes;
use std::fs::File;
use std::io::Read;

pub fn read_file_lines(path: &str) -> Vec<Vec<u8>> {
    let mut file = File::open(path).expect("Cant open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cant read file");
    let mut lines = Vec::new();

    for line in content.lines() {
        if let Ok(line_in_bytes) = hex_to_bytes(line) {
            lines.push(line_in_bytes);
        }
    }
    lines
}
