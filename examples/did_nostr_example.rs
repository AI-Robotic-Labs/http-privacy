//! Example: Using DID-NOSTR for request verification
//!
//! This example demonstrates how to:
//! 1. Create a DID from a NOSTR public key
//! 2. Parse a DID string
//! 3. Extract a public key from a DID
//! 4. Canonicalize a request for signing

use privacy_http_sdk::did_nostr::*;

fn main() {
    println!("=== DID-NOSTR Example ===");

    // Example 1: Create a DID from a NOSTR public key
    println!("\n1. Creating DID from NOSTR public key:");
    let pubkey_hex = "a".repeat(64);
    match NostrPublicKey::from_hex(&pubkey_hex) {
        Ok(pubkey) => {
            let did = DidNostr::from_pubkey(pubkey);
            println!("   Created DID: {}", did);
            println!("   Full DID: {}", did.to_string());
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 2: Parse a DID from string
    println!("\n2. Parsing DID from string:");
    let did_str = format!("did:nostr:{}", "b".repeat(64));
    match DidNostr::from_str(&did_str) {
        Ok(did) => {
            println!("   Parsed DID: {}", did);
            println!("   Public Key: {}", did.pubkey().as_hex());
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Example 3: Canonicalize a request for signing
    println!("\n3. Canonicalizing HTTP request:");
    let method = "POST";
    let path = "/api/v1/messages";
    let headers = vec![
        ("Host".to_string(), "api.example.com".to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Authorization".to_string(), "Bearer token123".to_string()),
    ];
    let body = r#"{"message": "Hello, NOSTR!"}"#;

    let canonical = RequestCanonicalizer::canonicalize(method, path, &headers, body);
    println!("   Canonical form:");
    for line in canonical.lines() {
        println!("     {}", line);
    }

    // Example 4: Verify a signature (stub)
    println!("\n4. Verifying NOSTR signature (stub):");
    let pubkey = NostrPublicKey::from_hex(&("c".repeat(64))).unwrap();
    let signature = NostrSignature::from_hex(&("d".repeat(128))).unwrap();
    let result = NostrVerifier::verify(&pubkey, "test message", &signature);
    println!("   Verification result: {}", if result.valid { "Valid" } else { "Invalid" });
    if let Some(did) = result.did {
        println!("   Verified DID: {}", did);
    }

    // Example 5: Identity configuration
    println!("\n5. Identity configuration:");
    let config = IdentityConfig {
        enabled: true,
        methods: vec!["did:nostr".to_string()],
        require_signature: true,
        log_identities: false,
    };
    println!("   Config: {:#?}", config);
}
