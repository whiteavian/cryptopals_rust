extern crate data_encoding;

use data_encoding::{BASE64, HEXLOWER};

const SET1CH1_TEST_IN: &str =
"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const SET1CH1_TEST_OUT: &str =
"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    set1ch1();
    set1ch2();
}

fn set1ch1() {
    let b64 = hex_to_base64(SET1CH1_TEST_IN);
    assert_eq!(b64, SET1CH1_TEST_OUT);
    println!("{:?}", b64);
    println!("Success on Set 1 Challenge 1!");
}

fn hex_to_base64(hex: &str) -> String {
    let bytes = HEXLOWER.decode(hex.as_bytes()).unwrap();
    BASE64.encode(&bytes)
}

fn set1ch2() {
    assert_eq!(xor(hex_to_bytes("1c0111001f010100061a024b53535009181c"), hex_to_bytes("686974207468652062756c6c277320657965")), hex_to_bytes("746865206b696420646f6e277420706c6179"));
    println!("Success on Set 1 Challenge 2!");
}

fn xor(vec1 : Vec<u8>, vec2 : Vec<u8>) -> Vec<u8> {
    assert_eq!(vec1.len(), vec2.len());
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x , y)| x ^ y)
        .collect()
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    HEXLOWER.decode(hex.as_bytes()).unwrap()
}
