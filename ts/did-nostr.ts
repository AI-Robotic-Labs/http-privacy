/**
 * TypeScript/JavaScript client library for DID-NOSTR integration
 *
 * This library provides utilities for creating DIDs, signing requests,
 * and verifying signatures using NOSTR keypairs.
 */

import init, {
  JsNostrPublicKey,
  JsDidNostr,
  JsNostrSignature,
  JsNostrVerifier,
  JsRequestCanonicalizer,
} from './pkg/privacy_http_sdk';

/**
 * Initialize WASM module
 */
export async function initDidNostr(): Promise<void> {
  await init();
}

/**
 * NostrPublicKey wrapper
 */
export class NostrPublicKey {
  private inner: JsNostrPublicKey;

  private constructor(inner: JsNostrPublicKey) {
    this.inner = inner;
  }

  static create(hex: string): NostrPublicKey {
    return new NostrPublicKey(new JsNostrPublicKey(hex));
  }

  asHex(): string {
    return this.inner.as_hex();
  }

  toString(): string {
    return this.inner.to_string();
  }
}

/**
 * DID-NOSTR identifier
 */
export class DidNostr {
  private inner: JsDidNostr;

  private constructor(inner: JsDidNostr) {
    this.inner = inner;
  }

  /**
   * Create DID from public key
   */
  static fromPubkey(pubkey: NostrPublicKey): DidNostr {
    const inner = JsDidNostr.from_pubkey((pubkey as any).inner);
    return new DidNostr(inner);
  }

  /**
   * Parse DID from string
   */
  static fromString(didStr: string): DidNostr {
    const inner = JsDidNostr.from_str(didStr);
    return new DidNostr(inner);
  }

  getPubkey(): NostrPublicKey {
    const pk = this.inner.pubkey();
    return new NostrPublicKey(pk as any);
  }

  toString(): string {
    return this.inner.to_string();
  }
}

/**
 * NOSTR Signature
 */
export class NostrSignature {
  private inner: JsNostrSignature;

  private constructor(inner: JsNostrSignature) {
    this.inner = inner;
  }

  static create(hex: string): NostrSignature {
    return new NostrSignature(new JsNostrSignature(hex));
  }

  asHex(): string {
    return this.inner.as_hex();
  }

  toString(): string {
    return this.inner.to_string();
  }
}

/**
 * Verification result
 */
export interface VerificationResult {
  valid: boolean;
  did?: DidNostr;
  error?: string;
}

/**
 * NOSTR Signature Verifier
 */
export class NostrVerifier {
  /**
   * Verify a NOSTR signature
   */
  static verify(
    pubkey: NostrPublicKey,
    message: string,
    signature: NostrSignature
  ): VerificationResult {
    const result = JsNostrVerifier.verify(
      (pubkey as any).inner,
      message,
      (signature as any).inner
    );

    return {
      valid: result.is_valid(),
      did: result.get_did()
        ? new DidNostr(result.get_did())
        : undefined,
      error: result.get_error() || undefined,
    };
  }
}

/**
 * HTTP Request Canonicalizer
 */
export class RequestCanonicalizer {
  /**
   * Create canonical form of HTTP request for signing
   */
  static canonicalize(
    method: string,
    path: string,
    headers: [string, string][],
    body: string
  ): string {
    // Convert headers to JS Array format
    const headersArray = (headers as any) as unknown[];
    return JsRequestCanonicalizer.canonicalize(
      method,
      path,
      headersArray as any,
      body
    );
  }
}

/**
 * Convenience function to sign a request
 *
 * This is a simplified example. In production, use nostr-tools or similar.
 */
export function createDIDHeader(pubkey: NostrPublicKey): string {
  const did = DidNostr.fromPubkey(pubkey);
  return `DID: ${did.toString()}`;
}

/**
 * Example: Sign a request with NOSTR signature
 *
 * Note: This requires integration with actual NOSTR key management,
 * such as via NIP-07 (window.nostr) in the browser.
 */
export async function signRequest(
  method: string,
  path: string,
  headers: [string, string][],
  body: string,
  nostrWindow?: any
): Promise<{ canonical: string; signature?: string }> {
  const canonical = RequestCanonicalizer.canonicalize(
    method,
    path,
    headers,
    body
  );

  // If NIP-07 is available, sign using window.nostr
  let signature: string | undefined;
  if (nostrWindow?.signEvent) {
    try {
      const event = {
        kind: 27235,  // HTTP auth kind
        content: canonical,
        created_at: Math.floor(Date.now() / 1000),
        tags: [],
      };
      const signed = await nostrWindow.signEvent(event);
      signature = signed.sig;
    } catch (err) {
      console.error('Failed to sign with NIP-07:', err);
    }
  }

  return { canonical, signature };
}
