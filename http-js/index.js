const express = require('express');
const helmet = require('helmet');
const randomUseragent = require('random-useragent');
const fetch = require('node-fetch');
const wasmModule = require('./pkg/your_wasm_module');
const OpenAI = require ('openai');

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware: Set secure HTTP headers using Helmet
app.use(helmet());

// Define route to call the AI API
app.get('/api/ai', async (req, res) => {
    try {
        // Generate a random user-agent
        const userAgent = randomUseragent.getRandom();
        
        // Example AI API request with the custom User-Agent header
        const aiApiUrl = 'https://api.example.com/ai'; // Replace with your AI API URL
        const inputData = { message: "Hello from WASM!" };

        // Optional: Perform preprocessing or data manipulation using WASM
        const processedData = wasmModule.process_message(JSON.stringify(inputData));

        // Make the HTTP request to the AI API
        const response = await fetch(aiApiUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'User-Agent': userAgent,
            },
            body: processedData,
        });

        if (!response.ok) {
            throw new Error(`API responded with status ${response.status}`);
        }

        const apiData = await response.json();
        res.json(apiData);
    } catch (error) {
        console.error('Error calling AI API:', error);
        res.status(500).json({ error: 'Failed to fetch data from AI API' });
    }
});

// Start server
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});