use reqwest::Client;
use tokio::runtime::Runtime;
use core::result::Result;
use gemini_client_rs::GeminiClient;
use wasm_bindgen::prelude::*;
use pyo3::prelude::*;
use aws_sdk_s3::Client as S3Client;
use aws_config::SdkConfig;

/// Configuration struct for AWS-related settings
#[derive(Clone)]
struct AwsConfig {
    s3_client: S3Client,
    sdk_config: SdkConfig,
    bucket_name: String,
    bucket_region: String,
}
/// Main HTTP client struct
pub struct HttpClient {
    client: Client,
    runtime: Runtime,
    api_key: String,
    openai_url: String,
    gpt4: bool,
    headers: Vec<(&'static str, &'static str)>,
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
    gemini_client: GeminiClient,
    deepseek_client: Client,
    deepseek_api_key: String,
    aws_config: AwsConfig,
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[pyclass]
pub struct HttpClientPy {
    #[pyo3(get, set)]
    api_key: String,
    #[pyo3(get, set)]
    openai_url: String,
}

#[pymethods]
impl HttpClientPy {
    #[new]
    fn new(api_key: String, openai_url: String) -> Self {
        Self {
            api_key,
            openai_url,
        }
    }
}

impl HttpClient {
    /// Creates a new instance of `HttpClient`.
    pub fn new(api_key: String) -> Self {
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let aws_config = runtime.block_on(Self::load_aws_config());

        Self {
            client: Client::new(),
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
            gemini_client: GeminiClient::new(api_key),
            aws_config,
        }
    }

    /// Loads AWS configuration asynchronously
    async fn load_aws_config() -> AwsConfig {
        let sdk_config = aws_config::from_env().load().await;

        AwsConfig {
            s3_client: S3Client::new(&sdk_config),
            sdk_config,
            bucket_name: String::new(),
            bucket_region: String::new(),
        }
    }    }
    /// Asynchronously sends a GET request
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<String, String> {
        let mut req = self.client.get(url);
        for &(key, value) in headers {
            req = req.header(key, value);
        }

        let response = req
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        response
            .text()
            .await
            .map_err(|e| format!("Failed to parse response body: {}", e))
    }    /// Asynchronously sends a POST request
    async fn post(&self, url: &str, headers: &[(&str, &str)], body: String) -> Result<String, String> {
        let mut req = self.client.post(url).body(body);
        for &(key, value) in headers {
            req = req.header(key, value);
        }

        let response = req
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        response
            .text()
            .await
            .map_err(|e| format!("Failed to parse response body: {}", e))
    }

    /// Synchronous wrapper for GET requests
    pub fn get_sync(&self, url: &str, headers: &[(&str, &str)]) -> Result<String, String> {
        self.runtime.block_on(self.get(url, headers))
    }

    /// Synchronous wrapper for POST requests
    pub fn post_sync(&self, url: &str, headers: &[(&str, &str)], body: String) -> Result<String, String> {
        self.runtime.block_on(self.post(url, headers, body))
    }