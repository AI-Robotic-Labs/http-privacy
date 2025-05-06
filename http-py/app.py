import os
from flask import Flask, jsonify, request
from user_agent import generate_user_agent
import json
import subprocess
from openai import OpenAI
from llama_cpp import Llama
import boto3
import google.generativeai as genai
import requests
from transformers import AutoModelForCausalLM, AutoTokenizer
import HttpClientPy
import Prompt
from python_a2a import A2AServer, skill, agent, run_server, TaskStatus, TaskState
import agent

app = Flask(__name__)

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

qwen_client = OpenAI(
    api_key=os.getenv("Qwen_API_KEY"),
    base_url="https://dashscope-intl.aliyuncs.com/compatible-mode/v1/chat/completion"
)

claude_client = OpenAI(
    api_key=os.getenv("Claude_API_KEY"),
    
    base_url="https://api.anthropic.com/v1/complete"
)

gemini_client = OpenAI(
    api_key=os.getenv("Gemini_API_KEY"),

    base_url="https://generativelanguage.googleapis.com"
)

ollama_client = OpenAI(
    api_key=os.getenv("Ollama_API_KEY"),
    base_url="http://localhost:11434"
)

A2AServer = A2AServer(
    api_key=os.getenv('A2A_API_KEY')
)
# Initialize with Stability AI API key
client = HttpClientPy ("your-stability-ai-api-key", "")

# Generate image and save to file
client.generate_image("A serene landscape", 512, 512, 50, "output.png")
print("Image saved to output.png")

# Configure Gemini API with the API key from environment variables
genai.configure(api_key=os.getenv("GEMINI_API_KEY"))
# Define the Gemini model
gemini_model = genai.GenerativeModel("gemini-pro")

# A2A server

def Ai_agent():
    agentjson_data = {
        "metadata": "metadata",
        "version": "version"
    }
    return agentjson_data
# Prompt AI example
Prompt.generate_prompt(
    prompt="hello world"
)
def main():
    print("Hello, World!")
    return 0

@app.route('/boto3', methods=['POST'])
def boto3_endpoint():
    input_data = request.get_json()
    if not input_data or 'message' not in input_data:
        return jsonify({'error': 'Invalid input data'}), 400
    message = input_data['message']
    return jsonify({'message': message}), 200
    
@app.route('/', methods=['POST'])
def handle_message():
    input_data = request.get_json()
    if not input_data or 'message' not in input_data:
        return jsonify({'error': 'Invalid input data'}), 400
    message = input_data['message']
    return jsonify({'message': message}), 200

# Helper function to call the WASM module
def process_message(input_data):
    try:
        wasm_result = subprocess.run(
            ["wasm-module"],
            input=input_data.encode(),
            capture_output=True,
            text=True
        )
        return wasm_result.stdout.strip()
    except Exception as e:
        print(f"Error running WASM module: {e}")
        return None

@app.route('/api/ai', methods=['POST'])
def call_ai_api():
    try:
        input_data = request.get_json()
        
        if not input_data or 'message' not in input_data:
            return jsonify({"error": "Missing 'message' in request body"}), 400

        processed_message = process_message(json.dumps(input_data))
        if not processed_message:
            return jsonify({"error": "Failed to process data using WASM module"}), 500

        api_url = "https://api.example.com/ai"
        headers = {
            "User-Agent": generate_user_agent(),
            "Content-Type": "application/json",
        }
        response = requests.post(api_url, headers=headers, data=processed_message)

        if response.status_code != 200:
            return jsonify({"error": f"API returned status {response.status_code}"}), response.status_code

        return jsonify(response.json())
    except Exception as e:
        print(f"Error: {e}")
        return jsonify({"error": "Internal server error"}), 500

@app.route('/xai', methods=['POST'])
def xai_endpoint():
    try:
        input_data = request.get_json()
        
        if not input_data or 'message' not in input_data:
            return jsonify({"error": "Missing 'message' in request body"}), 400

        # Call xAI's API using the OpenAI client
        completion = xai_client.chat.completions.create(
            model="grok-2-latest",
            messages=[
                {
                    "role": "system",
                    "content": "You are Grok, a chatbot inspired by the Hitchhikers Guide to the Galaxy."
                },
                {
                    "role": "user",
                    "content": input_data['message']
                },
            ],
        )
        
        response = {
            "message": completion.choices[0].message.content,
            "model": "grok-2-latest"
        }
        return jsonify(response), 200
        
    except Exception as e:
        print(f"Error calling xAI API: {e}")
        return jsonify({"error": "Failed to process xAI request"}), 500

if __name__ == '__main__':
    # Run Flask app
    app.run(debug=True, port=5000)
    
    # Example OpenAI API call (note: this won't run as it's after app.run())
    response = openai_client.chat.completions.create(
        model="deepseek-chat",
        messages=[
            {"role": "system", "content": "You are a helpful assistant"},
            {"role": "user", "content": "Hello"},
        ],
        stream=False
    )