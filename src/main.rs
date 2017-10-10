extern crate data_encoding;

use data_encoding::{BASE64, HEXLOWER};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;

fn main() {
    set1ch1();
    set1ch2();
    set1ch3();
    set1ch4();
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
    println!("{:?}", vec_to_string(decode_by_space_most_common(input)));
}

fn set1ch4() {
    let filename = "/home/whiteavian/Downloads/4.txt";
    let file = File::open(filename).expect("File not found.");
    let buf = BufReader::new(file);
    let inputs:Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect();

//    Any strings that have most common letters from the top 9 in English are finalists.
    let mut finalists:Vec<String> = Vec::new();

    for input in inputs {
        let line_attempt = decode_by_space_most_common(&input);
        if likely_english(&line_attempt) {
            finalists.push(vec_to_string(line_attempt));
        };
    }
    println!("{:?}", finalists);
}

fn likely_english(input: &Vec<u8>) -> bool {
    let top_letters: HashSet<u8> =
        vec![" ", "e", "t", "a", "o", "i", "n", "s", "h", "r"]
            .iter().map(|a| a.as_bytes()[0]).collect();

    let bc = sorted_byte_counts(&input);
    let mut top_letters_common = true;

    for i in 1..5 {
        if !top_letters.contains(&bc[i].0) {
            top_letters_common = false;
        }
    }
    top_letters_common
}

/// Return the human readable string from the given byte vector.
fn vec_to_string(input: Vec<u8>) -> String {
    String::from_utf8(input).unwrap_or(String::new())
}

/// Decode a string by a single key xor assuming that the most common character corresponds to
/// space.
fn decode_by_space_most_common(input: &str) -> Vec<u8> {
    let bytes = hex_to_bytes(input);

    let bytes_vector = sorted_byte_counts(&bytes);
    let top_letter = bytes_vector[0].0;
    let key = top_letter ^ " ".as_bytes()[0];

    let mut xor_result = Vec::new();
    for byte in &bytes {
        xor_result.push(byte ^ key);
    }

    xor_result
}

/// Return possible keys if the most frequent letter is in the top 9.
fn possible_keys(top_u8: &u8) -> Vec<u8> {
    let top_letters: Vec<&str> = vec![" ", "e", "t", "a", "o", "i", "n", "s", "h", "r"];
    let mut keys = Vec::new();

    for l in top_letters {
        keys.push(top_u8 ^ l.as_bytes()[0]);
    }

    keys
}

/// Return the frequency of each byte from a vector.
fn sorted_byte_counts(bytes: &Vec<u8>) -> Vec<(u8, u8)> {
    let mut char_counts:BTreeMap<u8, u8> = BTreeMap::new();

    for byte in bytes {
        *char_counts.entry(*byte).or_insert(1) += 1;
    }

    let mut bytes_vector = Vec::from_iter(char_counts);
    bytes_vector.sort_by(|a, b| b.1.cmp(&a.1));

    bytes_vector
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
