use hex::FromHex;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};

/// Compute the SHA256 hash of the data
/// simulating the hashing after Solidity's abi.encodePacked
pub(crate) fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute the KECCAK256 hash of the data
pub(crate) fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

pub(crate) fn encode(data: &[u8; 32]) -> String {
    hex::encode(data)
}

pub(crate) fn decode(data: &str) -> [u8; 32] {
    <[u8; 32]>::from_hex(data).expect("Invalid hex")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_str() {
        let input = "1234567890-32-byte-string-key";
        let secret = keccak256(input.as_bytes());
        assert_eq!(
            secret,
            [
                125, 149, 215, 216, 215, 68, 246, 157, 132, 148, 25, 101, 167, 114, 168, 136, 197,
                7, 93, 34, 129, 103, 21, 179, 167, 102, 180, 151, 13, 145, 182, 73
            ],
            "Invalid secret key"
        );
        assert_eq!(
            encode(&secret),
            "7d95d7d8d744f69d84941965a772a888c5075d22816715b3a766b4970d91b649",
            "Invalid secret key"
        );

        let hash = <[u8; 32]>::from(Sha256::digest(secret));
        assert_eq!(
            hash,
            [
                40, 103, 196, 181, 43, 147, 6, 208, 75, 94, 70, 165, 120, 50, 192, 67, 27, 192, 26,
                6, 51, 49, 0, 237, 26, 66, 172, 205, 85, 54, 64, 152
            ],
            "Invalid secret hash"
        );
        assert_eq!(
            encode(&hash),
            "2867c4b52b9306d04b5e46a57832c0431bc01a06333100ed1a42accd55364098",
            "Invalid secret hash"
        );
    }

    #[test]
    fn test_input_bytes() {
        let secret_key = [
            125, 149, 215, 216, 215, 68, 246, 157, 132, 148, 25, 101, 167, 114, 168, 136, 197, 7,
            93, 34, 129, 103, 21, 179, 167, 102, 180, 151, 13, 145, 182, 73,
        ];
        let hash = sha256(&secret_key);

        assert_eq!(
            hash,
            [
                40, 103, 196, 181, 43, 147, 6, 208, 75, 94, 70, 165, 120, 50, 192, 67, 27, 192, 26,
                6, 51, 49, 0, 237, 26, 66, 172, 205, 85, 54, 64, 152
            ],
            "Invalid secret hash"
        );

        let expected_hash_hex = "2867c4b52b9306d04b5e46a57832c0431bc01a06333100ed1a42accd55364098";
        let expected_hash = decode(expected_hash_hex);
        assert_eq!(encode(&hash), expected_hash_hex, "Invalid secret hash");
        assert_eq!(hash, expected_hash, "Invalid secret hash");
    }
}
