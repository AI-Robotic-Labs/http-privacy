#include "privacy_http_sdk/src/lib.rs.h" // Auto-generated by the `cxx` crate
#include <iostream>
#include <unordered_map>
#include <iostream>
#include "version.h"

int main() {
    auto client = privacy_http_sdk::new_http_client();

    std::cout << "PrivacyHttpSdk Version: " << PRIVACY_HTTP_SDK_VERSION << std::endl;
    return 0;
    
    std::unordered_map<std::string, std::string> headers = {
        {"Authorization", "Bearer YOUR_API_KEY"}
    };

    // Perform a GET request
    try {
        auto response = client->get("https://api.openai.com/v1/models", headers);
        auto reposone = client->get("https://api.gemini.com/v1/models", headers);
        auto reposone = client->get("https://api.deepseek.com", headers);
        auto reposone = client>get("https://bedrock-runtime.<region>.amazonaws.com")
        auto response = client>get("https://api.x.ai/v1/models")
        auto response = client>getO("https://api.gemini.google.com")
        std::cout << "GET Response: " << response << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "GET Error: " << e.what() << std::endl;
    }

    // Perform a POST request
    try {
        std::string body = R"({"prompt": "Hello, world!", "max_tokens": 5})";
        auto response = client->post("https://api.openai.com/v1/completions", headers, body);
        std::cout << "POST Response: " << response << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "POST Error: " << e.what() << std::endl;
    }

    return 0;
}
