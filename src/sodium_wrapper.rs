use sodiumoxide::crypto::{secretbox, box_};
use sodiumoxide::init as sodium_init;

pub fn encrypt_sodium(plaintext: &str) -> Result<(Vec<u8>, secretbox::Nonce, secretbox::Key), String> {
    sodium_init().map_err(|_| "Failed to initialize sodiumoxide")?;

    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let ciphertext = secretbox::seal(plaintext.as_bytes(), &nonce, &key);

    Ok((ciphertext, nonce, key))
}

pub fn decrypt_sodium(ciphertext: &[u8], nonce: &secretbox::Nonce, key: &secretbox::Key) -> Result<String, String> {
    secretbox::open(ciphertext, nonce, key)
        .map(|decrypted| String::from_utf8(decrypted).unwrap_or_else(|_| "Failed to decode plaintext".to_owned()))
        .map_err(|_| "Decryption failed".to_owned())
}
