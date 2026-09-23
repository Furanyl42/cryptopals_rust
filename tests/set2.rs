// tests/set2.rs
use cryptopals::aes::*;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_challenge_9_implement_pkcs7_padding() {
        let input = b"YELLOW SUBMARINE";
        let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        let result = pkcs_7_padding(input, 20);
        assert_eq!(result, expected);
    }
}
