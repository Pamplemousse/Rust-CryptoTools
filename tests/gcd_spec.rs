extern crate crypto_tools;

use crypto_tools::gcd;

#[test]
fn returns_the_gcd_of_two_numbers() {
    assert!(gcd(45, 20) == 5);
}

#[test]
fn with_coprime_numbers_returns_1() {
    assert!(gcd(13, 8) == 1);
}

#[test]
fn with_a_number_dividing_another_returns_the_first() {
    assert!(gcd(12, 4) == 4);
    assert!(gcd(4, 12) == 4);
}
