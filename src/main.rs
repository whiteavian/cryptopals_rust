extern crate data_encoding;

use data_encoding::{BASE64, HEXLOWER};
use std::collections::BTreeMap;
use std::iter::FromIterator;

fn main() {
    set1ch1();
    set1ch2();
    set1ch3();
}

fn set1ch1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(hex_to_base64(input), expected);
    println!("Success on Set 1 Challenge 1!");
}

fn hex_to_base64(hex: &str) -> String {
    let bytes = HEXLOWER.decode(hex.as_bytes()).unwrap();
    BASE64.encode(&bytes)
}

fn set1ch2() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965"; 
    let expected = "746865206b696420646f6e277420706c6179";
    assert_eq!(xor(hex_to_bytes(input1), hex_to_bytes(input2)), hex_to_bytes(expected));
    println!("Success on Set 1 Challenge 2!");
}

fn set1ch3() {
    let input= "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex_to_bytes(input);

//    for i in 65..123 {
//        let mut xor_result: Vec<u8> = Vec::new();
//        for byte in &bytes {
//            xor_result.push(byte ^ i);
//        }
//        let bc = byte_counts(xor_result);
//
//        println!("BEGIN NEW ATTEMPT");
//        println!("{:?}", i);
//        for byte in bc {
//            println!("{:?} {:?}", byte.0 as char, byte.1);
//        }
//    }

    let mut v = Vec::from_iter(byte_counts(bytes));
    v.sort_by(|a, b| b.1.cmp(&a.1));
    for byte in v {
        println!("{:?} {:?}", byte.0 as char, byte.1);
    }

}

/// Return the frequency of each byte from a vector.
fn byte_counts(bytes: Vec<u8>) -> BTreeMap<u8, u8> {
    let mut char_counts:BTreeMap<u8, u8> = BTreeMap::new();

    for byte in &bytes {
        *char_counts.entry(*byte).or_insert(1) += 1;
    }

    char_counts
}

/// Xor two equivalent length vectors.
fn xor(vec1 : Vec<u8>, vec2 : Vec<u8>) -> Vec<u8> {
    assert_eq!(vec1.len(), vec2.len());
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x , y)| x ^ y)
        .collect()
}

/// Return the raw bytes associate with a hex encoded string.
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    HEXLOWER.decode(hex.as_bytes()).unwrap()
}
