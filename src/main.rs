#![allow(dead_code)]

extern crate base64;
extern crate clap;

use std::{fmt::Write};

use base64::{encode};
use clap::{App, Arg};
use time;

mod helpers;
use helpers::{decode_hex, Uint8Vector};

fn one() -> bool {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let decoded = decode_hex(hex_str).unwrap();
    expected_str == encode(&decoded)
}

fn two() -> bool {
    let expected_str = "746865206b696420646f6e277420706c6179";
    let initial = Uint8Vector::from_hex_str("1c0111001f010100061a024b53535009181c").unwrap();
    let for_xor = Uint8Vector::from_hex_str("686974207468652062756c6c277320657965").unwrap();
    let result = initial ^ for_xor;
    expected_str == result.to_hex_str()
}

#[derive(Debug)]
struct ThreeResult {
    score: i32,
    bytes: Uint8Vector
}

fn three() -> String {
    let crypted = Uint8Vector::from_hex_str("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let t = crypted.len();
    let mut results: Vec<ThreeResult> = Vec::new();
    for n in 0..u8::max_value() {
        let attempt_vec = vec![n; t];
        let attempt = Uint8Vector(attempt_vec);
        let result = crypted.to_xor_with(&attempt);
        results.push(ThreeResult{ score: result.valid_ascii_score(), bytes: result })
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));

    let mut output = String::from("\n");

    for res in results[..5].iter() {
        writeln!(&mut output, "Score: {}\t value: {}", res.score, res.bytes.to_ascii_str()).unwrap();
    }

    output
}

fn main() {
    let matches = App::new("cryptopals")
        .version("0.1.0")
        .author("Colin McCulloch <colin@zyzle.dev>")
        .about("My solutions to the cryptopals exercises")
        .arg(Arg::with_name("SETNUM")
            .required(true)
            .takes_value(true)
            .short("s")
            .long("set")
            .help("The set of exercises to pick")
        )
        .arg(Arg::with_name("CHALLNUM")
            .required(true)
            .takes_value(true)
            .short("c")
            .long("challenge")
            .help("The propblem solution to run from then chosen set")
        )
        .get_matches();

    let _set = matches.value_of("SETNUM").unwrap();
    let _prob = matches.value_of("CHALLNUM").unwrap();

    let start = time::precise_time_ns();
    let result = three();

    println!("Result: {}", result);
    println!("Took {} seconds",
        (time::precise_time_ns() - start) as f64 / 10_f64.powf(9.0));
}
