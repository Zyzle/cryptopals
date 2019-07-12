use std::{fmt::Write, num::ParseIntError, ops::BitXor, ops::Deref};

#[derive(Debug, PartialEq)]
pub struct Uint8Vector(pub Vec<u8>);

impl Uint8Vector {
    pub fn from_hex_str(s: &str) -> Result<Uint8Vector, ParseIntError> {
        let arr = match decode_hex(s) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        Ok(Uint8Vector(arr))
    }

    pub fn to_hex_str(&self) -> String {
        encode_hex(self.as_slice())
    }

    pub fn to_xor_with(&self, o: &Uint8Vector) -> Uint8Vector {
        assert_eq!(self.len(), o.len());
        Uint8Vector(self.iter()
            .zip(o.iter())
            .map(|(x, y)| *x ^ *y)
            .collect()
        )
    }

    pub fn valid_ascii_score(&self) -> i32 {
        let mut score = 0;
        for &b in self.iter() {
            if b.is_ascii_alphanumeric() || b.is_ascii_whitespace() {
                score += 1;
            }
            else {
                score -= 1;
            }
        }
        score
    }

    pub fn to_ascii_str(&self) -> String {
        let mut res = String::with_capacity(self.len());

        for &b in self.iter() {
            write!(&mut res, "{}", b as char).unwrap();
        }
        res
    }
}

impl BitXor for Uint8Vector {
    type Output = Self;
    
    fn bitxor(self, Uint8Vector(rhs): Self) -> Self::Output {
        let Uint8Vector(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        Uint8Vector(lhs.iter()
            .zip(rhs.iter())
            .map(|(x, y)| *x ^ *y)
            .collect()
        )
    }
}

impl Deref for Uint8Vector {
    type Target = Vec<u8>;
    
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

#[derive(Debug)]
pub struct CryptTestResult {
    score: i32,
    bytes: Uint8Vector
}

impl CryptTestResult {

    pub fn new(score: i32, bytes: Uint8Vector) -> CryptTestResult {
        CryptTestResult {
            score: score,
            bytes: bytes
        }
    }

    pub fn score(&self) -> &i32 {
        &self.score
    }

    pub fn bytes(&self) -> &Uint8Vector {
        &self.bytes
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}