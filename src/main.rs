// src/main.rs
#![allow(unused)]
mod aes;
mod encoding;
mod scoring;
mod utils;
mod xor;

use crate::encoding::*;
use cryptopals::utils::read_file_lines;
use openssl::symm::*;
use std::collections::HashSet;

fn main() {
    println!("Run 'cargo test' !");
}
