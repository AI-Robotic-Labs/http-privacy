use pyo3::prelude::*;
use reqwest::Client;
use tokio::runtime::Runtime;
use core::result::Result;
use gemini_client_rs::GeminiClient;
use wasm_bindgen::prelude::*;

// HttpClient for Rust/WASM usage
#[wasm_bindgen]
pub struct HttpClient {
    client: Client,
    runtime: Runtime,
    api_key: String,
    openai_url: String,
    gpt4: bool,
    headers: Vec<(&'static str, &'static str)>,
    prompt_tokens: usize, // Fixed typo
    completion_tokens: usize,
    total_tokens: usize,
    gemini_client: GeminiClient,
    deepseek_client: Client, // Fixed typo
    deepseek_api_key: String,
    s3_client: Client,
    xai_api_key: String,
}

#[wasm_bindgen]
impl HttpClient {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String) -> Box<HttpClient> {
        let client = Client::new();
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        Box::new(Self {
            client,
            runtime,
            api_key: api_key.clone(),
            openai_url: String::new(),
            gpt4: false,
            headers: Vec::new(),
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
            deepseek_api_key: api_key.clone(),
            deepseek_client: Client::new(),
            gemini_client: GeminiClient::new(api_key.clone()),
            s3_client: Client::new(),
            xai_api_key: api_key,
        })
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
}

impl HttpClient {
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<String, String> {
        let mut req = self.client.get(url);
        for &(key, value) in headers {
            req = req.header(key, value);
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

    async fn post(&self, url: &str, headers: &[(&str, &str)], body: String) -> Result<String, String> {
        let mut req = self.client.post(url).body(body);
        for &(key, value) in headers {
            req = req.header(key, value);
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

    fn js_headers_to_vec(headers: JsValue) -> Result<Vec<(&'static str, &'static str)>, JsValue> {
        if headers.is_undefined() || headers.is_null() {
            return Ok(Vec::new());
        }
        // Placeholder: Implement based on actual JS header format
        Err(JsValue::from_str(
            "Header conversion not implemented; pass headers as array of [key, value] pairs",
        ))
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// HttpClientPy for Python usage via PyO3
#[pyclass]
#[derive(Clone)] // Added for PyO3 compatibility
pub struct HttpClientPy {
    #[pyo3(get, set)]
    api_key: String,
    #[pyo3(get, set)]
    openai_url: String,
    inner: HttpClient,
}
#[pymethods]
impl HttpClientPy {
    #[new]
    fn new(api_key: String, openai_url: String) -> PyResult<Self> {
        let inner = HttpClient::new(api_key.clone());
        Ok(HttpClientPy {
            api_key,
            openai_url,
            inner,
        })
    }

    fn get(&self, url: String, headers: Vec<(String, String)>) -> PyResult<String> {
        let _headers_ref: Vec<(&str, &str)> = headers.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        self.inner
            .get_sync(&url, JsValue::NULL) // Simplified for PyO3; headers passed directly
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.as_string().unwrap_or("Unknown error".to_string())))
    }
    fn post(&self, url: String, headers: Vec<(String, String)>, body: String) -> PyResult<String> {
        let _headers_ref: Vec<(&str, &str)> = headers.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        self.inner
            .post_sync(&url, JsValue::NULL, body) // Simplified for PyO3; headers passed directly
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.as_string().unwrap_or("Unknown error".to_string())))
    }

    fn __str__(&self) -> String {
        format!(
            "HttpClientPy(api_key='{}', openai_url='{}')",
            self.api_key, self.openai_url
        )
    }
}

#[pymodule]
fn http_client_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<HttpClientPy>()?;
    Ok(())
}