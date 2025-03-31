use reqwest::Client;
use tokio::runtime::Runtime;
use core::result::Result;
use gemini_client_rs::GeminiClient;
use wasm_bindgen::prelude::*;
use pyo3::prelude::*;
use aws_sdk_s3::Client as S3Client;
use aws_config::{meta::region::RegionProviderChain, Region};

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
    s3_client: S3Client,
    region_provider: RegionProviderChain,
    config: aws_config::SdkConfig,
    bucket_name: String,
    bucket_region: String,
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
    pub fn new_http_client(api_key: String) -> Box<Self> {
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");

        // Block on the async config loading with proper error handling
        let config = runtime.block_on(async {
            let config_result = aws_config::from_env().load().await;
            match config_result {
                Err(e) => {
                    eprintln!("Failed to load AWS config: {:?}", e);
                    // Fallback to default config with a default region
                    aws_config::SdkConfig::builder()
                        .region(Region::new("us-east-1")) // Set a default region
                        .build()
                }
                Ok(config) => config
            }
        });

        let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
            .or_default_provider();

        Box::new(Self {
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
            s3_client: S3Client::new(&config),
            config,
            region_provider,
            bucket_name: String::new(),
            bucket_region: String::new(),
        })
    }

    /// Asynchronously sends a GET request to the specified URL with headers.
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<String, String> {
        let mut req = self.client.get(url);
        for &(key, value) in headers {
            req = req.header(key, value);
        }

        let response = req.send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        let text = response.text()
            .await
            .map_err(|e| format!("Failed to parse response body: {}", e))?;
        Ok(text)
    }
}
    /// Asynchronously sends a POST request to the specified URL with headers and a body.
    async fn post(&self, url: &str, headers: &[(&str, &str)], body: String) -> Result<String, String> {
        let mut req = self.client.post(url).body(body);
        for &(key, value) in headers {
            req = req.header(key, value);
        }

        let response = req.send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        let text = response.text()
            .await
            .map_err(|e| format!("Failed to parse response body: {}", e))?;
        Ok(text)
    }

    /// Synchronous wrapper for the `get` method for FFI.
    pub fn get_sync(&self, url: &str, headers: &[(&str, &str)]) -> Result<String, String> {
        self.runtime.block_on(self.get(url, headers))
    }

    /// Synchronous wrapper for the `post` method for FFI.
    pub fn post_sync(&self, url: &str, headers: &[(&str, &str)], body: String) -> Result<String, String> {
        self.runtime.block_on(self.post(url, headers, body))
    }
