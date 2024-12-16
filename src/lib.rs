use reqwest::Client;
use tokio::runtime::Runtime;
use core::result::Result;

pub struct HttpClient {
    client: Client,
    runtime: Runtime,
    #[allow(dead_code)]
    api_key: String,
    #[allow(dead_code)]
    openai_url: String,
    #[allow(dead_code)]
    gpt4: bool,
    #[allow(dead_code)]
    headers: Vec<(&'static str, &'static str)>,
    #[allow(dead_code)]
    propmt_tokens: usize,
    #[allow(dead_code)]
    completion_tokens: usize,
    #[allow(dead_code)]
    total_tokens: usize,
    
}
impl HttpClient {
    /// Creates a new instance of `HttpClient`.
    pub fn new_http_client() -> Box<Self> {
        let client = Client::new();
        let runtime = Runtime::new().unwrap();
        Box::new(Self {
            client,
            runtime,
            api_key: String::new(),
            openai_url: String::new(),
            gpt4: false,
            headers: Vec::new(),
            propmt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        })
    }

    /// Asynchronously sends a GET request to the specified URL with headers.
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<String, String> {
        let mut req = self.client.get(url);
        for &(key, value) in headers {
            req = req.header(key, value);
        }

        // Send the request and handle errors more explicitly
        let response = req.send().await.map_err(|e| format!("Request failed: {}", e))?;
        let text = response.text().await.map_err(|e| format!("Failed to parse response body: {}", e))?;
        Ok(text)
    }

    /// Asynchronously sends a POST request to the specified URL with headers and a body.
    async fn post(&self, url: &str, headers: &[(&str, &str)], body: String) -> Result<String, String> {
        let mut req = self.client.post(url).body(body);
        for &(key, value) in headers {
            req = req.header(key, value);
        }

        // Send the request and handle errors more explicitly
        let response = req.send().await.map_err(|e| format!("Request failed: {}", e))?;
        let text = response.text().await.map_err(|e| format!("Failed to parse response body: {}", e))?;
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
}