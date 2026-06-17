#ifndef DID_NOSTR_HPP
#define DID_NOSTR_HPP

#include <string>
#include <memory>
#include <stdexcept>
#include <algorithm> // For std::sort

/**
 * C++ Wrapper for DID-NOSTR functionality
 *
 * This provides a safe, exception-based C++ interface to the Rust DID-NOSTR library.
 */

namespace http_privacy {
namespace did_nostr {

class NostrPublicKey {
private:
    std::string hex_;

public:
    /**
     * Create from hex string (must be 64 characters)
     */
    explicit NostrPublicKey(const std::string& hex) : hex_(hex) {
        if (!is_valid_hex(hex)) {
            throw std::invalid_argument("Invalid Nostr public key hex string. Must be 64 hex characters.");
        }
    }

    /**
     * Get hex representation
     */
    const std::string& as_hex() const;

    /**
     * Get redacted display string
     */
    std::string to_string() const {
        if (hex_.length() <= 16) {
            return hex_;
        }
        return hex_.substr(0, 8) + "..." + hex_.substr(hex_.length() - 8);
    }

    /**
     * Validate hex string
     */
    static bool is_valid_hex(const std::string& hex) {
        if (hex.length() != 64) return false;
        return std::all_of(hex.begin(), hex.end(), [](char c) {
            return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
        });
    }
};

class DidNostr {
private:
    NostrPublicKey pubkey_;

public:
    /**
     * Create DID from public key
     */
    explicit DidNostr(const NostrPublicKey& pubkey) : pubkey_(pubkey) {}

    /**
     * Parse DID from string (e.g., "did:nostr:...")
     */
    static DidNostr from_str(const std::string& did_str) {
        if (!did_str.starts_with("did:nostr:")) {
            throw std::invalid_argument("Invalid DID-NOSTR format. Must start with 'did:nostr:'.");
        }
        std::string hex = did_str.substr(10); // Skip "did:nostr:"
        return DidNostr(NostrPublicKey(hex));
    }

    /**
     * Get the underlying public key
     */
    const NostrPublicKey& pubkey() const { return pubkey_; }

    /**
     * Get DID as string
     */
    std::string to_string() const {
        return "did:nostr:" + pubkey_.as_hex();
    }
};

class NostrSignature {
private:
    std::string hex_;

public:
    /**
     * Create from hex string (must be 128 characters)
     */
    explicit NostrSignature(const std::string& hex) : hex_(hex) {
        if (!is_valid_hex(hex)) {
            throw std::invalid_argument("Invalid Nostr signature hex string. Must be 128 hex characters.");
        }
    }

    /**
     * Get hex representation
     */
    const std::string& as_hex() const { return hex_; }

    /**
     * Get redacted display string
     */
    std::string to_string() const {
        if (hex_.length() <= 16) {
            return hex_;
        }
        return hex_.substr(0, 8) + "..." + hex_.substr(hex_.length() - 8);
    }

    /**
     * Validate hex string
     */
    static bool is_valid_hex(const std::string& hex) {
        if (hex.length() != 128) return false;
        return std::all_of(hex.begin(), hex.end(), [](char c) {
            return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
        });
    }
};

struct VerificationResult {
    bool valid;
    std::string did;  // Empty if verification failed
    std::string error;  // Empty if verification succeeded

    bool is_valid() const { return valid; }
};

class NostrVerifier {
public:
    /**
     * Verify a NOSTR signature
     *
     * @param pubkey The NOSTR public key (hex-encoded)
     * @param message The canonicalized message that was signed
     * @param signature The NOSTR signature (hex-encoded)
     * @return VerificationResult indicating success or failure
     */
    static VerificationResult verify(
        const NostrPublicKey& pubkey,
        const std::string& message,
        const NostrSignature& signature
    ) {
        // --- SIMULATED VERIFICATION ---
        // In a real scenario, this would call into the Rust core for actual cryptographic verification.
        // For this example, we simulate success if the message is not empty and signature length is correct.
        bool valid = !message.empty() && signature.as_hex().length() == 128;
        std::string error_msg = valid ? "" : "Simulated verification failed.";
        return {valid, valid ? pubkey.as_hex() : "", error_msg};
    }
};

class RequestCanonicalizer {
public:
    /**
     * Create canonical form of HTTP request for signing
     *
     * Canonical format:
     * {METHOD}\n{PATH}\n{SORTED_HEADERS}\n{BODY}
     *
     * @param method HTTP method (GET, POST, etc.)
     * @param path Request path
     * @param headers Vector of {key, value} pairs
     * @param body Request body
     * @return Canonical string representation
     */
    static std::string canonicalize(
        const std::string& method,
        const std::string& path,
        const std::vector<std::pair<std::string, std::string>>& headers,
        const std::string& body
    ) {
        std::string canonical_form;
        canonical_form += method + "\n";
        canonical_form += path + "\n";

        // Sort headers by key for consistent canonicalization
        std::vector<std::pair<std::string, std::string>> sorted_headers = headers;
        std::sort(sorted_headers.begin(), sorted_headers.end(),
                  [](const auto& a, const auto& b) {
                      return a.first < b.first;
                  });

        for (const auto& header : sorted_headers) {
            // Exclude Authorization header from canonicalization for security reasons
            if (header.first != "Authorization" && header.first != "authorization") {
                canonical_form += header.first + ":" + header.second + "\n";
            }
        }
        canonical_form += "\n"; // Empty line before body
        canonical_form += body;
        return canonical_form;
    }
};

}  // namespace did_nostr
}  // namespace http_privacy

#endif  // DID_NOSTR_HPP
