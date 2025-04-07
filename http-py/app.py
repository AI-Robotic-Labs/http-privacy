import os
from flask import Flask, jsonify, request
from user_agent import generate_user_agent
import json
import subprocess
from openai import OpenAI
import boto3
import google.generativeai as genai

app = Flask(__name__)

# Configure Gemini API with the API key from environment variables
genai.configure(api_key=os.getenv("GEMINI_API_KEY"))

# Initialize clients with proper separation
openai_client = OpenAI(api_key="<DeepSeek API Key>", base_url="https://api.deepseek.com")
bedrock_client = boto3.client(
    service_name="bedrock-runtime",
    region_name="<region>"  # Replace with actual region
)
xai_client = OpenAI(
    api_key=os.getenv("XAI_API_KEY", "<XAI API Key>"),  # Use env var or fallback to direct key
    base_url="https://api.x.ai/v1"
)

# Define the Gemini model (no base_url needed, handled by the SDK)
gemini_model = genai.GenerativeModel("gemini-pro")

def main():
    print("Hello, World!")
    return 0

# Example endpoint using Gemini
@app.route('/gemini', methods=['POST'])
def gemini_endpoint():
    try:
        input_data = request.get_json()
        
        if not input_data or 'message' not in input_data:
            return jsonify({"error": "Missing 'message' in request body"}), 400

        # Use the Gemini model to generate a response
        response = gemini_model.generate_content(input_data['message'])
        
        return jsonify({
            "message": response.text,
            "model": "gemini-pro"
        }), 200
        
    except Exception as e:
        print(f"Error calling Gemini API: {e}")
        return jsonify({"error": "Failed to process Gemini request"}), 500

# Your existing endpoints (boto3, handle_message, call_ai_api, xai_endpoint) remain unchanged
# ... [rest of your code] ...

if __name__ == '__main__':
    app.run(debug=True, port=5000)