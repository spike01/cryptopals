use std::{num::ParseIntError};

const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    println!("{:?}", encode_base64(decode_hex("CAFEBABE").unwrap()));
}

fn encode_base64(bytes: Vec<u8>) -> &'static str {
    let alphabet = BASE64_ALPHABET.as_bytes();
    println!("bytes: {:?}", bytes);
    bytes
      .chunks(3) // use 3 bytes so we can split into 4 6-bit parts
                 // e.g. 01010101 01010101 01010101 -> 85 85 85
                 //     010101 010101 010101 010101 -> 21 21 21 21 -> VVVV
      .for_each(|i| {
        println!("{:?}", i);
      });

      "hello"
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_to_base64_small() {
        let input = "555555";
        let output = "VVVV";

        assert_eq!(encode_base64(decode_hex(input).unwrap()), output);
    }

    #[test]
    fn hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(encode_base64(decode_hex(input).unwrap()), output);
    }

    #[test]
    fn hex_to_binary() {
        let input = "0f269300";
        let output = [15, 38, 147, 0];

        assert_eq!(decode_hex(input).unwrap(), output);
    }

    #[test]
    fn hex_to_binary_small() {
        let input = "555555";
        let output = [85, 85, 85];

        assert_eq!(decode_hex(input).unwrap(), output);
    }
}
