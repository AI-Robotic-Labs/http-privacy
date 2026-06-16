/**
 * DID-NOSTR C++ Header
 *
 * This header provides C bindings for DID-NOSTR functionality.
 * All strings are null-terminated UTF-8.
 */

#ifndef DID_NOSTR_H
#define DID_NOSTR_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Create a DID from a NOSTR public key (hex)
 *
 * @param pubkey_hex 64-character hex string representing NOSTR public key
 * @return Allocated string containing the DID or error message
 *         Must be freed with did_nostr_free()
 */
char* did_nostr_create(const char* pubkey_hex);

/**
 * Parse a DID from string
 *
 * @param did_str DID string in format "did:nostr:..."
 * @return Allocated string containing the parsed DID or error message
 *         Must be freed with did_nostr_free()
 */
char* did_nostr_parse(const char* did_str);

/**
 * Extract public key from DID
 *
 * @param did_str DID string in format "did:nostr:..."
 * @return Allocated string containing the public key (hex) or error message
 *         Must be freed with did_nostr_free()
 */
char* did_nostr_get_pubkey(const char* did_str);

/**
 * Free a string allocated by the Rust library
 *
 * @param ptr Pointer to string allocated by did_nostr_* functions
 */
void did_nostr_free(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // DID_NOSTR_H
