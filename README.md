# avdoc 🚀

[![Crates.io](https://img.shields.io/crates/v/avdoc.svg)](https://crates.io/crates/avdoc)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-blue.svg)](https://www.rust-lang.org)

**avdoc** is a high-performance, AI-driven CLI companion designed for the modern developer. It bridges the gap between your terminal and powerful Large Language Models (LLMs), enabling seamless code generation, project exploration, and automated file creation directly within your workspace.

---

## 📚 Documentation

For deeper details on setup and architecture, please refer to our documentation guides:

*   [Getting Started](./docs/getting-started.md) — Detailed installation and first setup.
*   [Command Guide](./docs/commands.md) — Comprehensive explanation of all CLI commands.
*   [AI Provider Setup](./docs/providers.md) — How to configure Gemini, DeepSeek, and more.
*   [Configuration Guide](./docs/configuration.md) — Understanding global and project-level settings.

---

## 🏗️ Core Features

- **Multi-Provider Support**: First-class integration with Gemini, DeepSeek, and more.
- **Automated Code Generation**: Describe what you need, and `avdoc` generates the code and writes it to the correct files.
- **Project Exploration**: Built-in file explorer with professional table formatting.
- **Global Configuration**: Centralized management for API keys, models, and default providers.
- **Dry-Run Capabilities**: Preview AI prompts and model selections before execution.

## 🚀 Quick Installation

### **Standard Installation**
If you have Rust installed, the easiest way is via Cargo:
```bash
cargo install avdoc
```

### **Universal Install Scripts**

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.ps1 | iex
```

**Linux/macOS:**
```bash
curl -sSf https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.sh | sh
```

---

## 🛠️ Getting Started

### **1. Initialize Your Project**
Create a `.avdoc` configuration directory in your project root:
```bash
avdoc init
```

### **2. Configure Your AI Provider**
Set up your preferred provider with an API key and default model:
```bash
# General setup (provider, model, API key)
avdoc config set <provider> <model> <api_key>

# Example: Using DeepSeek
avdoc config set deepseek deepseek-chat YOUR_API_KEY
```

### **3. Manage Providers & Models**
Explore available models and set defaults:
```bash
# List available providers
avdoc provider list

# See models for a specific provider
avdoc models --provider gemini

# Set a default provider and model
avdoc provider use gemini gemini-1.5-flash
```

---

## 💻 Usage

### **The `run` Command**
The heart of `avdoc`. Use natural language to generate code or documentation.

```bash
avdoc run "Create a new Rust module in utils/logger.rs with a colored logging function"
```

> [!TIP]
> Use the `--dry-run` flag to see the prompt without making an API call:
> `avdoc run "Refactor my main function" --dry-run`

### **File Exploration**
View your project structure with a clean, formatted output:
```bash
avdoc ls
```

---

## 🧠 How it Works

`avdoc` uses a specialized system prompt that instructs LLMs to output code in a structured format:

```xml
<avdoc_file name="path/to/file.rs">
// Generated code here...
</avdoc_file>
```

The CLI automatically parses these tags and:
1. Creates parent directories if they don't exist.
2. Writes the generated content to the specified file path.
3. Provides real-time feedback in your terminal.

---

## ⚙️ Configuration

`avdoc` stores global settings in your home directory (`~/.avdoc/global.json`) and project-specific settings in `.avdoc/`.

| Command | Description |
|---------|-------------|
| `config list` | Show all saved configurations |
| `config get <key>` | Retrieve a specific setting |
| `provider show` | Check the currently active provider and model |
| `provider set-key <p> <k>` | Quickly update an API key |

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! If you have ideas for new features or encounter bugs, please open an issue on our [GitHub Repository](https://github.com/Aryan-202/avdoc).

---

Designed with ❤️ for vibecoders.