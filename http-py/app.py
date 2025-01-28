import requests
from flask import Flask, jsonify, request
from user_agent import generate_user_agent
import json
import subprocess

app = Flask(__name__)

# Helper function to call the WASM module
def process_message(input_data):
    try:
        # Assume the WASM module is a CLI binary generated from a Rust project.
        wasm_result = subprocess.run(
            ["wasm-module"], input=input_data.encode(), capture_output=True, text=True
        )
        return wasm_result.stdout.strip()
    except Exception as e:
        print(f"Error running WASM module: {e}")
        return None

# Endpoint to call the AI API
@app.route('/api/ai', methods=['POST'])
def call_ai_api():
    try:
        # Extract input data
        input_data = request.get_json()

        if not input_data or 'message' not in input_data:
            return jsonify({"error": "Missing 'message' in request body"}), 400

        # Preprocess the message using WASM
        processed_message = process_message(json.dumps(input_data))
        if not processed_message:
            return jsonify({"error": "Failed to process data using WASM module"}), 500

        # AI API request
        api_url = "https://api.example.com/ai"  # Replace with the actual AI API endpoint
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

if __name__ == '__main__':
    app.run(debug=True, port=5000)
