//! # Cryptographic Utilities
//!
//! Core cryptographic operations for Augmented-ID including:
//! - Key generation and management
//! - Digital signatures (Ed25519)
//! - Hash functions (SHA-256)
//! - Encryption (AES-256-GCM)
//!
//! All cryptographic operations are designed for offline-first use
//! with secure enclave integration.

use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Verifier, Signature};
use sha2::{Sha256, Digest};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, Key};
use rand::rngs::OsRng;
use hex::{encode, decode};
use crate::error::{AugIdError, AugIdResult};

/// ============================================================================
/// KEY MANAGEMENT
/// ============================================================================

/// Key pair for DID operations
pub struct KeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl KeyPair {
    /// Generate a new key pair using OS RNG
    pub fn generate() -> AugIdResult<Self> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }
    
    /// Export verifying key as hex string (for DID document)
    pub fn verifying_key_hex(&self) -> String {
        encode(self.verifying_key.as_bytes())
    }
    
    /// Import verifying key from hex string
    pub fn verifying_key_from_hex(hex_str: &str) -> AugIdResult<VerifyingKey> {
        let bytes = decode(hex_str).map_err(|_| AugIdError::KeyGenerationFailed)?;
        if bytes.len() != 32 {
            return Err(AugIdError::KeyGenerationFailed);
        }
        
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        
        VerifyingKey::from_bytes(&key_bytes)
            .map_err(|_| AugIdError::KeyGenerationFailed)
    }
}

/// ============================================================================
/// DIGITAL SIGNATURES
/// ============================================================================

/// Sign data with Ed25519
pub fn sign_data(signing_key: &SigningKey, data: &[u8]) -> AugIdResult<String> {
    let signature = signing_key.sign(data);
    Ok(encode(signature.to_bytes()))
}

/// Verify Ed25519 signature
pub fn verify_signature(
    verifying_key: &VerifyingKey,
    data: &[u8],
    signature_hex: &str,
) -> AugIdResult<()> {
    let signature_bytes = decode(signature_hex)
        .map_err(|_| AugIdError::SignatureVerificationFailed)?;
    
    if signature_bytes.len() != 64 {
        return Err(AugIdError::SignatureVerificationFailed);
    }
    
    let mut signature_arr = [0u8; 64];
    signature_arr.copy_from_slice(&signature_bytes);
    let signature = Signature::from_bytes(&signature_arr);
    
    verifying_key
        .verify(data, &signature)
        .map_err(|_| AugIdError::SignatureVerificationFailed)
}

/// ============================================================================
/// HASH FUNCTIONS
/// ============================================================================

/// Compute SHA-256 hash of data
pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("sha256:{}", encode(result))
}

/// Verify data against expected hash
pub fn verify_hash(data: &[u8], expected_hash: &str) -> AugIdResult<()> {
    let computed_hash = sha256_hash(data);
    
    if computed_hash != expected_hash {
        return Err(AugIdError::SnapshotHashMismatch {
            expected: expected_hash.to_string(),
            actual: computed_hash,
        });
    }
    
    Ok(())
}

/// ============================================================================
/// ENCRYPTION (AES-256-GCM)
/// ============================================================================

/// Encrypt data with AES-256-GCM
pub fn encrypt_data(key: &[u8; 32], plaintext: &[u8]) -> AugIdResult<(String, String)> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    
    // Generate random nonce
    let nonce = aes_gcm::aead::OsRng.gen::<aes_gcm::Nonce>();
    
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| AugIdError::EncryptionFailed)?;
    
    Ok((
        encode(ciphertext),
        encode(nonce.as_slice()),
    ))
}

/// Decrypt data with AES-256-GCM
pub fn decrypt_data(key: &[u8; 32], ciphertext_hex: &str, nonce_hex: &str) -> AugIdResult<Vec<u8>> {
    let ciphertext = decode(ciphertext_hex)
        .map_err(|_| AugIdError::DecryptionFailed)?;
    let nonce_bytes = decode(nonce_hex)
        .map_err(|_| AugIdError::DecryptionFailed)?;
    
    if nonce_bytes.len() != 12 {
        return Err(AugIdError::DecryptionFailed);
    }
    
    let mut nonce_arr = [0u8; 12];
    nonce_arr.copy_from_slice(&nonce_bytes);
    let nonce = Nonce::from(nonce_arr);
    
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    
    cipher
        .decrypt(&nonce, ciphertext.as_slice())
        .map_err(|_| AugIdError::DecryptionFailed)
}

