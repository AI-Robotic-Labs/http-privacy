use std::collections::HashMap;
use serde_json::Value;
use reqwest::Client;
use tokio::runtime::Runtime;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type HttpClient;

        fn new_http_client() -> Box<HttpClient>;
        fn get(&self, url: &str, headers: HashMap<String, String>) -> Result<String, String>;
        fn post(&self, url: &str, headers: HashMap<String, String>, body: String) -> Result<String, String>;
    }
}

pub struct HttpClient {
    client: Client,
    runtime: Runtime,
}

impl HttpClient {
    pub fn new_http_client() -> Box<Self> {
        let client = Client::new();
        let runtime = Runtime::new().unwrap();
        Box::new(Self { client, runtime })
    }

    pub fn get(&self, url: &str, headers: HashMap<String, String>) -> Result<String, String> {
        let mut req = self.client.get(url);
        for (key, value) in headers {
            req = req.header(&key, &value);
        }

        self.runtime.block_on(async {
            req.send()
                .await
                .and_then(|res| res.text().await)
                .map_err(|e| e.to_string())
        })
    }

    pub fn post(&self, url: &str, headers: HashMap<String, String>, body: String) -> Result<String, String> {
        let mut req = self.client.post(url).body(body);
        for (key, value) in headers {
            req = req.header(&key, &value);
        }

        self.runtime.block_on(async {
            req.send()
                .await
                .and_then(|res| res.text().await)
                .map_err(|e| e.to_string())
        })
    }
}
