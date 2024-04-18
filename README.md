# VibeProtocol

VibeProtocol is a fast and secure encryption library for Rust, providing end-to-end encryption with a focus on performance and security. It combines the simplicity of the Caesar cipher with the robustness of libsodium's encryption mechanisms and base64 encoding.

## Features

- End-to-end encryption using a blend of classic and modern techniques.
- Caesar cipher for initial transformation.
- Sodium library for strong encryption.
- Base64 encoding for output formatting.
- Designed for high performance with security in mind.

## Quick Start

Add VibeProtocol to your `Cargo.toml`:

```toml
[dependencies]
VibeProtocol = "0.1.0"
```

## Use VibeProtocol in your project:

```rust
use VibeProtocol::{encrypt, decrypt};

fn main() {
    let message = "Secret Message";
    let caesar_shift = 13;

    let (encrypted_message, nonce, key) = encrypt(message, caesar_shift).unwrap();
    let decrypted_message = decrypt(&encrypted_message, &nonce, &key, caesar_shift).unwrap();

    assert_eq!(message, decrypted_message);
}
```
## Contributing
We welcome contributions! Please feel free to submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
