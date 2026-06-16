#include "did_nostr.hpp"
#include "../include/did_nostr.h"
#include <algorithm>
#include <sstream>
#include <cctype>

namespace http_privacy {
namespace did_nostr {

// ============= NostrPublicKey =============

NostrPublicKey::NostrPublicKey(const std::string& hex) : hex_(hex) {
    if (!is_valid_hex(hex)) {
        throw std::invalid_argument("Invalid NOSTR public key: " + hex);
    }
    // Convert to lowercase
    std::transform(hex_.begin(), hex_.end(), hex_.begin(),
                   [](unsigned char c) { return std::tolower(c); });
}

const std::string& NostrPublicKey::as_hex() const {
    return hex_;
}

std::string NostrPublicKey::to_string() const {
    // Redacted format: npub1...{last 8 chars}
    size_t start = hex_.length() > 8 ? hex_.length() - 8 : 0;
    return "npub1..." + hex_.substr(start);
}

bool NostrPublicKey::is_valid_hex(const std::string& hex) {
    if (hex.length() != 64) {
        return false;
    }
    for (char c : hex) {
        if (!std::isxdigit(c)) {
            return false;
        }
    }
    return true;
}

// ============= DidNostr =============

DidNostr::DidNostr(const NostrPublicKey& pubkey) : pubkey_(pubkey) {}

DidNostr DidNostr::from_str(const std::string& did_str) {
    const std::string prefix = "did:nostr:";
    if (did_str.substr(0, prefix.length()) != prefix) {
        throw std::invalid_argument("Invalid DID format: " + did_str);
    }
    std::string pubkey_part = did_str.substr(prefix.length());
    return DidNostr(NostrPublicKey(pubkey_part));
}

const NostrPublicKey& DidNostr::pubkey() const {
    return pubkey_;
}

std::string DidNostr::to_string() const {
    return "did:nostr:" + pubkey_.as_hex();
}

// ============= NostrSignature =============

NostrSignature::NostrSignature(const std::string& hex) : hex_(hex) {
    if (!is_valid_hex(hex)) {
        throw std::invalid_argument("Invalid NOSTR signature: " + hex);
    }
    // Convert to lowercase
    std::transform(hex_.begin(), hex_.end(), hex_.begin(),
                   [](unsigned char c) { return std::tolower(c); });
}

const std::string& NostrSignature::as_hex() const {
    return hex_;
}

std::string NostrSignature::to_string() const {
    // Redacted format: sig_...{last 8 chars}
    size_t start = hex_.length() > 8 ? hex_.length() - 8 : 0;
    return "sig_..." + hex_.substr(start);
}

bool NostrSignature::is_valid_hex(const std::string& hex) {
    if (hex.length() != 128) {
        return false;
    }
    for (char c : hex) {
        if (!std::isxdigit(c)) {
            return false;
        }
    }
    return true;
}

// ============= NostrVerifier =============

VerificationResult NostrVerifier::verify(
    const NostrPublicKey& pubkey,
    const std::string& message,
    const NostrSignature& signature
) {
    // TODO: Implement actual secp256k1 verification
    // For now, return success for demonstration
    return VerificationResult{
        true,  // valid
        "did:nostr:" + pubkey.as_hex(),  // did
        ""  // error
    };
}

// ============= RequestCanonicalizer =============

std::string RequestCanonicalizer::canonicalize(
    const std::string& method,
    const std::string& path,
    const std::vector<std::pair<std::string, std::string>>& headers,
    const std::string& body
) {
    // Sort headers by key
    auto sorted_headers = headers;
    std::sort(sorted_headers.begin(), sorted_headers.end(),
              [](const auto& a, const auto& b) { return a.first < b.first; });

    // Build canonical string
    std::ostringstream oss;
    oss << method << "\n" << path << "\n";

    for (size_t i = 0; i < sorted_headers.size(); ++i) {
        oss << sorted_headers[i].first << ":" << sorted_headers[i].second;
        if (i < sorted_headers.size() - 1) {
            oss << "\n";
        }
    }

    oss << "\n" << body;
    return oss.str();
}

}  // namespace did_nostr
}  // namespace http_privacy
