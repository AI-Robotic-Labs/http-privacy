//! WASM bindings for DID-NOSTR functionality via wasm-bindgen

use wasm_bindgen::prelude::*;
use crate::did_nostr::*;

/// JavaScript wrapper for NostrPublicKey
#[wasm_bindgen]
pub struct JsNostrPublicKey {
    inner: NostrPublicKey,
}

#[wasm_bindgen]
impl JsNostrPublicKey {
    /// Create from hex string
    #[wasm_bindgen(constructor)]
    pub fn new(hex: String) -> Result<JsNostrPublicKey, JsValue> {
        let inner = NostrPublicKey::from_hex(&hex)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(JsNostrPublicKey { inner })
    }

    /// Get hex representation
    pub fn as_hex(&self) -> String {
        self.inner.as_hex().to_string()
    }

    /// Get display string (redacted)
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

/// JavaScript wrapper for DidNostr
#[wasm_bindgen]
pub struct JsDidNostr {
    inner: DidNostr,
}

#[wasm_bindgen]
impl JsDidNostr {
    /// Create DID from public key
    pub fn from_pubkey(pubkey: &JsNostrPublicKey) -> JsDidNostr {
        JsDidNostr {
            inner: DidNostr::from_pubkey(pubkey.inner.clone()),
        }
    }

    /// Parse DID from string
    pub fn from_str(did_str: String) -> Result<JsDidNostr, JsValue> {
        let inner = DidNostr::from_str(&did_str)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(JsDidNostr { inner })
    }

    /// Get public key
    pub fn pubkey(&self) -> JsNostrPublicKey {
        JsNostrPublicKey {
            inner: self.inner.pubkey().clone(),
        }
    }

    /// Get DID as string
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

/// JavaScript wrapper for NostrSignature
#[wasm_bindgen]
pub struct JsNostrSignature {
    inner: NostrSignature,
}

#[wasm_bindgen]
impl JsNostrSignature {
    /// Create from hex string
    #[wasm_bindgen(constructor)]
    pub fn new(hex: String) -> Result<JsNostrSignature, JsValue> {
        let inner = NostrSignature::from_hex(&hex)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(JsNostrSignature { inner })
    }

    /// Get hex representation
    pub fn as_hex(&self) -> String {
        self.inner.as_hex().to_string()
    }

    /// Get display string (redacted)
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

/// JavaScript wrapper for VerificationResult
#[wasm_bindgen]
pub struct JsVerificationResult {
    valid: bool,
    did: Option<JsDidNostr>,
    error: Option<String>,
}

#[wasm_bindgen]
impl JsVerificationResult {
    /// Check if verification was successful
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Get verified DID (if successful)
    pub fn get_did(&self) -> Option<JsDidNostr> {
        self.did.clone()
    }

    /// Get error message (if failed)
    pub fn get_error(&self) -> Option<String> {
        self.error.clone()
    }
}

impl Clone for JsDidNostr {
    fn clone(&self) -> Self {
        JsDidNostr {
            inner: self.inner.clone(),
        }
    }
}

impl Clone for JsVerificationResult {
    fn clone(&self) -> Self {
        JsVerificationResult {
            valid: self.valid,
            did: self.did.clone(),
            error: self.error.clone(),
        }
    }
}

/// JavaScript wrapper for NostrVerifier
#[wasm_bindgen]
pub struct JsNostrVerifier;

#[wasm_bindgen]
impl JsNostrVerifier {
    /// Verify a NOSTR signature
    pub fn verify(
        pubkey: &JsNostrPublicKey,
        message: String,
        signature: &JsNostrSignature,
    ) -> JsVerificationResult {
        let result = NostrVerifier::verify(&pubkey.inner, &message, &signature.inner);
        JsVerificationResult {
            valid: result.valid,
            did: result.did.map(|d| JsDidNostr { inner: d }),
            error: result.error,
        }
    }
}

/// JavaScript wrapper for RequestCanonicalizer
#[wasm_bindgen]
pub struct JsRequestCanonicalizer;

#[wasm_bindgen]
impl JsRequestCanonicalizer {
    /// Create canonical form of HTTP request for signing
    /// Headers should be an array of [key, value] pairs
    pub fn canonicalize(
        method: String,
        path: String,
        headers: js_sys::Array,
        body: String,
    ) -> String {
        let mut headers_vec = Vec::new();
        for item in headers.iter() {
            let arr = js_sys::Array::from(&item);
            if arr.length() >= 2 {
                let k = arr.get(0).as_string().unwrap_or_default();
                let v = arr.get(1).as_string().unwrap_or_default();
                headers_vec.push((k, v));
            }
        }
        RequestCanonicalizer::canonicalize(&method, &path, &headers_vec, &body)
    }
}
