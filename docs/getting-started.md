# Getting Started with avdoc 🚀

Welcome to **avdoc**, a high-performance CLI for "vibecoders" designed to streamline your AI-assisted development workflow.

## 💾 Installation

### **Direct Installation (Rust developers)**
The fastest way to install is via Cargo:
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

## 🏗️ First Time Setup

After installation, follow these three steps to get up and running:

### **1. Initialize your project**
Run this in your project root to create the `.avdoc` directory:
```bash
avdoc init
```

### **2. Configure your AI provider**
You need to set up a provider (like Gemini or DeepSeek) to use the AI features.
```bash
# Example for Gemini
avdoc config set gemini gemini-1.5-flash YOUR_API_KEY
```

### **3. Set your default provider**
Tell avdoc which provider and model to use by default:
```bash
avdoc provider use gemini gemini-1.5-flash
```

## 🚦 Basic Usage

Try listing your files with professional formatting:
```bash
avdoc ls
```

Now try generating some code:
```bash
avdoc run "create a simple hello world in rust"
```

Next, check out the [Commands Guide](./commands.md) for a full list of features.
