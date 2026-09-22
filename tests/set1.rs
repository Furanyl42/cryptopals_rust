// tests/set1.rs

use cryptopals::aes::*;
use cryptopals::encoding::*;
use cryptopals::utils::*;
use cryptopals::xor::*;

use openssl::symm::{Cipher, decrypt};
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_1_convert_hex_to_base64() {
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

    #[test]
    fn test_challenge_4_detect_single_character_xor() {
        let expected = "Now that the party is jumping\n";
        let file = "tests/4.txt";
        let encrypted_lines = read_file_lines(file);
        let mut cracked_best = Cracked::default();
        for line in encrypted_lines {
            let current_cracked = single_byte_crack(&line);
            if current_cracked.coef > cracked_best.coef {
                cracked_best.coef = current_cracked.coef;
                cracked_best.message = current_cracked.message;
                cracked_best.key = current_cracked.key;
            }
        }
        let plaintext = String::from_utf8_lossy(&cracked_best.message);

        assert_eq!(plaintext, expected);
    }

    #[test]
    fn test_challenge_5_implement_repeating_key_xor() {
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let message = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let result = repeating_key_xor(message.as_bytes(), b"ICE");

        let plaintext = bytes_to_hex(&result);
        assert_eq!(plaintext, expected);
    }

    #[test]
    fn test_challenge_6_break_repeating_key_xor() {
        let expected_key = "Terminator X: Bring the noise";
        let file = "tests/6.txt";
        let file_content = fs::read_to_string(file).expect("Cant read file");
        let encrypted_bytes = base64_to_bytes(&file_content);
        let (_, result_key) = repeating_key_crack(&encrypted_bytes);
        let plaintext = String::from_utf8_lossy(&result_key);

        assert_eq!(plaintext, expected_key);
    }

    #[test]
    fn test_challenge_7_aes_in_ecb_mode() {
        let base64_content = fs::read_to_string("tests/7.txt").expect("Cant read file");
        let decoded = base64_to_bytes(&base64_content);
        let key = b"YELLOW SUBMARINE";

        let cipher = Cipher::aes_128_ecb();
        let Ok(result) = decrypt(cipher, key, None, &decoded) else {
            panic!("Cant decrypt");
        };
        let plaintext = String::from_utf8_lossy(&result);
        assert!(plaintext.starts_with("I'm back and I'm ringin' the bell"));
        assert!(plaintext.ends_with("Play that funky music \n"));
    }

    #[test]
    fn test_challenge_8_detect_aes_in_ecb_mode() {
        let file = "tests/8.txt";
        let hex_decoded = read_file_lines(file);
        let Some((idx, _)) = hex_decoded
            .into_iter()
            .enumerate()
            .find(|(_, line)| detect_ecb(line))
        else {
            panic!("No ECB");
        };
        assert_eq!(idx, 132);
    }
}
