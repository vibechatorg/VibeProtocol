#[cfg(test)]
mod caesar_tests {
    use VibeProtocol::caesar::{encrypt_caesar, decrypt_caesar};

    #[test]
    fn test_encrypt_caesar() {
        let plaintext = "hello";
        let shift = 3; // A common shift for Caesar's cipher
        let ciphertext = encrypt_caesar(plaintext, shift);
        assert_eq!(ciphertext, "khoor");
    }

    #[test]
    fn test_decrypt_caesar() {
        let ciphertext = "khoor";
        let shift = 3; // Must be the same shift used for encryption
        let plaintext = decrypt_caesar(ciphertext, shift);
        assert_eq!(plaintext, "hello");
    }
}
