#include <iostream>
#include <vector>
#include "../cpp/did_nostr.hpp"

using namespace http_privacy::did_nostr;

int main() {
    std::cout << "=== DID-NOSTR Example ===\n" << std::endl;

    try {
        // Example 1: Create a DID from a NOSTR public key
        std::cout << "1. Creating DID from NOSTR public key:" << std::endl;
        std::string pubkey_hex(64, 'a');
        NostrPublicKey pubkey(pubkey_hex);
        DidNostr did(pubkey);
        std::cout << "   Created DID: " << did.to_string() << std::endl;

        // Example 2: Parse a DID from string
        std::cout << "\n2. Parsing DID from string:" << std::endl;
        std::string did_str = "did:nostr:" + std::string(64, 'b');
        DidNostr parsed_did = DidNostr::from_str(did_str);
        std::cout << "   Parsed DID: " << parsed_did.to_string() << std::endl;
        std::cout << "   Public Key: " << parsed_did.pubkey().as_hex() << std::endl;

        // Example 3: Canonicalize a request for signing
        std::cout << "\n3. Canonicalizing HTTP request:" << std::endl;
        std::string method = "POST";
        std::string path = "/api/v1/messages";
        std::vector<std::pair<std::string, std::string>> headers = {
            {"Host", "api.example.com"},
            {"Content-Type", "application/json"},
            {"Authorization", "Bearer token123"},
        };
        std::string body = R"({"message": "Hello, NOSTR!"})";

        std::string canonical = RequestCanonicalizer::canonicalize(
            method, path, headers, body
        );
        std::cout << "   Canonical form:" << std::endl;
        for (const auto& line : canonical) {
            if (line == '\n') {
                std::cout << std::endl << "     ";
            } else {
                std::cout << line;
            }
        }
        std::cout << std::endl;

        // Example 4: Verify a signature (stub)
        std::cout << "\n4. Verifying NOSTR signature (stub):" << std::endl;
        NostrPublicKey verify_pubkey(std::string(64, 'c'));
        NostrSignature signature(std::string(128, 'd'));
        VerificationResult result = NostrVerifier::verify(
            verify_pubkey, "test message", signature
        );
        std::cout << "   Verification result: "
                  << (result.is_valid() ? "Valid" : "Invalid") << std::endl;
        if (result.is_valid()) {
            std::cout << "   Verified DID: " << result.did << std::endl;
        }

        std::cout << "\n=== Example Complete ===" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}
