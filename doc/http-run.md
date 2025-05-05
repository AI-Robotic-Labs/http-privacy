
# HTTP Servers Setup Guide (C++, Python, Rust, JavaScript)


This document provides instructions to set up and run HTTP servers in **C++**, **Python**, **Rust**, and **JavaScript** from the `http-privacy` repository. Follow these steps carefully to avoid setup issues or Markdown rendering problems.


## 📥 Clone the Repository

```bash
git clone https://github.com/AI-Robotic-Labs/http-privacy.git
````
---

## ⚙️ C++ Server (`http-c++`)

### Prerequisites

* **C++ Compiler**: GCC, Clang, or MSVC (C++17+)
* **CMake**: Version 3.10+
* **Dependencies**: Boost.Asio, OpenSSL, spdlog

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y g++ cmake libboost-all-dev libssl-dev libspdlog-dev
```

#### macOS (Homebrew)

```bash
brew install cmake boost openssl spdlog
```

### Build and Run

```bash
cd http-privacy/http-c++
mkdir build && cd build
cmake ..
make
./http_server
```

### Test

```bash
curl http://localhost:8080
```

### Configuration

* **Port**: Edit `server.cpp`.
* **HTTPS**: Provide `cert.pem` and `key.pem` in the project root and enable SSL in `server.cpp`.

---

## 🐍 Python Server (`http-py`)

### Prerequisites

* **Python**: 3.8+
* **Dependencies**:

```bash
pip install fastapi uvicorn pydantic
```

### Run

```bash
cd http-privacy/http-py
uvicorn main:app --host 0.0.0.0 --port 8000
```

### Test

```bash
curl http://localhost:8000
```

### Configuration

* **Port**: Change `--port` in the `uvicorn` command.
* **HTTPS**: Add `--ssl-keyfile` and `--ssl-certfile` to the command.

---

## 🦀 Rust Server (`http-rs`)

### Prerequisites

* **Rust**: Install via [rustup](https://rustup.rs)
* **Dependencies**: `actix-web`, `openssl` (via `Cargo.toml`)

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y libssl-dev
```

#### macOS

```bash
brew install openssl
```

### Build and Run

```bash
cd http-privacy/http-rs
cargo build --release
cargo run --release
```

### Test

```bash
curl http://localhost:8080
```

### Configuration

* **Port**: Edit `src/main.rs`.
* **HTTPS**: Configure SSL with certs in `src/main.rs`.

---

## ⚡ JavaScript Server (`http-js`)

### Prerequisites

* **Node.js**: Version 16+
* **Dependencies**: `express`

```bash
cd http-privacy/http-js
npm install
```

### Run

```bash
cd http-privacy/http-js
node server.js
```

### Test

```bash
curl http://localhost:3000
```

### Configuration

* **Port**: Edit `server.js`.
* **HTTPS**: Use Node's `https` module with certs in `server.js`.

## NPM, PyPi and Crates run

[![crates.io](https://img.shields.io/crates/v/privacy_http_sdk)](https://crates.io/crates/privacy_http_sdk)
[![NPM version](https://img.shields.io/npm/v/http-privacy-js.svg)](https://www.npmjs.com/package/http-privacy-js)
[![PyPI](https://img.shields.io/pypi/v/http-privacy-sdk.svg)](https://pypi.org/project/http-privacy-sdk)