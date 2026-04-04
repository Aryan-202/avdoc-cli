# avdoc - AI-Powered Development Assistant

## What is avdoc?

avdoc is a CLI tool that brings AI assistance directly to your terminal. Think of it as having a coding assistant that understands your project structure, can write code, fix bugs, explain complex logic, and help you build features - all from the command line.

## Quick Installation

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.ps1 | iex
```

**Mac/Linux:**
```bash
curl -sSf https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.sh | sh
```

**Via Cargo (Rust developers):**
```bash
cargo install avdoc
```

## First Time Setup

1. Initialize avdoc in your project:
```bash
avdoc init
```

2. Set your API key (OpenAI, Anthropic, or any compatible provider):
```bash
avdoc config set openai sk-your-key-here
```

3. You're ready to go!

## Core Commands

### Run AI Tasks
The main command - just describe what you want:
```bash
avdoc run "create a REST API endpoint for user authentication"
avdoc run "add error handling to src/main.rs"
avdoc run "explain how the sorting algorithm works"
```

### Plan Before Executing
See what avdoc will do before it does it:
```bash
avdoc plan "refactor the database connection code"
```
This shows you a step-by-step plan without making changes.

### Apply a Plan
Execute a previously generated plan:
```bash
avdoc apply plan.json
```

### Manage Project Context
Tell avdoc which files to look at:
```bash
avdoc context add src/           # Add entire directory
avdoc context add Cargo.toml     # Add specific file
avdoc context list               # See current context
avdoc context clear              # Reset context
```

### Configuration
```bash
avdoc config set openai <key>    # Set OpenAI API key
avdoc config set anthropic <key> # Set Anthropic API key  
avdoc config use openai          # Choose default provider
avdoc config get model           # Check current model
avdoc config list                # Show all settings
```

### History & Debugging
```bash
avdoc history                    # See your command history
avdoc logs                       # View debug logs
avdoc doctor                     # Check if everything works
```

## Real-World Examples

**Build a new feature:**
```bash
avdoc run "add a --verbose flag to show debug output"
```

**Fix a bug:**
```bash
avdoc run "fix the off-by-one error in the pagination logic"
```

**Understand code:**
```bash
avdoc run "explain what the authentication middleware does"
```

**Generate documentation:**
```bash
avdoc run "write doc comments for all public functions"
```

## Advanced Usage

### Interactive Mode
Start a conversation with the AI:
```bash
avdoc chat
```

### Auto-apply Changes
Skip confirmation prompts:
```bash
avdoc run "add unit tests" --yes
```

### Dry Run
Preview changes without applying:
```bash
avdoc run "refactor the config parser" --dry-run
```

### Specify Model
```bash
avdoc run "complex refactoring" --model gpt-4
avdoc run "simple fix" --model gpt-3.5-turbo
```

## CI/CD Integration

Use avdoc in your pipelines to enforce documentation standards:

```yaml
# GitHub Actions example
- name: Check documentation quality
  run: avdoc lint --min-score 80
  env:
    OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
```

## Supported Languages

avdoc works with:
- Rust
- Python  
- JavaScript/TypeScript
- Go
- Java
- C/C++
- And more coming soon

## Project Structure

When you run `avdoc init`, it creates:
- `.avdoc/config.toml` - Your project settings
- `.avdoc/history.json` - Command history
- `.avdoc/context.json` - Files avdoc can access

## Environment Variables

```bash
export OPENAI_API_KEY=your-key    # OpenAI (default)
export ANTHROPIC_API_KEY=your-key # Anthropic
export AVDOC_MODEL=gpt-4          # Override default model
export AVDOC_PROVIDER=openai      # Set default provider
```

## Uninstalling

**If installed via cargo:**
```bash
cargo uninstall avdoc
```

**If installed via script (Mac/Linux):**
```bash
rm /usr/local/bin/avdoc  # or ~/.local/bin/avdoc
rm -rf ~/.avdoc           # Remove config files
```

**If installed via script (Windows):**
```powershell
rm $env:LOCALAPPDATA\Programs\avdoc\avdoc.exe
rm -r $env:APPDATA\avdoc
```

## Troubleshooting

**"avdoc: command not found"**
- Restart your terminal after installation
- Check if the install directory is in your PATH

**"No API key found"**
- Run `avdoc config set openai your-key-here`
- Or set the OPENAI_API_KEY environment variable

**"Project not initialized"**
- Run `avdoc init` in your project root

## Contributing

Found a bug or want a feature? Open an issue on GitHub. Pull requests welcome!

## License

MIT License - feel free to use this in personal or commercial projects.

## Links

- GitHub: https://github.com/Aryan-202/avdoc
- Issues: https://github.com/Aryan-202/avdoc/issues
- Sponsorship: https://github.com/sponsors/Aryan-202

## Why avdoc?

Most AI coding tools are either:
- Web-based (copy-paste back and forth)
- IDE plugins (lock you into one editor)
- Overly complex (dozens of commands to learn)

avdoc is different - it lives in your terminal, works with any editor, and focuses on one thing: turning your natural language descriptions into actual code changes.

No context switching. No copy-pasting. Just describe and go.