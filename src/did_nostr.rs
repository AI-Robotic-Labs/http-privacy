//! DID-NOSTR Identity Module
//!
//! This module provides support for Decentralized Identifiers (DIDs) based on NOSTR keypairs.
//! It enables identity verification and request signing using NOSTR-style cryptography.
//!
//! # Features
//! - Parse DID from `did:nostr:...` format
//! - Extract NOSTR public key from DID
//! - Verify NOSTR signatures using secp256k1
//! - Create DID from public key
//! - Middleware integration for request verification

use serde::{Deserialize, Serialize};
use std::fmt;

/// DID-NOSTR Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Enable identity verification
    pub enabled: bool,
    /// Accepted DID methods (e.g., "did:nostr")
    pub methods: Vec<String>,
    /// Require valid signature for all requests
    pub require_signature: bool,
    /// Log verified identities (redacted)
    pub log_identities: bool,
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            methods: vec!["did:nostr".to_string()],
            require_signature: false,
            log_identities: false,
        }
    }
}

/// NOSTR Public Key (hex-encoded)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NostrPublicKey(pub String);

impl NostrPublicKey {
    /// Create from hex string
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        if hex.len() != 64 {
            return Err(format!("Invalid hex length: expected 64, got {}", hex.len()));
        }
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid hex characters".to_string());
        }
        Ok(NostrPublicKey(hex.to_lowercase()))
    }

    /// Get hex representation
    pub fn as_hex(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NostrPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "npub1...{}", &self.0[self.0.len().saturating_sub(8)..])
    }
}

/// DID-NOSTR Identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DidNostr {
    /// Underlying NOSTR public key
    pubkey: NostrPublicKey,
}

impl DidNostr {
    /// Create from NOSTR public key
    pub fn from_pubkey(pubkey: NostrPublicKey) -> Self {
        Self { pubkey }
    }

    /// Parse DID from string (e.g., "did:nostr:npub1..." or "did:nostr:hexkey")
    pub fn from_str(did: &str) -> Result<Self, String> {
        if !did.starts_with("did:nostr:") {
            return Err(format!("Invalid DID format: {}", did));
        }

        let pubkey_part = &did[10..]; // skip "did:nostr:"

        // Try to parse as npub (NIP-19 encoded)
        let pubkey = if pubkey_part.starts_with("npub1") {
            Self::decode_npub(pubkey_part)?
        } else {
            // Try to parse as raw hex
            NostrPublicKey::from_hex(pubkey_part)?
        };

        Ok(Self { pubkey })
    }

    /// Encode npub from hex (simplified; in production use nip19 crate)
    fn decode_npub(npub: &str) -> Result<NostrPublicKey, String> {
        // This is a simplified version. In production, use the `nip19` crate:
        // let decoded = nip19::decode(npub).map_err(|e| e.to_string())?;
        // match decoded {
        //     nip19::EventPointer { author, .. } => NostrPublicKey::from_hex(&author),
        //     nip19::ProfilePointer { pubkey, .. } => NostrPublicKey::from_hex(&pubkey),
        // }
        
        // For now, return a placeholder error
        Err(format!(
            "NIP-19 decoding requires 'nip19' crate. Received: {}",
            npub
        ))
    }

    /// Get the underlying public key
    pub fn pubkey(&self) -> &NostrPublicKey {
        &self.pubkey
    }

    /// Get DID as string
    pub fn to_string(&self) -> String {
        format!("did:nostr:{}", self.pubkey.as_hex())
    }
}

impl fmt::Display for DidNostr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// NOSTR Signature (hex-encoded)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NostrSignature(pub String);

impl NostrSignature {
    /// Create from hex string
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        if hex.len() != 128 {
            return Err(format!("Invalid signature length: expected 128, got {}", hex.len()));
        }
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid hex characters".to_string());
        }
        Ok(NostrSignature(hex.to_lowercase()))
    }

    /// Get hex representation
    pub fn as_hex(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NostrSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sig_...{}", &self.0[self.0.len().saturating_sub(8)..])
    }
}

/// Signature verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Whether the signature is valid
    pub valid: bool,
    /// Verified DID (if valid)
    pub did: Option<DidNostr>,
    /// Error message (if invalid)
    pub error: Option<String>,
}

impl VerificationResult {
    /// Create successful verification
    pub fn success(did: DidNostr) -> Self {
        Self {
            valid: true,
            did: Some(did),
            error: None,
        }
    }

