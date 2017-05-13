use std::cmp::Eq;

pub struct Mod {
    pub value: i32,
    pub modulus: i32,
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
