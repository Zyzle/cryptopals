#![allow(dead_code)]

extern crate base64;
extern crate clap;

use base64::{encode};
use clap::{App, Arg};
use time;

mod helpers;
use helpers::{decode_hex, encode_hex, Uint8Vector};

fn one() -> bool {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let decoded = decode_hex(hex_str).unwrap();
    expected_str == encode(&decoded)
}

fn two() -> bool {
    let expected_str = "746865206b696420646f6e277420706c6179";
    let is = decode_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let initial = Uint8Vector(is);
    let ifx = decode_hex("686974207468652062756c6c277320657965").unwrap();
    let for_xor = Uint8Vector(ifx);
    let result = initial ^ for_xor;
    expected_str == encode_hex(result.as_slice())
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
    let result = two();

    println!("Result: {:?}", result);
    println!("Took {} seconds",
        (time::precise_time_ns() - start) as f64 / 10_f64.powf(9.0));
}
