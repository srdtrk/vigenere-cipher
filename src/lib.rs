//! Minimal pure rust implementation of the vigenere cipher.
//!
//! Allows users to encrypt and decrypt english words using the vigenere cipher.
//!
//! This crate was written for the Session 1 of Open Rust Cryptography Engineering Study Group
//!
//! https://hackmd.io/@thor314/ryEWRY6Qs

const ASCII_A: i8 = 'A' as i8;
const ASCII_Z: i8 = 'Z' as i8;
const ABC_SIZE: i8 = ASCII_Z - ASCII_A + 1;

/// this function returns an encrypted ciphertext using vigenere cipher
pub fn encrypt(plaintext: &str, key: &str) -> String {
    if !plaintext.chars().all(|c| c.is_ascii_alphabetic())
        | !key.chars().all(|c| c.is_ascii_alphabetic())
    {
        panic!("the plaintext and/or key includes a character outside of the 26 letter English alphabet")
    }

    let mut ciphertext = String::from("");

    for (step, c) in plaintext.chars().enumerate() {
        let key_index = step % key.len();
        let new_char = char_shift(c, (key.chars().nth(key_index).unwrap() as i8) - ASCII_A);
        ciphertext.push(new_char);
    }

    ciphertext
}

/// this function returns the decrypted plaintext of an encrypted message using vigenere cipher
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    if !ciphertext.chars().all(|c| c.is_ascii_alphabetic())
        | !key.chars().all(|c| c.is_ascii_alphabetic())
    {
        panic!("the ciphertext and/or key includes a character outside of the 26 letter English alphabet")
    }

    let mut plaintext = String::from("");

    for (step, c) in ciphertext.chars().enumerate() {
        let key_index = step % key.len();
        let plain_char = char_shift(c, -((key.chars().nth(key_index).unwrap() as i8) - ASCII_A));
        plaintext.push(plain_char);
    }

    plaintext
}

fn char_shift(c: char, shift: i8) -> char {
    // we only work with uppercase letters
    let input = c.to_ascii_uppercase();

    if shift >= ABC_SIZE || shift <= -ABC_SIZE {
        panic!("invalid shift");
    }

    let input_index = input as i8 - ASCII_A;
    let output_index = (input_index + shift).rem_euclid(ABC_SIZE);
    ((output_index + ASCII_A) as u8) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_encrypt_decrypt() {
        let plaintext = String::from("attackatdawn");
        let key = String::from("LEMONLEMONLE");

        let ciphertext = encrypt(&plaintext, &key);
        let expected_ciphertext = String::from("LXFOPVEFRNHR");
        assert_eq!(ciphertext, expected_ciphertext);

        let decrypted_plaintext = decrypt(&ciphertext, &key);
        let expected_plaintext = String::from("ATTACKATDAWN");
        assert_eq!(expected_plaintext, decrypted_plaintext);
    }
}
