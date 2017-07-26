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
fn mod_numbers_with_different_values_differs() {
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

#[test]
fn add_numbers_with_same_modulus_succeeds() {
    let mod_1: Mod = Mod::new(3, 5);
    let mod_2: Mod = Mod::new(1, 5);
    let result: Mod = Mod::new(4, 5);

    assert!(mod_1 + mod_2 == result);
}

#[test]
fn add_mod_numbers_results_in_canonical_form() {
    let mod_1: Mod = Mod::new(1, 5);
    let mod_2: Mod = Mod::new(7, 5);
    let result: Mod = Mod::new(3, 5);

    assert!(mod_1 + mod_2 == result);
}

#[test]
fn substract_numbers_with_same_modulus_succeeds() {
    let mod_1: Mod = Mod::new(3, 5);
    let mod_2: Mod = Mod::new(1, 5);
    let result: Mod = Mod::new(2, 5);

    assert!(mod_1 - mod_2 == result);
}

#[test]
fn substract_mod_numbers_results_in_canonical_form() {
    let mod_1: Mod = Mod::new(1, 5);
    let mod_2: Mod = Mod::new(3, 5);
    let result: Mod = Mod::new(3, 5);

    assert!(mod_1 - mod_2 == result);
}

#[test]
fn multiply_numbers_with_same_modulus_succeeds() {
    let mod_1: Mod = Mod::new(2, 7);
    let mod_2: Mod = Mod::new(3, 7);
    let result: Mod = Mod::new(6, 7);

    assert!(mod_1 * mod_2 == result);
}

#[test]
fn multiply_mod_numbers_results_in_canonical_form() {
    let mod_1: Mod = Mod::new(3, 4);
    let mod_2: Mod = Mod::new(2, 4);
    let result: Mod = Mod::new(2, 4);

    assert!(mod_1 * mod_2 == result);
}
