#ifndef DID_NOSTR_HPP
#define DID_NOSTR_HPP

#include <string>
#include <memory>
#include <stdexcept>

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
    explicit NostrPublicKey(const std::string& hex);

    /**
     * Get hex representation
     */
    const std::string& as_hex() const;

    /**
     * Get redacted display string
     */
    std::string to_string() const;

    /**
     * Validate hex string
     */
    static bool is_valid_hex(const std::string& hex);
};

class DidNostr {
private:
    NostrPublicKey pubkey_;

public:
    /**
     * Create DID from public key
     */
    explicit DidNostr(const NostrPublicKey& pubkey);

    /**
     * Parse DID from string (e.g., "did:nostr:...")
     */
    static DidNostr from_str(const std::string& did_str);

    /**
     * Get the underlying public key
     */
    const NostrPublicKey& pubkey() const;

    /**
     * Get DID as string
     */
    std::string to_string() const;
};

class NostrSignature {
private:
    std::string hex_;

public:
    /**
     * Create from hex string (must be 128 characters)
     */
    explicit NostrSignature(const std::string& hex);

    /**
     * Get hex representation
     */
    const std::string& as_hex() const;

    /**
     * Get redacted display string
     */
    std::string to_string() const;

    /**
     * Validate hex string
     */
    static bool is_valid_hex(const std::string& hex);
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
    );
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
    );
};

}  // namespace did_nostr
}  // namespace http_privacy

#endif  // DID_NOSTR_HPP
