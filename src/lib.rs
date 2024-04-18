pub mod caesar;
pub mod base64_wrapper;
pub mod sodium_wrapper;

use caesar::{encrypt_caesar, decrypt_caesar};
use sodium_wrapper::{decrypt_sodium, encrypt_sodium};
use base64_wrapper::{encode_base64, decode_base64};

/// Encrypts a plaintext message using the VibeProtocol encryption sequence:
/// Caesar cipher -> Sodium encryption -> Base64 encoding.
pub fn encrypt(plaintext: &str, caesar_shift: u8) -> Result<(String, String, String), String> {
    // Step 1: Caesar cipher
    let caesar_cipher_text = encrypt_caesar(plaintext, caesar_shift);

    // Step 2: Sodium encryption
    let (ciphertext, nonce, key) = encrypt_sodium(&caesar_cipher_text)?;

    // Step 3: Base64 encoding
    let ciphertext_b64 = encode_base64(&ciphertext);
    let nonce_b64 = encode_base64(nonce.as_ref());
    let key_b64 = encode_base64(key.as_ref());

    Ok((ciphertext_b64, nonce_b64, key_b64))
}

/// Decrypts a ciphertext message using the VibeProtocol decryption sequence:
/// Base64 decoding -> Sodium decryption -> Caesar cipher.
pub fn decrypt(ciphertext_b64: &str, nonce_b64: &str, key_b64: &str, caesar_shift: u8) -> Result<String, String> {
    // Step 1: Base64 decoding with error mapping
    let ciphertext = decode_base64(ciphertext_b64)
        .map_err(|e| e.to_string())?;
    let nonce = decode_base64(nonce_b64)
        .map_err(|e| e.to_string())?;
    let key = decode_base64(key_b64)
        .map_err(|e| e.to_string())?;

    // Ensure nonce and key are the correct length
    let nonce = sodiumoxide::crypto::secretbox::Nonce::from_slice(&nonce)
        .ok_or("Nonce is not the correct length")?;
    let key = sodiumoxide::crypto::secretbox::Key::from_slice(&key)
        .ok_or("Key is not the correct length")?;

    // Step 2: Sodium decryption
    let decrypted_text = decrypt_sodium(&ciphertext, &nonce, &key)?;

    // Step 3: Caesar cipher decryption
    let plaintext = decrypt_caesar(&decrypted_text, caesar_shift);

    Ok(plaintext)
}
