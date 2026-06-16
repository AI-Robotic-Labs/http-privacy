#!/usr/bin/env python3
"""Example: Using DID-NOSTR for request verification

This example demonstrates how to:
1. Create a DID from a NOSTR public key
2. Parse a DID string
3. Extract a public key from a DID
4. Canonicalize a request for signing
"""

from http_client_module import did_nostr

def main():
    print("=== DID-NOSTR Example ===")

    # Example 1: Create a DID from a NOSTR public key
    print("\n1. Creating DID from NOSTR public key:")
    try:
        pubkey_hex = "a" * 64
        pubkey = did_nostr.PyNostrPublicKey(pubkey_hex)
        did = did_nostr.PyDidNostr.from_pubkey(pubkey)
        print(f"   Created DID: {did}")
    except Exception as e:
        print(f"   Error: {e}")

    # Example 2: Parse a DID from string
    print("\n2. Parsing DID from string:")
    try:
        did_str = f"did:nostr:{'b' * 64}"
        did = did_nostr.PyDidNostr.from_str(did_str)
        print(f"   Parsed DID: {did}")
        print(f"   Public Key: {did.pubkey().as_hex()}")
    except Exception as e:
        print(f"   Error: {e}")

    # Example 3: Canonicalize a request for signing
    print("\n3. Canonicalizing HTTP request:")
    method = "POST"
    path = "/api/v1/messages"
    headers = [
        ("Host", "api.example.com"),
        ("Content-Type", "application/json"),
        ("Authorization", "Bearer token123"),
    ]
    body = '{"message": "Hello, NOSTR!"}'

    canonical = did_nostr.PyRequestCanonicalizer.canonicalize(
        method, path, headers, body
    )
    print("   Canonical form:")
    for line in canonical.split("\n"):
        print(f"     {line}")

    # Example 4: Verify a signature (stub)
    print("\n4. Verifying NOSTR signature (stub):")
    try:
        pubkey = did_nostr.PyNostrPublicKey("c" * 64)
        signature = did_nostr.PyNostrSignature("d" * 128)
        result = did_nostr.PyNostrVerifier.verify(
            pubkey, "test message", signature
        )
        print(f"   Verification result: {'Valid' if result.valid else 'Invalid'}")
        if result.did:
            print(f"   Verified DID: {result.did}")
    except Exception as e:
        print(f"   Error: {e}")

    print("\n=== Example Complete ===")

if __name__ == "__main__":
    main()