/// ============================================================================
/// BIOMETRIC VAULT KEY DERIVATION
/// ============================================================================

/// Derive vault key from device secure enclave
/// This is a placeholder - actual implementation would use platform-specific
/// secure enclave APIs (iOS Secure Enclave, Android Keystore, etc.)
pub fn derive_vault_key(device_id: &str, user_pin: &[u8]) -> AugIdResult<[u8; 32]> {
    // Combine device ID and PIN for key derivation
    let mut hasher = Sha256::new();
    hasher.update(device_id.as_bytes());
    hasher.update(user_pin);
    let result = hasher.finalize();
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    
    Ok(key)
}

/// ============================================================================
/// SNAPSHOT SIGNING AND VERIFICATION
/// ============================================================================

/// Sign an identity snapshot
pub fn sign_snapshot(
    signing_key: &SigningKey,
    snapshot_data: &[u8],
) -> AugIdResult<(String, String)> {
    let snapshot_hash = sha256_hash(snapshot_data);
    let signature = sign_data(signing_key, snapshot_data.as_ref())?;
    
    Ok((snapshot_hash, signature))
}

/// Verify an identity snapshot
pub fn verify_snapshot(
    verifying_key: &VerifyingKey,
    snapshot_data: &[u8],
    expected_hash: &str,
    signature_hex: &str,
) -> AugIdResult<()> {
    // Verify hash
    verify_hash(snapshot_data, expected_hash)?;
    
    // Verify signature
    verify_signature(verifying_key, snapshot_data, signature_hex)?;
    
    Ok(())
}

/// ============================================================================
/// TOKEN SIGNING HELPERS
/// ============================================================================

/// Sign a BfcToken (serialize then sign)
pub fn sign_token<T: serde::Serialize>(
    signing_key: &SigningKey,
    token: &T,
) -> AugIdResult<String> {
    let serialized = serde_json::to_vec(token)
        .map_err(|_| AugIdError::InternalError {
            message: "token serialization failed".to_string(),
        })?;
    
    sign_data(signing_key, &serialized)
}

/// Verify a BfcToken signature
pub fn verify_token<T: serde::Serialize>(
    verifying_key: &VerifyingKey,
    token: &T,
    signature_hex: &str,
) -> AugIdResult<()> {
    let serialized = serde_json::to_vec(token)
        .map_err(|_| AugIdError::InternalError {
            message: "token serialization failed".to_string(),
        })?;
    
    verify_signature(verifying_key, &serialized, signature_hex)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_pair_generation() {
        let keypair = KeyPair::generate();
        assert!(keypair.is_ok());
        
        let keypair = keypair.unwrap();
        assert_eq!(keypair.verifying_key_hex().len(), 64);
    }
    
    #[test]
    fn test_sign_and_verify() {
        let keypair = KeyPair::generate().unwrap();
        let data = b"test data to sign";
        
        let signature = sign_data(&keypair.signing_key, data).unwrap();
        assert!(!signature.is_empty());
        
        let verify_result = verify_signature(&keypair.verifying_key, data, &signature);
        assert!(verify_result.is_ok());
    }
    
    #[test]
    fn test_sha256_hash() {
        let data = b"test data";
        let hash = sha256_hash(data);
        
        assert!(hash.starts_with("sha256:"));
        assert_eq!(hash.len(), 71);  // "sha256:" + 64 hex chars
        
        // Verify hash is deterministic
        let hash2 = sha256_hash(data);
        assert_eq!(hash, hash2);
    }
    
    #[test]
    fn test_encrypt_decrypt() {
        let key = [1u8; 32];
        let plaintext = b"secret message";
        
        let (ciphertext, nonce) = encrypt_data(&key, plaintext).unwrap();
        assert!(!ciphertext.is_empty());
        assert!(!nonce.is_empty());
        
        let decrypted = decrypt_data(&key, &ciphertext, &nonce).unwrap();
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_snapshot_sign_verify() {
        let keypair = KeyPair::generate().unwrap();
        let snapshot_data = b"identity snapshot data";
        
        let (hash, signature) = sign_snapshot(&keypair.signing_key, snapshot_data).unwrap();
        
        let verify_result = verify_snapshot(
            &keypair.verifying_key,
            snapshot_data,
            &hash,
            &signature,
        );
        assert!(verify_result.is_ok());
    }
}
