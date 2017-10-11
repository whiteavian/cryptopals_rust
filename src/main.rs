extern crate data_encoding;
extern crate hamming;

use data_encoding::{BASE64, HEXLOWER};
use hamming::distance;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    set1ch1();
    set1ch2();
    set1ch3();
    set1ch4();
    set1ch5();
    set1ch6();
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

fn set1ch5() {
    let input =
        "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let key = "ICE";
    let result = repeat_xor_encrypt(input, key);

    let expected =
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    assert_eq!(result, hex_to_bytes(expected));
    println!("Success on Set 1 Challenge 5!");
}

fn set1ch6() {
    let in1 = "this is a test";
    let in2 = "wokka wokka!!!";
    println!("{:?}", distance(in1.as_bytes(), in2.as_bytes()));

    let filename = "/home/whiteavian/Downloads/6.txt";
    let file = File::open(filename).expect("File not found.");
    let buf = BufReader::new(file);
    let inputs:Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect();

    let mut input_bytes = Vec::new();
    for line in inputs {
        input_bytes.append(&mut BASE64.decode(line.as_bytes()).unwrap());
    }

    let mut key_length_distances = BTreeMap::new();

    let max_average = 4;

    for i in 2..41 {
        let mut first: Vec<Vec<u8>> = Vec::new();

        for m in 0..max_average {
            for j in m * i..(m + 1) * i {
                if first.len() < m + 1 {
                    first.push(Vec::new());
                }
                first[m].push(input_bytes[j]);
            }
        }

        let mut sum = 0;
        for m in 0..max_average {
            if m == max_average - 1 {
                sum += distance(&first[m], &first[0]);
            } else {
                sum += distance(&first[m], &first[m + 1]);
            }
        }
        let average = sum /((i as u64) * max_average as u64);

        key_length_distances.insert(i, average);
    }


    let mut ordered_key_lengths = Vec::from_iter(key_length_distances);
    ordered_key_lengths.sort_by(|a, b| a.1.cmp(&b.1));

    let mut potential_key_lengths = Vec::new();
    let mut min_length = ordered_key_lengths[0].0 as u64;

    for k in ordered_key_lengths {
        let key_length = k.0;
        let offset = k.1;

        if offset < min_length {
            min_length = offset;
            potential_key_lengths.clear();
            potential_key_lengths.push(key_length);
        } else if offset == min_length {
            potential_key_lengths.push(key_length);
        }

    }

    println!("{:?}", potential_key_lengths);
}

/// Use repeating-key XOR to encrypt the given string with the given key.
fn repeat_xor_encrypt(input: &str, key: &str) -> Vec<u8> {
    let length = input.as_bytes().len();
    let repeat = key.as_bytes().iter().cycle().take(length).collect::<Vec<&u8>>();

//    TODO figure out how to fix the reference type here.
    let mut deref_repeat = Vec::new();
    for entry in repeat {
        deref_repeat.push(*entry);
    }

    xor(input.as_bytes().to_vec(), deref_repeat)
}

/// Determine if the given byte vector has top characters corresponding to English.
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
