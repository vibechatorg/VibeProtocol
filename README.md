<p align="center"><a href="https://vibechat.nl" target="_blank"><img src="https://cdn.discordapp.com/attachments/1230609534297571598/1230619097562353714/vibebanner.png?ex=6633fa58&is=66218558&hm=4347ed1662fb5fde16b3e85dec9b327616cd8275ba6975bfcf2bd88c8e1c7d67&" width="400" alt="Laravel Logo"></a></p>

<p align="center">
<img alt="Crates.io Version" src="https://img.shields.io/crates/v/vibeprotocol">
<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/vibechatorg/VibeProtocol/.github%2Fworkflows%2Frust.yml">
</p>


# VibeProtocol

VibeProtocol is a fast and secure encryption library for Rust, providing end-to-end encryption with a focus on performance and security. It combines the simplicity of the Caesar cipher with the robustness of libsodium's encryption mechanisms and base64 encoding.

## Features

- End-to-end encryption using a blend of classic and modern techniques.
- Caesar cipher for initial transformation.
- Sodium library for strong encryption.
- Base64 encoding for output formatting.
- Designed for high performance with security in mind.

## Quick Start

Run the following command to add VibeProtocol


```bash
cargo add VibeProtocol
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

## Documentation
Visit the [VibeProtocol documentation](https://docs.rs/VibeProtocol/latest/VibeProtocol/) for more information.
## Contributing
We welcome contributions! Please feel free to submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
