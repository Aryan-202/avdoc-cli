# avdoc

AI-powered documentation gatekeeper and architecture visualizer.

## Overview

avdoc provides tools to analyze and improve repository documentation. It can evaluate codebases to generate a documentation quality score, automatically generate missing documentation, and produce architecture diagrams.

## Commands

### Lint

Lint the repository to evaluate documentation coverage and quality.
`avdoc lint [OPTIONS]`

Options:

- `--path <DIRECTORY>`: Path to the repository (defaults to current directory)
- `--min-score <SCORE>`: Enforce a minimum documentation score from 0 to 100
- `--format <FORMAT>`: Output format (terminal, json, markdown)

### Diagram

Generate architecture diagrams and visually map the repository structure.
`avdoc diagram [OPTIONS]`

Options:

- `--path <DIRECTORY>`: Path to the repository
- `--format <FORMAT>`: Output diagram format (mermaid, ascii)
- `--update-readme`: Automatically append the generated architecture diagram to README.md

### Heal

Analyze and automatically generate missing documentation for the codebase.
`avdoc heal [OPTIONS]`

Options:

- `--path <DIRECTORY>`: Path to the repository
- `--files <FILES>...`: Target specific files; if omitted, targets all low-scoring files
- `--interactive`: Ask for confirmation before applying documentation changes

## Installation and Usage

Run `avdoc --help` for full usage details and available flags.
