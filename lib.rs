use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Core HttpClient providing privacy-focused HTTP operations.
pub struct HttpClient {
    inner: Client,
}

impl HttpClient {
    /// Initializes a new client with mandatory TLS enforcement.
    pub fn new() -> Self {
        Self {
            inner: Client::builder()
                .https_only(true) // TLS Enforcement per spec v1.1
                .build()
                .expect("Failed to initialize privacy HTTP client"),
        }
    }

    /// Applies privacy rules by filtering sensitive or identifying headers.
    fn apply_privacy_rules(&self, headers: HashMap<String, String>) -> HashMap<String, String> {
        // Blacklist common tracking or identifying headers
        let blacklist = ["Cookie", "Referer", "X-Forwarded-For", "CF-Connecting-IP"];
        headers.into_iter()
            .filter(|(k, _)| !blacklist.iter().any(|&b| b.eq_ignore_ascii_case(k)))
            .collect()
    }

    pub fn get(&self, url: &str, headers: HashMap<String, String>) -> Result<String, String> {
        let filtered_headers = self.apply_privacy_rules(headers);
        let mut request = self.inner.get(url);
        for (k, v) in filtered_headers {
            request = request.header(k, v);
        }
        request.send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())
    }

    pub fn post(&self, url: &str, headers: HashMap<String, String>, body: &str) -> Result<String, String> {
        let filtered_headers = self.apply_privacy_rules(headers);
        let mut request = self.inner.post(url);
        for (k, v) in filtered_headers {
            request = request.header(k, v);
        }
        request.body(body.to_string())
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())
    }

    /// Generates an image via the local Stable Diffusion server and saves it to disk.
    pub fn generate_image(&self, prompt: &str, width: i32, height: i32, steps: i32, output_path: &str) -> Result<(), String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let body = format!(
            r#"{{"prompt": "{}", "width": {}, "height": {}, "steps": {}}}"#,
            prompt, width, height, steps
        );
        
        let response = self.post("http://127.0.0.1:8080/txt2img", headers, &body)?;
        
        // Parse JSON response and decode image (requires serde_json and base64)
        let v: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
        let image_base64 = v["image"].as_str().ok_or("Missing image data")?;
        
        let bytes = base64::decode(image_base64).map_err(|e| e.to_string())?;
        let mut file = File::create(output_path).map_err(|e| e.to_string())?;
        file.write_all(&bytes).map_err(|e| e.to_string())?;
        
        Ok(())
    }
}

// --- C++ Bindings (CXX) ---
#[cxx::bridge(namespace = "privacy_http_sdk")]
mod ffi {
    struct Header {
        key: String,
        value: String,
    }

    extern "Rust" {
        type HttpClient;
        fn new_http_client() -> Box<HttpClient>;
        #[rust_name = "get_ffi"]
        fn get(self: &HttpClient, url: &str, headers: Vec<Header>) -> String;
        #[rust_name = "post_ffi"]
        fn post(self: &HttpClient, url: &str, headers: Vec<Header>, body: &str) -> String;
        #[rust_name = "generate_image_ffi"]
        fn generate_image(self: &HttpClient, prompt: &str, width: i32, height: i32, steps: i32, output_path: &str) -> String;
    }
}

fn new_http_client() -> Box<HttpClient> {
    Box::new(HttpClient::new())
}

impl HttpClient {
    fn get_ffi(&self, url: &str, headers: Vec<ffi::Header>) -> String {
        let map: HashMap<String, String> = headers.into_iter().map(|h| (h.key, h.value)).collect();
        self.get(url, map).unwrap_or_else(|e| format!("Error: {}", e))
    }

    fn post_ffi(&self, url: &str, headers: Vec<ffi::Header>, body: &str) -> String {
        let map: HashMap<String, String> = headers.into_iter().map(|h| (h.key, h.value)).collect();
        self.post(url, map, body).unwrap_or_else(|e| format!("Error: {}", e))
    }

    fn generate_image_ffi(&self, prompt: &str, width: i32, height: i32, steps: i32, output_path: &str) -> String {
        self.generate_image(prompt, width, height, steps, output_path)
            .map(|_| "Success".to_string()).unwrap_or_else(|e| format!("Error: {}", e))
    }
}

// --- Python Bindings (PyO3) ---
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass]
pub struct HttpClientPy {
    client: HttpClient,
}

#[cfg(feature = "python")]
#[pymethods]
impl HttpClientPy {
    #[new]
    fn new() -> Self {
        HttpClientPy { client: HttpClient::new() }
    }

    fn get(&self, url: &str, headers: HashMap<String, String>) -> PyResult<String> {
        self.client.get(url, headers).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn post(&self, url: &str, headers: HashMap<String, String>, body: String) -> PyResult<String> {
        self.client.post(url, headers, &body).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn generate_image(&self, prompt: &str, width: i32, height: i32, steps: i32, output_path: &str) -> PyResult<()> {
        self.client.generate_image(prompt, width, height, steps, output_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }
}