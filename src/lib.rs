use std::cmp::Eq;
use std::ops::Add;

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

    fn add(self, other: Mod) -> Mod {
        Mod::new(self.value + other.value, self.modulus)
    }
}

pub fn gcd(a: i32, b: i32) -> i32 {
    let mut r1 = b;
    let mut r2 = a;
    let mut r = r2 % r1;

    while r != 0 {
        r2 = r1;
        r1 = r;
        r = r2 % r1;
    }

    r1
}
