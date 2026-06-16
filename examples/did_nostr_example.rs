//! Example: Using DID-NOSTR for request verification
//!
//! This example demonstrates how to:
//! 1. Create a DID from a NOSTR public key
//! 2. Parse a DID string
//! 3. Extract a public key from a DID
//! 4. Canonicalize a request for signing

// Define the module structure
mod did_nostr {
    use std::fmt;
    use std::str::FromStr;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NostrPublicKey {
        hex: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NostrSignature {
        hex: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DidNostr {
        pubkey: NostrPublicKey,
    }

    #[derive(Debug)]
    pub struct VerificationResult {
        pub valid: bool,
        pub did: Option<DidNostr>,
    }

    #[derive(Debug)]
    pub struct IdentityConfig {
        pub enabled: bool,
        pub methods: Vec<String>,
        pub require_signature: bool,
        pub log_identities: bool,
    }

    impl NostrPublicKey {
        pub fn from_hex(hex: &str) -> Result<Self, String> {
            if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err("Invalid hex public key".to_string());
            }
            Ok(NostrPublicKey { hex: hex.to_string() })
        }

        pub fn as_hex(&self) -> &str {
            &self.hex
        }
    }

    impl NostrSignature {
        pub fn from_hex(hex: &str) -> Result<Self, String> {
            if hex.len() != 128 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err("Invalid hex signature".to_string());
            }
            Ok(NostrSignature { hex: hex.to_string() })
        }
    }

    impl DidNostr {
        pub fn from_pubkey(pubkey: NostrPublicKey) -> Self {
            DidNostr { pubkey }
        }

        pub fn from_str(s: &str) -> Result<Self, String> {
            if !s.starts_with("did:nostr:") {
                return Err("Invalid DID format".to_string());
            }
            let hex = &s[10..]; // Skip "did:nostr:"
            let pubkey = NostrPublicKey::from_hex(hex)?;
            Ok(DidNostr { pubkey })
        }

        pub fn pubkey(&self) -> &NostrPublicKey {
            &self.pubkey
        }
    }

    impl fmt::Display for DidNostr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "did:nostr:{}", self.pubkey.hex)
        }
    }

    impl FromStr for DidNostr {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            DidNostr::from_str(s)
        }
    }

    pub struct RequestCanonicalizer;

    impl RequestCanonicalizer {
        pub fn canonicalize(method: &str, path: &str, headers: &[(String, String)], body: &str) -> String {
            let mut lines = Vec::new();
            
            // Add method and path
            lines.push(format!("{} {}", method, path));
            
            // Add headers (excluding Authorization for security)
            for (key, value) in headers {
                if key != "Authorization" {
                    lines.push(format!("{}: {}", key, value));
                }
            }
            
            // Add empty line before body
            lines.push(String::new());
            
            // Add body
            lines.push(body.to_string());
            
            lines.join("\n")
        }
    }

    pub struct NostrVerifier;

    impl NostrVerifier {
        pub fn verify(pubkey: &NostrPublicKey, message: &str, signature: &NostrSignature) -> VerificationResult {
            // In a real implementation, this would verify the NOSTR signature
            // For this example, we'll simulate verification
            let valid = !message.is_empty() && signature.hex.len() == 128;
            VerificationResult {
                valid,
                did: if valid {
                    Some(DidNostr::from_pubkey(pubkey.clone()))
                } else {
                    None
                }
            }
        }
    }

    // Helper function to create a default IdentityConfig for examples
    #[allow(dead_code)]
    pub fn default_identity_config() -> IdentityConfig {
        IdentityConfig {
            enabled: true,
            methods: vec!["did:nostr".to_string()],
            require_signature: true,
            log_identities: false,
        }
    }
}

// Import the module contents
use did_nostr::*;

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

    // Example 5: Identity configuration - using the fields to avoid dead_code warnings
    println!("\n5. Identity configuration:");
    let config = IdentityConfig {
        enabled: true,
        methods: vec!["did:nostr".to_string(), "did:web".to_string()],
        require_signature: true,
        log_identities: false,
    };
    
    // Use all fields to demonstrate the configuration
    println!("   Identity config:");
    println!("     Enabled: {}", config.enabled);
    println!("     Methods: {:?}", config.methods);
    println!("     Require signature: {}", config.require_signature);
    println!("     Log identities: {}", config.log_identities);
    
    // Demonstrate using the config in a real scenario
    if config.enabled {
        println!("   Identity verification is enabled");
        if config.require_signature {
            println!("   Signatures are required for all requests");
        }
        println!("   Supported methods: {}", config.methods.join(", "));
    }
    
    // Example of using the helper function
    #[allow(unused_variables)]
    let default_config = did_nostr::default_identity_config();
    println!("   Default config available via helper function");
}