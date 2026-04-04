# ⚙️ Configuration

`avdoc` manages its configuration in two ways: globally and project-specifically.

## 💾 Storage Locations

### **Global Configuration**
Stored in your computer's home directory. This includes your default settings and API keys across all projects.

**Windows:** `%USERPROFILE%\.avdoc\global.json`
**Linux/macOS:** `~/.avdoc/global.json`

### **Project Configuration**
Stored in a `.avdoc/` directory within your project's root. This is created when you run `avdoc init`.

**Path:** `./.avdoc/`

---

## 🛠️ Managing Configuration

Use the `avdoc config` command to interact with your settings.

### **Common Commands**

| Command | Action |
|---------|--------|
| `config set <p> <m> <k>` | Set up a provider with a specific model and API key. |
| `config get <key>` | Get the current value of a configuration field. |
| `config list` | List all available configuration settings. |

### **Example: Check Current Model**
```bash
avdoc config get model
```

### **Example: Set a Global API Key**
```bash
avdoc config set openai gpt-4o sk-youropenaikey
```

---

## 🔒 Security

`avdoc` does not transmit your API keys anywhere except directly to the configured provider API endpoints. Your keys are stored locally on your machine.

> [!CAUTION]
> Never share your API keys or commit the `.avdoc/` directory (if it contains sensitive data) to your repository. **Add `.avdoc/` to your `.gitignore`**.

---

Back to [Main Documentation](../README.md)
