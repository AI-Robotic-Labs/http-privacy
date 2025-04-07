use pyo3::prelude::*;
use reqwest::Client;
use tokio::runtime::Runtime;
use core::result::Result;
use gemini_client_rs::GeminiClient;
use wasm_bindgen::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Python, PyResult as PyResultType, Bound};

// HttpClient for Rust/WASM usage
#[wasm_bindgen]
#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    runtime: Runtime,
    api_key: String,
    openai_url: String,
    gpt4: bool,
    headers: Vec<(String, String)>,
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
    gemini_client: String,
    deepseek_client: Client,
    deepseek_api_key: String,
    s3_client: Client,
    xai_api_key: String,
}

#[wasm_bindgen]
impl HttpClient {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let deepseek_client = Client::new();
        
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
            gemini_client: api_key.clone().to_string(),
            deepseek_client,
            deepseek_api_key: api_key.clone(),
            s3_client: Client::new(),
            xai_api_key: api_key,
        }
    }    pub fn get_sync(&self, url: &str, headers: JsValue) -> Result<String, JsValue> {
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