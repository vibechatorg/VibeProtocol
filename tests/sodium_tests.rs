#[cfg(test)]
mod sodium_tests {
    use VibeProtocol::sodium_wrapper::{encrypt_sodium, decrypt_sodium};

    #[test]
    fn test_encrypt_and_decrypt_sodium() {
        let plaintext = "secure message";
        let (ciphertext, nonce, key) = encrypt_sodium(plaintext).expect("Encryption failed");

        // Make sure the ciphertext is not the same as the plaintext
        assert_ne!(ciphertext, plaintext.as_bytes());

        let decrypted_plaintext = decrypt_sodium(&ciphertext, &nonce, &key).expect("Decryption failed");

        // Make sure the decrypted text matches the original plaintext
        assert_eq!(decrypted_plaintext, plaintext);
    }
}
