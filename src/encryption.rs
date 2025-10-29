use crate::error::{Error, Result};
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use sha1::{Digest, Sha1};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

/// Custom Base64 encoding matching PHP library
///
/// Converts standard Base64 characters to custom encoding:
/// - '+' → '-'
/// - '/' → '.'
/// - '=' → '_'
fn custom_base64_encode(data: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let standard = STANDARD.encode(data);
    standard
        .replace('+', "-")
        .replace('/', ".")
        .replace('=', "_")
}

/// Derives AES encryption key from password using SHA1
///
/// Key derivation process (matches PHP library):
/// 1. SHA1 hash of password
/// 2. Lowercase hexadecimal string
/// 3. Truncate to 32 characters
/// 4. Convert hex string to bytes (16-byte key)
fn derive_encryption_key(password: &str) -> [u8; 16] {
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();

    // Convert to lowercase hex string
    let hex_string = format!("{:x}", hash);

    // Take first 32 hex chars (16 bytes when converted)
    let key_hex = &hex_string[..32];

    // Convert hex string to bytes
    let mut key = [0u8; 16];
    for (i, chunk) in key_hex.as_bytes().chunks(2).enumerate() {
        let hex_byte = std::str::from_utf8(chunk).unwrap();
        key[i] = u8::from_str_radix(hex_byte, 16).unwrap();
    }

    key
}

/// Encrypts text using AES-128-CBC with custom Base64 encoding
///
/// Encryption process matching PHP library:
/// 1. Derive key from password using SHA1
/// 2. Apply PKCS7 padding to plaintext
/// 3. Encrypt using AES-128-CBC with provided IV
/// 4. Encode with custom Base64
///
/// # Arguments
///
/// * `plaintext` - Text to encrypt
/// * `password` - Encryption password
/// * `iv` - 16-byte initialization vector
///
/// # Returns
///
/// Encrypted and custom Base64 encoded string
///
/// # Errors
///
/// Returns an error if encryption fails
pub fn encrypt_text(plaintext: &str, password: &str, iv: &[u8; 16]) -> Result<String> {
    // Derive encryption key
    let key = derive_encryption_key(password);

    // Convert plaintext to bytes
    let plaintext_bytes = plaintext.as_bytes();

    // Create cipher
    let cipher = Aes128CbcEnc::new(&key.into(), iv.into());

    // Encrypt with PKCS7 padding
    // We need a buffer large enough for the plaintext + padding
    let mut buffer = vec![0u8; plaintext_bytes.len() + 16];
    buffer[..plaintext_bytes.len()].copy_from_slice(plaintext_bytes);

    let ciphertext = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, plaintext_bytes.len())
        .map_err(|_| Error::InvalidConfig("Encryption failed".to_string()))?;

    // Encode with custom Base64
    Ok(custom_base64_encode(ciphertext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_base64_encode() {
        // Test standard Base64 conversions
        let input = b"Hello, World!";
        let encoded = custom_base64_encode(input);

        // Should not contain +, /, or =
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
        assert!(!encoded.contains('='));

        // Should contain custom characters
        // (May or may not contain -, ., _ depending on input)
    }

    #[test]
    fn test_derive_encryption_key() {
        let password = "test_password";
        let key = derive_encryption_key(password);

        // Should be 16 bytes
        assert_eq!(key.len(), 16);

        // Same password should produce same key
        let key2 = derive_encryption_key(password);
        assert_eq!(key, key2);

        // Different password should produce different key
        let key3 = derive_encryption_key("different_password");
        assert_ne!(key, key3);
    }

    #[test]
    fn test_encrypt_text() {
        let plaintext = "Hello, World!";
        let password = "test_password";
        let iv = [0u8; 16]; // Use zero IV for testing

        let result = encrypt_text(plaintext, password, &iv);
        assert!(result.is_ok());

        let encrypted = result.unwrap();
        assert!(!encrypted.is_empty());
        assert_ne!(encrypted, plaintext);

        // Should be custom Base64 encoded
        assert!(!encrypted.contains('+'));
        assert!(!encrypted.contains('/'));
        assert!(!encrypted.contains('='));
    }

    #[test]
    fn test_encrypt_with_random_iv() {
        use rand::Rng;

        let plaintext = "Secret message";
        let password = "secure_password";
        let mut iv = [0u8; 16];
        rand::thread_rng().fill(&mut iv);

        let result = encrypt_text(plaintext, password, &iv);
        assert!(result.is_ok());

        // Different IV should produce different ciphertext
        let mut iv2 = [0u8; 16];
        rand::thread_rng().fill(&mut iv2);
        let result2 = encrypt_text(plaintext, password, &iv2);
        assert!(result2.is_ok());

        assert_ne!(result.unwrap(), result2.unwrap());
    }

    #[test]
    fn test_encrypt_empty_string() {
        let plaintext = "";
        let password = "test_password";
        let iv = [0u8; 16];

        let result = encrypt_text(plaintext, password, &iv);
        assert!(result.is_ok());

        // Even empty string should produce ciphertext due to padding
        let encrypted = result.unwrap();
        assert!(!encrypted.is_empty());
    }

    #[test]
    fn test_encrypt_long_message() {
        let plaintext = "This is a much longer message that spans multiple AES blocks and should be properly encrypted with PKCS7 padding.";
        let password = "test_password";
        let iv = [0u8; 16];

        let result = encrypt_text(plaintext, password, &iv);
        assert!(result.is_ok());

        let encrypted = result.unwrap();
        assert!(!encrypted.is_empty());
    }
}
