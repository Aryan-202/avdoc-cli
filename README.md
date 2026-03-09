# avdoc

[![Crates.io](https://img.shields.io/crates/v/avdoc.svg)](https://crates.io/crates/avdoc)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-pink.svg)](https://github.com/sponsors/Aryan-202)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

AI-powered documentation gatekeeper and architecture visualizer.

## Overview

avdoc provides tools to analyze and improve repository documentation. It can evaluate codebases to generate a documentation quality score, automatically generate missing documentation, and produce architecture diagrams.

## Installation

### Option 1 — cargo install (recommended for Rust users)

```bash
cargo install avdoc
```

### Option 2 — One-line install script (Mac / Linux)

```bash
curl -sSf https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.sh | sh
```

### Option 3 — One-line install script (Windows PowerShell)

```powershell
irm https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.ps1 | iex
```

### Option 4 — Manual download

Download the pre-built binary for your platform from the [latest release](https://github.com/Aryan-202/avdoc/releases/latest):

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `avdoc-linux-x86_64` |
| Linux ARM64 | `avdoc-linux-arm64` |
| macOS x86_64 | `avdoc-macos-x86_64` |
| macOS ARM64 (Apple Silicon) | `avdoc-macos-arm64` |
| Windows x86_64 | `avdoc-windows-x86_64.exe` |

### Requirements

- An API key from OpenAI, Anthropic, or a compatible provider
- Set one of the following environment variables before use:

```bash
export OPENAI_API_KEY=your_key_here
# or
export ANTHROPIC_API_KEY=your_key_here
# or
export AVDOC_API_KEY=your_key_here
```

To use a specific model, set:

```bash
export AVDOC_MODEL=gpt-4o   # defaults to gpt-3.5-turbo
```

---

## Commands

### `avdoc lint`

Lint the repository to evaluate documentation coverage and quality.

```bash
avdoc lint [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--path <DIRECTORY>` | Path to the repository (defaults to current directory) |
| `--min-score <SCORE>` | Enforce a minimum documentation score from 0 to 100 |
| `--format <FORMAT>` | Output format: `terminal`, `json`, `markdown` |

**Examples:**

```bash
# Lint current directory
avdoc lint

# Fail CI if score drops below 80
avdoc lint --min-score 80

# Output as JSON
avdoc lint --format json

# Lint a specific repo path
avdoc lint --path ./my-project
```

---

### `avdoc diagram`

Generate architecture diagrams and visually map the repository structure.

```bash
avdoc diagram [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--path <DIRECTORY>` | Path to the repository |
| `--format <FORMAT>` | Output format: `mermaid` (default), `ascii` |
| `--update-readme` | Automatically append the diagram to README.md |

**Examples:**

```bash
# Generate a Mermaid diagram
avdoc diagram

# Generate ASCII diagram
avdoc diagram --format ascii

# Generate and append to README
avdoc diagram --update-readme
```

---

### `avdoc heal`

Analyze and automatically generate missing documentation for the codebase using AI.

```bash
avdoc heal [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--path <DIRECTORY>` | Path to the repository |
| `--files <FILES>...` | Target specific files; if omitted, targets all low-scoring files |
| `--interactive` | Ask for confirmation before applying each change |

**Examples:**

```bash
# Heal all underdocumented files
avdoc heal

# Heal specific files
avdoc heal --files src/main.rs src/lib.rs

# Review each change before applying
avdoc heal --interactive
```

---

## Updating

Re-run your original install command to get the latest version. If you installed via `cargo`:

```bash
cargo install avdoc
```

avdoc follows [semantic versioning](https://semver.org/): patch versions (`0.1.x`) are bug fixes, minor versions (`0.x.0`) add new features, and major versions (`x.0.0`) indicate breaking changes.

---

## Supported Languages

avdoc currently parses and scores documentation for:

- Rust
- Python
- JavaScript / TypeScript
- Go
- Java
- C / C++

---

## CI Integration

You can use avdoc as a documentation gate in CI pipelines. It exits with a non-zero code if the score falls below the minimum:

```yaml
# GitHub Actions example
- name: Check documentation score
  run: avdoc lint --min-score 70
  env:
    OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
```

---

## License

MIT License — see [LICENSE](LICENSE) for details.