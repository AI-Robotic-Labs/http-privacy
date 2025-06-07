use pyo3::prelude::*;
use reqwest::Client;
use tokio::runtime::Runtime;
use core::result::Result;
use wasm_bindgen::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Python, PyResult as PyResultType, Bound};
use serde_json::json;
use base64::Engine;


pub fn hello() -> &'static str {
    "Hello from privacy_http_sdk"
}
// HttpClient for Rust/WASM usage
#[wasm_bindgen]
#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    runtime: Runtime,
    api_key: String,
    #[allow(dead_code)]
    openai_url: String,
    #[allow(dead_code)]
    gpt4: bool,
    #[allow(dead_code)]
    headers: Vec<(String, String)>,
    #[allow(dead_code)]
    prompt_tokens: usize,
    #[allow(dead_code)]
    completion_tokens: usize,
    #[allow(dead_code)]
    total_tokens: usize,
    #[allow(dead_code)]
    gemini_client: String,
    #[allow(dead_code)]
    deepseek_client: Client,
    #[allow(dead_code)]
    qwen_client: Client,
    #[allow(dead_code)]
    deepseek_api_key: String,
    #[allow(dead_code)]
    s3_client: Client,
    #[allow(dead_code)]
    xai_api_key: String,
    #[allow(dead_code)]
    qwen_api_key: String,
    #[allow(dead_code)]
    claude_api_key: String,
    #[allow(dead_code)]
    ollama_api_key: String,
    #[allow(dead_code)]
    a2_a_sever: String,
    #[allow(dead_code)]
    request: String,
    #[allow(dead_code)]
    response: String,
    #[allow(dead_code)]
    mcp_server: String,
    #[allow(dead_code)]
    base_url_:String
}
#[wasm_bindgen]
impl HttpClient {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .https_only(true) // Enforce HTTPS for privacy
            .build()
            .expect("Failed to create reqwest client");
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let deepseek_client = Client::new();
        let qwen_client = Client::new();

        Self {
            client,
            runtime,
            api_key: api_key.clone(),
            openai_url: String::new(),
            gpt4: false,
            headers: Vec::new(),
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
            gemini_client: api_key.clone(),
            deepseek_client,
            qwen_api_key: api_key.clone(),
            qwen_client,
            a2_a_sever: String::new(),
            deepseek_api_key: api_key.clone(),
            s3_client: Client::new(),
            xai_api_key: api_key.clone(),
            claude_api_key: api_key.clone(),
            ollama_api_key: api_key,
            request: String::new(),
            response: String::new(),
            mcp_server: String::new(),
            base_url_: String::new()
        }
    }
    pub fn get_sync(&self, url: &str, headers: JsValue) -> Result<String, JsValue> {
        let headers_vec = Self::js_headers_to_vec(headers)?;
        self.runtime
            .block_on(self.get(url, &headers_vec))
            .map_err(|e| JsValue::from_str(&e))
    }

    pub fn post_sync(&self, url: &str, headers: JsValue, body: String) -> Result<String, JsValue> {
        let headers_vec = Self::js_headers_to_vec(headers)?;
        self.runtime
            .block_on(self.post(url, &headers_vec, body))
            .map_err(|e| JsValue::from_str(&e))
    }

    // General prompt for AI API
    pub fn prompt(&self, prompt: &str) -> Result<String, JsValue> {
        let json_data = json!({
            "prompt": prompt,
        });
        Ok(json_data.to_string())
    }

    /// Generate an image using Stable Diffusion API (for WASM)
    pub fn generate_image_sync(&self, prompt: &str, width: u32, height: u32, steps: u32) -> Result<String, JsValue> {
        let url = "https://api.stability.ai/v1/generation/stable-diffusion-xl-beta-v2-2-2/text-to-image";
        let headers = js_sys::Array::new();
        let auth_header = js_sys::Array::new();
        auth_header.push(&JsValue::from_str("Authorization"));
        auth_header.push(&JsValue::from_str(&format!("Bearer {}", self.api_key)));
        headers.push(&auth_header.into());
        let content_type = js_sys::Array::new();
        content_type.push(&JsValue::from_str("Content-Type"));
        content_type.push(&JsValue::from_str("application/json"));
        headers.push(&content_type.into());

        let body = json!({
            "prompt": prompt,
            "width": width,
            "height": height,
            "steps": steps
        })
        .to_string();

        self.post_sync(url, headers.into(), body)
            .map(|response| {
                let json: serde_json::Value = serde_json::from_str(&response).unwrap_or_default();
                json["artifacts"][0]["binary"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            })
    }
}

