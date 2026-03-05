# CNB CLI

[![License](https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-edition%202024-orange?style=flat-square)](https://www.rust-lang.org/)

An unofficial command line tool for [CNB](https://cnb.cool). Manage your CNB platform resources efficiently from the terminal.

> [!NOTE]
> This is **NOT** an official CNB product. It is a community-driven project.

## Features

- **Authentication** — Login, check status, logout
- **AI Chat** — Interact with CNB OpenAPI using natural language
- **Issue Management** — Create, list, close, comment, download issues
- **Pull Request Management** — Create, update, merge pull requests
- **Release Management** — Create releases, upload/clean assets
- **Commit Management** — Upload/clean commit assets
- **File Download** — Download repository files by path/branch with glob filtering
- **Statistics Dashboard** — Commit leaderboard, commit trends, star trends (TUI)
- **Knowledge Base** — Query and manage repository embedding knowledge base
- **Organization** — Update organization logo
- **Workspace** — Clean up closed cloud-native workspaces
- **Shell Completion** — Bash, Zsh, Fish, PowerShell

## Installation

Download the binary for your platform from the [Releases page](https://cnb.cool/prevailna/cnb/-/releases).

## Quick Start

```bash
# Login to CNB
cnb auth login

# View repository info
cnb info

# List issues
cnb issue list

# Chat with AI
cnb chat --do "List my repositories"

# Download files
cnb download --files README.md
```

Also available as a Git subcommand:

```bash
git cnb info
git cnb issue list
```

## Project Structure

```
cnb/
├── src/                  # Main binary (cnb / git-cnb)
│   ├── main.rs
│   └── commands/         # Command implementations
├── crates/
│   ├── cnb-api/          # CNB platform API client
│   ├── cnb-chat/         # AI chat functionality
│   ├── cnb-core/         # Core types and context
│   └── cnb-tui/          # Terminal UI (stats, stars)
└── docs/                 # VitePress documentation site
```

## Building from Source

**Prerequisites:** [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

```bash
git clone https://cnb.cool/prevailna/cnb.git
cd cnb
cargo build --release
```

The binary will be at `target/release/cnb` (or `target/release/cnb.exe` on Windows).

## Documentation

- **https://cnb.wwvo.fun**
- **https://cnba.pages.dev**

## License

[Apache-2.0](LICENSE)
