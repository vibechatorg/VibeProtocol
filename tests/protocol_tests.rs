#[cfg(test)]
mod protocol_tests {
    use VibeProtocol::{encrypt, decrypt};

    #[test]
    fn test_encryption_and_decryption_flow() {
        let plaintext = "Hello, VibeProtocol!";
        let caesar_shift = 3; // Use a shift value for the Caesar cipher

        // Encrypt the message
        let (ciphertext_b64, nonce_b64, key_b64) = encrypt(plaintext, caesar_shift).unwrap();

        // Decrypt the message
        let decrypted_text = decrypt(&ciphertext_b64, &nonce_b64, &key_b64, caesar_shift).unwrap();

        // The decrypted text should match the original plaintext
        assert_eq!(decrypted_text, plaintext);
    }
}
