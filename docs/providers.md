# 🌍 AI Provider Setup

`avdoc` is designed to be provider-agnostic, supporting a wide range of Large Language Models (LLMs). Here's how to set them up.

## 🛠️ Supported Providers

| Provider | Model Example | Description |
|----------|---------------|-------------|
| **Gemini** | `gemini-1.5-flash` | High-performance, large context. |
| **DeepSeek** | `deepseek-chat` | Efficient and reliable. |
| **Groq** | `llama-3.1-70b-versatile` | Ultra-fast inference with Llama models. |
| **OpenAI** | `gpt-4o` | The industry-standard models. |
| **OpenRouter** | `anthropic/claude-3-opus` | Aggregate access to multiple providers. |

---

## 🔑 Setting Up API Keys

For each provider, you must set an API key using the following command:

```bash
avdoc config set <provider> <model> <api_key>
```

### **1. Google Gemini**
1. Get your API Key from [AI Studio](https://aistudio.google.com/app/apikey).
2. Configure `avdoc`:
   ```bash
   avdoc config set gemini gemini-1.5-flash YOUR_GEMINI_KEY
   ```

### **2. DeepSeek**
1. Get your API Key from the [DeepSeek Platform](https://platform.deepseek.com/).
2. Configure `avdoc`:
   ```bash
   avdoc config set deepseek deepseek-chat YOUR_DEEPSEEK_KEY
   ```

### **3. Groq**
1. Get your API Key from the [Groq Console](https://console.groq.com/keys).
2. Configure `avdoc`:
   ```bash
   avdoc config set groq llama-3.1-70b-versatile YOUR_GROQ_KEY
   ```

### **4. OpenAI**
1. Get your API Key from the [OpenAI Platform](https://platform.openai.com/).
2. Configure `avdoc`:
   ```bash
   avdoc config set openai gpt-4o YOUR_OPENAI_KEY
   ```

---

## 🔄 Switching Between Providers

You can switch your default provider at any time:

```bash
avdoc provider use <provider> <model>
```

Alternatively, override the provider for a single command:

```bash
avdoc run "add unit tests to src/main.rs" --provider gemini --model gemini-1.5-pro
```

Next: [Global and Project Configuration](./configuration.md)