impl HttpClient {
    async fn get(&self, url: &str, headers: &[(String, String)]) -> Result<String, String> {
        let mut req = self.client.get(url);
        for (key, value) in headers {
            req = req.header(key.as_str(), value.as_str());
        }
        let response = req
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to parse response body: {}", e))?;
        Ok(text)
    }

    async fn post(&self, url: &str, headers: &[(String, String)], body: String) -> Result<String, String> {
        let mut req = self.client.post(url).body(body);
        for (key, value) in headers {
            req = req.header(key.as_str(), value.as_str());
        }
        let response = req
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to parse response body: {}", e))?;
        Ok(text)
    }

    fn js_headers_to_vec(headers: JsValue) -> Result<Vec<(String, String)>, JsValue> {
        if headers.is_undefined() || headers.is_null() {
            return Ok(Vec::new());
        }
        let array = js_sys::Array::from(&headers);
        let mut result = Vec::new();
        for pair in array.iter() {
            let pair_array = js_sys::Array::from(&pair);
            if pair_array.length() >= 2 {
                let key = pair_array.get(0).as_string().unwrap_or_default();
                let value = pair_array.get(1).as_string().unwrap_or_default();
                result.push((key, value));
            }
        }
        Ok(result)
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// HttpClientPy for Python usage via PyO3
#[pyclass]
pub struct HttpClientPy {
    #[pyo3(get, set)]
    api_key: String,
    #[pyo3(get, set)]
    openai_url: String,
    inner: HttpClient,
}

fn headers_to_jsvalue(headers: Vec<(String, String)>) -> JsValue {
    let array = js_sys::Array::new();
    for (key, value) in headers {
        let pair = js_sys::Array::new();
        pair.push(&JsValue::from_str(&key));
        pair.push(&JsValue::from_str(&value));
        array.push(&pair.into());
    }
    array.into()
}

#[pymethods]
impl HttpClientPy {
    #[new]
    fn new(api_key: String, openai_url: String) -> Self {
        let inner = HttpClient::new(api_key.clone());
        Self {
            api_key,
            openai_url,
            inner,
        }
    }

    fn get(&self, url: String, headers: Vec<(String, String)>) -> PyResult<String> {
        self.inner
            .get_sync(&url, headers_to_jsvalue(headers))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(
                e.as_string().unwrap_or("Unknown error".to_string())
            ))
    }

    fn post(&self, url: String, headers: Vec<(String, String)>, body: String) -> PyResult<String> {
        self.inner
            .post_sync(&url, headers_to_jsvalue(headers), body)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(
                e.as_string().unwrap_or("Unknown error".to_string())
            ))
    }

    /// Generate an image using Stable Diffusion API and save it to a file (for Python)
    fn generate_image(&self, prompt: String, width: u32, height: u32, steps: u32, output_path: String) -> PyResult<()> {
        let url = "https://api.stability.ai/v1/generation/stable-diffusion-xl-beta-v2-2-2/text-to-image";
        let headers = vec![
            ("Authorization".to_string(), format!("Bearer {}", self.api_key)),
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Cache-Control".to_string(), "no-cache".to_string()),
        ];
        let body = json!({
            "prompt": prompt,
            "width": width,
            "height": height,
            "steps": steps
        })
        .to_string();

        let response = self.inner
            .post_sync(&url, headers_to_jsvalue(headers), body)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(
                e.as_string().unwrap_or("Unknown error".to_string())
            ))?;

        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to parse response: {}", e)))?;
        let image_data = json["artifacts"][0]["binary"]
            .as_str()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No image data in response"))?;

        // Decode base64 and save to file
        let decoded = base64::engine::general_purpose::STANDARD.decode(image_data)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to decode image: {}", e)))?;
        std::fs::write(&output_path, decoded)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to save image: {}", e)))?;

        Ok(())
    }

    fn __str__(&self) -> String {
        format!(
            "HttpClientPy(api_key='{}', openai_url='{}')",
            self.api_key, self.openai_url
        )
    }
}

#[pymodule]
fn http_client_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResultType<()> {
    m.add_class::<HttpClientPy>()?;
    Ok(())
}

#[cxx::bridge]
mod ffi {
    #[allow(dead_code)]
    unsafe extern "C++" {
        include!("lib.rs.h");

        fn greet(name: &str) -> String;
    }
}
