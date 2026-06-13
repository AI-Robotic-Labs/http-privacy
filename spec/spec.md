# 🔐 HTTP Privacy Specification v1.1

**Repository**: [AI-Robotic-Labs/http-privacy](https://github.com/AI-Robotic-Labs/http-privacy)

**Goal**: Empower developers to create modular, privacy-respecting HTTP clients or middleware in any environment (Rust, Python, WASM) following a unified spec. Your `HttpClient` already implements some core aspects.

---

## 📦 Core Capabilities (with Mapping to Implementation)

| Feature             | Spec Requirement           | Your `HttpClient` Support                         |
| ------------------- | -------------------------- | ------------------------------------------------- |
| Header Filtering    | `whitelist` or `blacklist` | ✅ `js_headers_to_vec`, manual header control      |
| Payload Obfuscation | `base64`, `xor`, `aes-gcm` | ✅ `base64::decode`, extendable                    |
| IP Masking          | Remove or override headers | ⚠️ Manual header strip needed                     |
| TLS Enforcement     | Force HTTPS or HSTS        | ✅ `https_only(true)` in `reqwest::Client`         |
| Logging Control     | Redact / suppress logs     | 🚫 Not yet implemented (placeholder fields exist) |

---

## 🧱 Architecture Overview

You support dual environments:

### 1. WASM / JS (via `wasm_bindgen`)

* Methods: `get_sync`, `post_sync`, `generate_image_sync`
* Input: `JsValue` headers → `[key, value]` arrays
* Output: `Result<String, JsValue>`

### 2. Python (via `PyO3`)

* Exposed: `HttpClientPy` class
* Methods: `get`, `post`, `generate_image`
* Integrated: error mapping, base64 decoding, file I/O

---

## 🔁 Request Flow (Middleware Logic)

```plaintext
[Client Request]
→ Parse Headers (Vec<(String, String)>)
→ Apply Privacy Rules:
   → Filter headers (blacklist/whitelist)
   → Obfuscate payload (if enabled)
   → Mask IP headers (if enabled)
→ Enforce TLS
→ Execute HTTP Request via reqwest
```

```plaintext
[Target Server Response]
→ Deobfuscate payload
→ Filter response headers
→ Return result to JS or Python
```

---

## ✅ JSON Config Example

To integrate a configuration-driven privacy layer:

```json
{
  "filter_headers": {
    "mode": "blacklist",
    "headers": ["User-Agent", "Referer", "Cookie"]
  },
  "obfuscation": {
    "enabled": true,
    "method": "base64"
  },
  "ip_masking": {
    "enabled": true,
    "remove_headers": ["X-Forwarded-For", "CF-Connecting-IP"]
  },
  "tls_enforce": true,
  "logging": "metadata"
}
```

This config can be injected into `HttpClient` on creation or dynamically applied per request.

---

## ⚙️ Developer Implementation Notes

To implement your own bindings or middleware:

* Use `Client::builder().https_only(true)` to enforce HTTPS.
* Provide header filtering logic before request.
* Enable `base64`, `xor`, or `aes-gcm` encoding for payloads.
* Strip common IP-related headers unless proxying is intentional.
* Avoid logging sensitive data (e.g., redact tokens).
* Keep the design modular (`HttpClient`, `HttpClientPy`, etc).

---

## 🧩 Optional Enhancements

| Feature                     | Description                                                     |
| --------------------------- | --------------------------------------------------------------- |
| WASM-Compatible Obfuscation | Implement `aes-gcm` via `wasm-bindgen` or native Rust crate     |
| IP Header Masking           | Strip `X-Forwarded-For` inside `get/post`                       |
| Configurable JSON Inputs    | Accept config objects for each call                             |
| Privacy Budget              | Rate limit per IP/request/session                               |
| C++ Bridge                  | Already partially wired via `cxx::bridge` (`greet` placeholder) |

---

## 🔒 Security Best Practices

* ❌ No hardcoded secrets.
* ✅ Use `https_only` for TLS.
* 🔐 Add TLS pinning if needed (`rustls` + cert hash).
* 🧼 Sanitize header values and logs.
* 🛡️ Validate JSON input fields.

---

## 📄 License

MIT

---

## 🙌 Contributing

* Submit your Rust/Python/WASM crates via PR
* Suggest additional obfuscation strategies or privacy enhancements
* Help improve conformance tests for middleware plugins

---

## 🔧 Developer Guide: Creating Language Bindings from `lib.rs`

This guide explains how to use the existing `lib.rs` foundation in `http-privacy` to build native bindings for other languages like:

* **TypeScript (via WASM)**
* **Dart / Flutter**
* **C / C++**
* **Python (already implemented via PyO3)**

---

### 🧩 1. TypeScript / JavaScript (WASM)

Leverage `wasm-bindgen` and `wasm-pack` to generate bindings:

**Update `Cargo.toml`:**

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

**Command to Build:**

```bash
wasm-pack build --target bundler --out-dir pkg
```

**In TypeScript:**

```ts
import init, { HttpClient } from './pkg/http_privacy';

async function run() {
  await init();
  const client = new HttpClient("your-api-key");
  const response = client.get_sync("https://example.com", []);
  console.log(response);
}
```

> ✅ Ideal for Node.js, Deno, or browser environments

---

### 📱 2. Dart / Flutter (via `flutter_rust_bridge`)

Install:

```bash
cargo install flutter_rust_bridge_codegen
```

**Project Layout:**

```
rust/             ← contains lib.rs
flutter_app/      ← your Flutter frontend
```

**In `lib.rs`:**

```rust
use flutter_rust_bridge::frb;

#[frb]
pub fn greet(name: String) -> String {
    format!("Hello, {name}")
}
```

**Generate Bridge:**

```bash
flutter_rust_bridge_codegen \
  --rust-input rust/src/lib.rs \
  --dart-output flutter_app/lib/bridge_generated.dart
```

**In Flutter:**

```dart
import 'bridge_generated.dart';

final api = createRustImpl();
final message = await api.greet("Dart");
```

> ✅ Full async support with `tokio` + stream bridging available

---

### ⚙️ 3. C / C++ (via `cxx` or `cbindgen`)

You already use `cxx::bridge`. Extend it like this:

**In `lib.rs`:**

```rust
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn greet(name: &str) -> String;
    }
}
```

**Generate Header:**

```bash
cbindgen --config cbindgen.toml --crate http_privacy --output include/http_privacy.h
```

**In C/C++:**

```cpp
#include "http_privacy.h"

extern "C" {
  const char* greet(const char* name);
}
```

> ✅ Use `cbindgen` for C headers, or `cxx` for safe C++ interop

---

### 🐍 4. Python (via `PyO3`)

Already implemented:

```python
from http_client_module import HttpClientPy

client = HttpClientPy(api_key="...", openai_url="...")
response = client.get("https://example.com", [("Authorization", "Bearer ...")])
```

To build:

```bash
maturin develop
# or
python3 setup.py develop
```
### 🍏 Swift Integration Example (macOS / iOS)

**Generate C Header:**

```bash
cbindgen --crate http_privacy --output include/http_privacy.h
```

**module.modulemap:**

```modulemap
module HttpPrivacy {
  header "http_privacy.h"
  export *
}
```

**Swift Bridging Example:**

```swift
import HttpPrivacy

let message = greet("Swift user")
print(String(cString: message))
```

---

### 🤖 Kotlin/Native Example (Multiplatform)

**Rust (lib.rs):**

```rust
#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = c_str.to_str().unwrap_or("unknown");
    let result = format!("Hello, {name_str}");
    CString::new(result).unwrap().into_raw()
}
```

**Kotlin (using cinterop):**

```kotlin
fun main() {
    val result = greet("Kotlin")
    println(result?.toKString())
}
```

> Use Gradle plugin `konan` for native interop.

---

### 🦫 Go Integration Example (CGO)

**Rust (lib.rs):**

```rust
#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *const c_char {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("User");
    CString::new(format!("Hello, {name}")).unwrap().into_raw()
}
```

**Build as Shared Library:**

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

**Go Wrapper:**

```go
/*
#cgo LDFLAGS: -L. -lhttp_privacy
#include "http_privacy.h"
*/
import "C"
import "fmt"

func main() {
    name := C.CString("Go User")
    defer C.free(unsafe.Pointer(name))
    msg := C.greet(name)
    fmt.Println(C.GoString(msg))
}
```

---

By using these language bridges, you ensure full privacy-aware HTTP logic is **centralized in Rust** and reused across:

* 🧠 iOS/macOS (Swift)
* 📱 Android (Kotlin)
* 🐹 Backend / CLI tools (Go)
* 🌐 Web (WASM)
* 🐍 Python

---

## 🔐 Privacy-Conforming Bindings

Regardless of the language:

* Always apply header filtering and TLS checks before sending
* If using obfuscation (`base64`, `aes-gcm`), expose encoder/decoder to bindings
* All errors should be safely catchable in the target language
