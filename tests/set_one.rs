use std::io::prelude::*;
use std::{
    fs::File,
    io,
    io::{BufReader, Read},
};

use cryptopals::{decode_hex, CryptoVec};

use base64::encode;

pub struct CryptTestResult {
    score: i32,
    bytes: CryptoVec,
}

impl CryptTestResult {
    pub fn new(score: i32, bytes: CryptoVec) -> CryptTestResult {
        CryptTestResult { score, bytes }
    }

    pub fn score(&self) -> &i32 {
        &self.score
    }

    pub fn bytes(&self) -> &CryptoVec {
        &self.bytes
    }
}

fn crypt_test(crypted: &CryptoVec) -> Vec<CryptTestResult> {
    let t = crypted.len();
    let mut results: Vec<CryptTestResult> = Vec::new();
    for n in 0..u8::max_value() {
        let attempt_vec = vec![n; t];
        let attempt = CryptoVec(attempt_vec);
        let result = crypted ^ &attempt;
        results.push(CryptTestResult::new(result.valid_ascii_score(), result));
    }
    results.sort_by(|a, b| b.score.cmp(&a.score()));
    results
}

fn file_read(filename: &str) -> Result<io::BufReader<File>, io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(f);
    Ok(file)
}

#[test]
fn one() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let decoded = decode_hex(hex_str).unwrap();

    assert_eq!(expected_str, encode(&decoded));
}

#[test]
fn two() {
    let expected_str = "746865206b696420646f6e277420706c6179";
    let initial = CryptoVec::from_hex_str("1c0111001f010100061a024b53535009181c").unwrap();
    let for_xor = CryptoVec::from_hex_str("686974207468652062756c6c277320657965").unwrap();
    let result = initial ^ for_xor;

    assert_eq!(expected_str, result.to_hex_str());
}

#[test]
fn three() {
    let crypted = CryptoVec::from_hex_str(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    )
    .unwrap();
    let results = crypt_test(&crypted);
    assert_eq!(
        "Cooking MC\'s like a pound of bacon",
        results[0].bytes().to_ascii_str()
    );
}

#[test]
fn four() {
    let x = file_read("assets/4.txt").unwrap();
    let mut file_vecs: Vec<CryptTestResult> = Vec::new();

    for line in x.lines() {
        let l = line.unwrap();
        let crypt_vec = CryptoVec::from_hex_str(&l).unwrap();
        file_vecs.extend(crypt_test(&crypt_vec));
    }

    file_vecs.sort_by(|a, b| b.score().cmp(&a.score()));

    assert_eq!(
        "Now that the party is jumping\n",
        file_vecs[0].bytes().to_ascii_str()
    );
}

#[test]
fn five() {
    let expected_hex = String::from(
        "0b3637272a2b2e63622c2e69692a2369\
                                     3a2a3c6324202d623d63343c2a262263\
                                     24272765272a282b2f20430a652e2c65\
                                     2a3124333a653e2b2027630c692b2028\
                                     3165286326302e27282f",
    );
    let plain_str = String::from(
        "Burning 'em, if you ain't quick and nimble\n\
                                  I go crazy when I hear a cymbal",
    );

    let plain_vec = CryptoVec::from_str(&plain_str);
    let key = String::from("ICE");
    let key_vec = CryptoVec::from_str(&key);

    let crypted_vec = plain_vec.to_rolling_xor_with(key_vec.as_slice());

    assert_eq!(expected_hex, crypted_vec.to_hex_str());
}

#[test]
fn six_a() {
    let one = CryptoVec::from_str("this is a test");
    let two = CryptoVec::from_str("wokka wokka!!!");
    let hd = CryptoVec::hamming_distance(one.as_slice(), two.as_slice());
    assert_eq!(37, hd);
}

#[test]
fn six_b() {
    let mut x = file_read("assets/6.txt").unwrap();
    let mut file_string = String::new();
    x.read_to_string(&mut file_string).unwrap();

    let file_vec = CryptoVec::from_base64(&file_string).unwrap();
    let mut distance_list: Vec<(u32, usize)> = Vec::new();

    for keysize in 2..=40 {
        let first_slice = &file_vec[..keysize];
        let second_slice = &file_vec[keysize..keysize * 2];
        let norm_distance = CryptoVec::hamming_distance(first_slice, second_slice) / keysize as u32;
        distance_list.push((norm_distance, keysize));
    }

    distance_list.sort_by(|a, b| a.0.cmp(&b.0));
    let found_keysize = distance_list[0].1;
    println!("{:?}", distance_list);
}
