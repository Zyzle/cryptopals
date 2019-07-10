use std::{fmt::Write, num::ParseIntError, ops::BitXor, ops::Deref};

#[derive(Debug, PartialEq)]
pub struct Uint8Vector(pub Vec<u8>);

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