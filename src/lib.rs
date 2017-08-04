use std::cmp::Eq;
use std::ops::Add;
use std::ops::Mul;

pub struct Mod {
    value: i32,
    modulus: i32,
}

impl Mod {
    pub fn new(value: i32, modulus: i32) -> Mod {
        Mod {
            value: value % modulus,
            modulus: modulus,
        }
    }
}

impl PartialEq for Mod {
    fn eq(&self, other: &Mod) -> bool {
        self.value == other.value && self.modulus == other.modulus
    }
}

impl Eq for Mod {}

impl Add for Mod {
    type Output = Mod;

    fn add(self, other: Self) -> Self::Output {
        Mod::new(self.value + other.value, self.modulus)
    }
}

impl Mul for Mod {
    type Output = Mod;

    fn mul(self, other: Self) -> Self::Output {
        Mod::new(self.value * other.value, self.modulus)
    }
}

pub fn gcd(a: i32, b: i32) -> i32 {
    match b == 0 {
        true => a,
        false => gcd(b, a % b)
    }
}
