extern crate data_encoding;

use data_encoding::{BASE64, HEXLOWER};

const SET1CH1_TEST_IN: &str =
"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const SET1CH1_TEST_OUT: &str =
"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    set1ch1();
}

fn set1ch1() {
    let b64 = hex_to_base64(SET1CH1_TEST_IN);
    assert_eq!(b64, SET1CH1_TEST_OUT);
    println!("{:?}", b64);
    println!("Success!");
}

fn hex_to_base64(hex: &str) -> String {
    let bytes = HEXLOWER.decode(hex.as_bytes()).unwrap();
    BASE64.encode(&bytes)
}