# 🖥️ Command Guide

Detailed documentation for all available `avdoc` commands.

---

## **`avdoc ls [path]`**
List files in the current or specified directory using a clean, professional table format.

**Arguments:**
- `path`: (Optional) The directory you want to explorer. Defaults to the current directory (`.`).

---

## **`avdoc init`**
Initialize a new `avdoc` project. This command creates a `.avdoc` directory in the project root to store project-specific settings and history.

---

## **`avdoc run <prompt>`**
The primary command for AI-assisted coding. It sends your prompt to the configured AI model and automatically parses any generated code files.

**Arguments:**
- `prompt`: The description of what you want to achieve.

**Options:**
- `--provider <name>`: Temporarily override the configured provider.
- `--model <name>`: Temporarily override the configured model.
- `--dry-run`: View the prompt and configuration without calling the AI API.

---

## **`avdoc config <action>`**
Manages both global and project-specific configuration for providers, models, and API keys.

**Subcommands:**
- `config set <provider> <model> <api_key>`: Set up a new provider with its corresponding model and key.
- `config get <key>`: Retrieve the value of a specific setting.
- `config list`: List all currently saved configurations.

---

## **`avdoc provider <action>`**
Granular management of AI providers and their specific settings.

**Subcommands:**
- `provider list`: List all AI providers currently supported by `avdoc`.
- `provider set-key <provider> <key>`: Update just the API key for a specific provider.
- `provider use <provider> <model>`: Set the default provider and model for all future `run` commands.
- `provider show`: Display the currently active provider and model settings.
- `provider list-models <provider>`: Fetches and displays available models from the provider (if supported).

---

## **`avdoc models [provider]`**
Lists all models available across all providers or for a specific specified provider.

**Options:**
- `-p, --provider <name>`: Filter by a specific provider.

---

Next: [Provider Setup](./providers.md)