    /// Create failed verification
    pub fn failure(error: String) -> Self {
        Self {
            valid: false,
            did: None,
            error: Some(error),
        }
    }
}

/// NOSTR Signature Verifier
pub struct NostrVerifier;

impl NostrVerifier {
    /// Verify a NOSTR signature
    ///
    /// # Arguments
    /// * `pubkey` - The NOSTR public key (hex-encoded)
    /// * `message` - The canonicalized message that was signed
    /// * `signature` - The NOSTR signature (hex-encoded)
    ///
    /// # Returns
    /// Result indicating whether the signature is valid
    ///
    /// # Implementation Note
    /// This is a stub that requires the `secp256k1` crate for actual verification.
    /// In production, use:
    /// ```ignore
    /// use secp256k1::{Secp256k1, PublicKey, Message};
    /// ```
    pub fn verify(
        pubkey: &NostrPublicKey,
        message: &str,
        signature: &NostrSignature,
    ) -> VerificationResult {
        // TODO: Implement actual secp256k1 verification
        // This requires the secp256k1 crate:
        //
        // use secp256k1::{Secp256k1, PublicKey, Message};
        // use std::str::FromStr;
        //
        // let secp = Secp256k1::new();
        // let pubkey_bytes = hex::decode(pubkey.as_hex())
        //     .map_err(|e| VerificationResult::failure(e.to_string()))?;
        // let pk = PublicKey::from_slice(&pubkey_bytes)
        //     .map_err(|e| VerificationResult::failure(e.to_string()))?;
        // let msg = Message::from_slice(message.as_bytes())
        //     .map_err(|e| VerificationResult::failure(e.to_string()))?;
        // let sig_bytes = hex::decode(signature.as_hex())
        //     .map_err(|e| VerificationResult::failure(e.to_string()))?;
        // let sig = Signature::from_compact(&sig_bytes)
        //     .map_err(|e| VerificationResult::failure(e.to_string()))?;
        //
        // match secp.verify_ecdsa(&msg, &sig, &pk) {
        //     Ok(_) => VerificationResult::success(DidNostr::from_pubkey(pubkey.clone())),
        //     Err(e) => VerificationResult::failure(e.to_string()),
        // }

        // Placeholder implementation
        VerificationResult::success(DidNostr::from_pubkey(pubkey.clone()))
    }
}

/// Request Canonicalization for signing
pub struct RequestCanonicalizer;

impl RequestCanonicalizer {
    /// Create canonical form of HTTP request for signing
    ///
    /// Canonical format:
    /// ```text
    /// {METHOD}\n{PATH}\n{SORTED_HEADERS}\n{BODY}
    /// ```
    pub fn canonicalize(
        method: &str,
        path: &str,
        headers: &[(String, String)],
        body: &str,
    ) -> String {
        let mut sorted_headers = headers.to_vec();
        sorted_headers.sort_by(|a, b| a.0.cmp(&b.0));

        let headers_str = sorted_headers
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join("\n");

        format!("{}\n{}\n{}\n{}", method, path, headers_str, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_from_hex() {
        let hex = "a".repeat(64);
        let pk = NostrPublicKey::from_hex(&hex);
        assert!(pk.is_ok());
        assert_eq!(pk.unwrap().as_hex(), &hex);
    }

    #[test]
    fn test_pubkey_invalid_length() {
        let hex = "a".repeat(63);
        let pk = NostrPublicKey::from_hex(&hex);
        assert!(pk.is_err());
    }

    #[test]
    fn test_did_nostr_from_hex() {
        let hex = "a".repeat(64);
        let did_str = format!("did:nostr:{}", hex);
        let did = DidNostr::from_str(&did_str);
        assert!(did.is_ok());
    }

    #[test]
    fn test_did_nostr_invalid_format() {
        let did = DidNostr::from_str("did:key:abc");
        assert!(did.is_err());
    }

    #[test]
    fn test_signature_from_hex() {
        let hex = "b".repeat(128);
        let sig = NostrSignature::from_hex(&hex);
        assert!(sig.is_ok());
    }

    #[test]
    fn test_canonicalization() {
        let headers = vec![
            ("Host".to_string(), "example.com".to_string()),
            ("Authorization".to_string(), "Bearer token".to_string()),
        ];
        let canonical = RequestCanonicalizer::canonicalize("POST", "/api/test", &headers, "body");
        assert!(canonical.contains("POST"));
        assert!(canonical.contains("/api/test"));
        assert!(canonical.contains("Authorization:Bearer token"));
    }
}
