pub fn encrypt_caesar(plaintext: &str, shift: u8) -> String {
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let c = c as u8;
                let shifted = (c - base + shift) % 26 + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decrypt_caesar(ciphertext: &str, shift: u8) -> String {
    encrypt_caesar(ciphertext, 26 - (shift % 26))
}
