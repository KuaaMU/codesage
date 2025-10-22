# CodeSage

> 🦀 AI-powered Code Review & Refactoring Tool built with Rust

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

CodeSage is an intelligent code analysis and refactoring tool that combines the power of static analysis, semantic understanding, and AI to help developers write better code.

### Key Features

- 🔍 **Smart Code Review**: Go beyond syntax - understand logic, edge cases, and best practices
- ♻️ **Intelligent Refactoring**: AI-driven suggestions for improving code structure and maintainability
- 📊 **Technical Debt Tracking**: Quantify and visualize code quality over time
- 🔒 **Security Scanning**: Identify vulnerabilities and security issues
- 🚀 **High Performance**: Built with Rust for speed and reliability
- 🔌 **Multiple Interfaces**: CLI, VS Code extension, Git hooks, and web dashboard
- 🏠 **Privacy-First**: Support for local deployment and offline models

### Supported Languages

- Rust
- JavaScript / TypeScript
- Python
- Go
- Java
- C/C++
- More coming soon...

## Quick Start

```bash
# Install via cargo (coming soon)
cargo install codesage

# Review a file
codesage review src/main.rs

# Review entire project
codesage review . --recursive

# Interactive refactoring
codesage refactor src/utils.rs --interactive

# Generate technical debt report
codesage debt --output-html report.html
```

## Project Status

🚧 **Early Development** - This project is currently in active development.

### Roadmap

- [ ] Core architecture and AST parsing (tree-sitter)
- [ ] Basic static analysis engine
- [ ] AI integration (Claude/GPT/Local LLM)
- [ ] CLI interface
- [ ] VS Code extension
- [ ] Web dashboard
- [ ] CI/CD integrations

## Architecture

```
codesage/
├── crates/
│   ├── codesage-core/       # Core analysis engine
│   ├── codesage-parser/     # Multi-language code parsing
│   ├── codesage-analyzer/   # Static & semantic analysis
│   ├── codesage-ai/         # AI/LLM integration
│   ├── codesage-refactor/   # Refactoring engine
│   └── codesage-cli/        # Command-line interface
├── docs/                    # Documentation
├── examples/                # Usage examples
└── tests/                   # Integration tests
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

## License

MIT License - see [LICENSE](LICENSE) file for details

## Acknowledgments

Built with:
- [tree-sitter](https://tree-sitter.github.io/tree-sitter/) - Incremental parsing system
- [Rust](https://www.rust-lang.org/) - Performance and safety
- AI models from Anthropic, OpenAI, and open-source community

---

**Note**: This is an independent open-source project and is not affiliated with Anthropic's Claude Code.
