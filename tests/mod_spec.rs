extern crate crypto_tools;

use crypto_tools::Mod;

#[test]
fn same_mod_numbers_are_equals() {
    let mod_1: Mod = Mod::new(1, 5);
    let mod_2: Mod = Mod::new(1, 5);

    assert!(mod_1 == mod_2);
}

#[test]
fn mod_numbers_with_different_modulus_fails() {
    let mod_1: Mod = Mod::new(1, 5);
    let mod_2: Mod = Mod::new(1, 4);

    assert!(mod_1 != mod_2);
}

#[test]
fn mod_numbers_with_different_values_fails() {
    let mod_1: Mod = Mod::new(1, 5);
    let mod_2: Mod = Mod::new(3, 5);

    assert!(mod_1 != mod_2);
}

#[test]
fn similar_mod_numbers_are_equals() {
    let mod_1: Mod = Mod::new(1, 5);
    let mod_2: Mod = Mod::new(6, 5);

    assert!(mod_1 == mod_2);
}
