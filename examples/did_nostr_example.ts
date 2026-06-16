/**
 * Example: Using DID-NOSTR for request verification
 *
 * This example demonstrates how to:
 * 1. Create a DID from a NOSTR public key
 * 2. Parse a DID string
 * 3. Extract a public key from a DID
 * 4. Canonicalize a request for signing
 * 5. Sign a request using NOSTR (if available)
 */

import {
  initDidNostr,
  NostrPublicKey,
  DidNostr,
  NostrSignature,
  NostrVerifier,
  RequestCanonicalizer,
  signRequest,
} from '../ts/did-nostr';

async function main() {
  console.log('=== DID-NOSTR Example ===\n');

  // Initialize WASM module
  await initDidNostr();

  // Example 1: Create a DID from a NOSTR public key
  console.log('1. Creating DID from NOSTR public key:');
  try {
    const pubkeyHex = 'a'.repeat(64);
    const pubkey = NostrPublicKey.create(pubkeyHex);
    const did = DidNostr.fromPubkey(pubkey);
    console.log(`   Created DID: ${did.toString()}`);
  } catch (e) {
    console.error(`   Error: ${e}`);
  }

  // Example 2: Parse a DID from string
  console.log('\n2. Parsing DID from string:');
  try {
    const didStr = `did:nostr:${'b'.repeat(64)}`;
    const did = DidNostr.fromString(didStr);
    console.log(`   Parsed DID: ${did.toString()}`);
    console.log(`   Public Key: ${did.getPubkey().asHex()}`);
  } catch (e) {
    console.error(`   Error: ${e}`);
  }

  // Example 3: Canonicalize a request for signing
  console.log('\n3. Canonicalizing HTTP request:');
  const method = 'POST';
  const path = '/api/v1/messages';
  const headers: [string, string][] = [
    ['Host', 'api.example.com'],
    ['Content-Type', 'application/json'],
    ['Authorization', 'Bearer token123'],
  ];
  const body = JSON.stringify({ message: 'Hello, NOSTR!' });

  const canonical = RequestCanonicalizer.canonicalize(
    method,
    path,
    headers,
    body
  );
  console.log('   Canonical form:');
  for (const line of canonical.split('\n')) {
    console.log(`     ${line}`);
  }

  // Example 4: Verify a signature (stub)
  console.log('\n4. Verifying NOSTR signature (stub):');
  try {
    const pubkey = NostrPublicKey.create('c'.repeat(64));
    const signature = NostrSignature.create('d'.repeat(128));
    const result = NostrVerifier.verify(pubkey, 'test message', signature);
    console.log(
      `   Verification result: ${result.valid ? 'Valid' : 'Invalid'}`
    );
    if (result.did) {
      console.log(`   Verified DID: ${result.did.toString()}`);
    }
  } catch (e) {
    console.error(`   Error: ${e}`);
  }

  // Example 5: Sign a request (requires NIP-07 or similar)
  console.log('\n5. Signing request with NOSTR:');
  try {
    const nostr = (window as any).nostr; // NIP-07 interface
    if (nostr) {
      const { canonical: signed_canonical, signature } = await signRequest(
        method,
        path,
        headers,
        body,
        nostr
      );
      console.log(`   Request signed: ${signature ? 'Yes' : 'No (NIP-07 not available)'}`);
    } else {
      console.log('   NIP-07 not available (expected in browser environment)');
    }
  } catch (e) {
    console.error(`   Error: ${e}`);
  }

  console.log('\n=== Example Complete ===');
}

main().catch(console.error);
