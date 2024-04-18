#[cfg(test)]
mod base64_tests {
    use VibeProtocol::base64_wrapper::{encode_base64, decode_base64};

    #[test]
    fn test_encode_and_decode_base64() {
        let data = "vibe protocol";
        let encoded = encode_base64(data.as_bytes());

        // Base64 strings are always a multiple of 4 characters long
        assert!(encoded.len() % 4 == 0);

        let decoded = decode_base64(&encoded).expect("Decoding failed");

        // Make sure the decoded data matches the original data
        assert_eq!(decoded, data.as_bytes());
    }
}
