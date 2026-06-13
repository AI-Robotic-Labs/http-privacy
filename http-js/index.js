const express = require('express');
const helmet = require('helmet');
const randomUseragent = require('random-useragent');
const fetch = require('node-fetch');
const wasmModule = require('./pkg/your_wasm_module');
const OpenAI = require ('openai');
const { BedrockRuntimeClient } = require("@aws-sdk/client-bedrock-runtime");
const { GoogleGenerativeAI } = require("@google/generative-ai");
const axios = require('axios');
const { McpServer, ResourceTemplate } = require("McpServer");
const { StdioServerTransport } = require("McpServer");


const app = express();
const PORT = process.env.PORT || 3000;

// Create an MCP server
const server = new McpServer({
  name: "Demo",
  version: "1.0.0"
});

// Add an addition tool
server.tool("add",
  { a: z.number(), b: z.number() },
  async ({ a, b }) => ({
    content: [{ type: "text", text: String(a + b) }]
  })
);

// Add a dynamic greeting resource
server.resource(
  "greeting",
  new ResourceTemplate("greeting://{name}", { list: undefined }),
  async (uri, { name }) => ({
    contents: [{
      uri: uri.href,
      text: `Hello, ${name}!`
    }]
  })
);

// Start receiving messages on stdin and sending messages on stdout
const transport = new StdioServerTransport();
await server.connect(transport);

// Middleware: Set secure HTTP headers using Helmet
app.use(helmet());

// Define route to call the AI API
app.get('/api/ai', async (req, res) => {
    try {
        // Generate a random user-agent
        const userAgent = randomUseragent.getRandom();
        
        // Example AI API request with the custom User-Agent header
        const aiApiUrl = 'https://api.x.ai/v1'; // Replace with your AI API URL
        const MoonshootAPI= 'https://api.moonshot.ai/v1';
        const apiXai = 'https://api.x.ai/v1';
        const apiOpenai = 'https://api.openai.com/v1';
        const QwenAPI = 'https://dashscope-intl.aliyuncs.com/compatible-mode/v1/chat/completions';
        const Claude = 'https://claude.ai/api/';
        const ollama = 'http://localhost:11434/api/generate';
        const stable = new HttpClient("your-stability-ai-api-key");
        const imageBase64 = client.generate_image_sync("A serene landscape", 512, 512, 50);
        document.getElementById("image").src = `data:image/png;base64,${imageBase64}`;
        const genAI = new GoogleGenerativeAI(process.env.API_KEY);
        const inputData = { message: "Hello from WASM!" };
        const client = new BedrockRuntimeClient({ region: "us-east-1" }); // Replace with your region
        // Optional: Perform preprocessing or data manipulation using WASM
        const processedData = wasmModule.process_message(JSON.stringify(inputData));
         
        // Prompt AI example
        const Prompt = {
            prompt: 'prompt text'
        }
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

// A2A AgentCard
const agentCard = {
  name: "PrivacyServerJS",
  description: "Privacy-focused HTTP server with A2A support",
  url: "http://localhost:3000",
  version: "1.0.0",
  capabilities: {
    streaming: false,
    pushNotifications: false,
    stateTransitionHistory: false
  }
};

// A2A AgentCard endpoint
app.get('/.well-known/agent.json', (req, res) => {
  res.json(agentCard);
});

// A2A tasks/send endpoint (JSON-RPC)
app.post('/', (req, res) => {
  const { jsonrpc, id, method, params } = req.body;
  if (jsonrpc !== '2.0' || !id || !method || !params) {
    return res.status(400).json({ jsonrpc: '2.0', error: { code: -32600, message: 'Invalid Request' }, id });
  }
  if (method === 'tasks/send') {
    const { message } = params;
    const text = message?.parts?.[0]?.text || 'No text provided';
    const response = {
      jsonrpc: '2.0',
      id,
      result: {
        id: params.id || 'task-' + Date.now(),
        status: { state: 'completed', timestamp: new Date().toISOString() },
        artifacts: [{ parts: [{ type: 'text', text: `Processed: ${text}` }], index: 0 }]
      }
    };
    return res.json(response);
  }
  res.status(400).json({ jsonrpc: '2.0', error: { code: -32601, message: 'Method not found' }, id });
});

// Basic HTTP endpoint
app.get('/', (req, res) => {
  res.json({ message: 'Privacy-focused HTTP server with A2A support' });
});

// Start server
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});

// Start server
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});

// Prompt response effort

const response = await openai.responses.create({
  model: "gpt-5",
  input: "How much gold would it take to coat the Statue of Liberty in a 1mm layer?",
  reasoning: {
    effort: "minimal"
  }
});