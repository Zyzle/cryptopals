use std::{fmt::Write, num::ParseIntError, ops::BitXor, ops::Deref};

use base64;

#[derive(Debug, PartialEq)]
pub struct CryptoVec(pub Vec<u8>);

impl CryptoVec {
    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        CryptoVec(Vec::from(bytes))
    }

    pub fn from_hex_str(s: &str) -> Result<CryptoVec, ParseIntError> {
        let arr = decode_hex(s)?;
        Ok(CryptoVec(arr))
    }

    pub fn from_base64(s: &str) -> Result<CryptoVec, base64::DecodeError> {
        let fixed: String = s
            .chars()
            .filter(|c| ![' ', '\n', '\t', '\r', '\x0b', '\x0c'].contains(c))
            .collect();
        let raw = base64::decode(fixed)?;
        Ok(CryptoVec(raw))
    }

    pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| *x ^ *y)
            .map(|z| z.count_ones())
            .sum()
    }

    pub fn to_hex_str(&self) -> String {
        encode_hex(self.as_slice())
    }

    pub fn valid_ascii_score(&self) -> i32 {
        let mut score = 0;
        for &b in self.iter() {
            if b.is_ascii_alphanumeric() || b.is_ascii_whitespace() {
                score += 1;
            } else {
                score -= 1;
            }
        }
        score
    }

    pub fn to_ascii_str(&self) -> String {
        let mut res = String::with_capacity(self.len());

        for &b in self.iter() {
            write!(&mut res, "{}", b as char).unwrap()
        }

        res
    }

    pub fn to_rolling_xor_with(&self, o: &[u8]) -> Self {
        let mut new_vec = Vec::new();
        for (n, byte) in self.iter().enumerate() {
            new_vec.push(byte ^ o[n % o.len()]);
        }

        CryptoVec(new_vec)
    }
}

impl BitXor for CryptoVec {
    type Output = Self;

    fn bitxor(self, CryptoVec(rhs): Self) -> Self::Output {
        let CryptoVec(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        CryptoVec(lhs.iter().zip(rhs.iter()).map(|(x, y)| *x ^ *y).collect())
    }
}

impl<'a, 'b> BitXor<&'b CryptoVec> for &'a CryptoVec {
    type Output = CryptoVec;

    fn bitxor(self, CryptoVec(rhs): &'b CryptoVec) -> Self::Output {
        let CryptoVec(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        CryptoVec(lhs.iter().zip(rhs.iter()).map(|(x, y)| *x ^ *y).collect())
    }
}

impl Deref for CryptoVec {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
