# Contributing to CodeSage

Thank you for your interest in contributing to CodeSage!

## Development Setup

### Prerequisites

- Rust 1.75 or higher
- Git

### Building from Source

```bash
git clone https://github.com/yourusername/codesage.git
cd codesage
cargo build
cargo test
```

### Running

```bash
cargo run -- review examples/sample.rs
```

## Project Structure

- `crates/codesage-core/` - Core analysis engine
- `crates/codesage-parser/` - Code parsing with tree-sitter
- `crates/codesage-analyzer/` - Static and semantic analysis
- `crates/codesage-ai/` - AI/LLM integrations
- `crates/codesage-refactor/` - Refactoring engine
- `crates/codesage-cli/` - CLI interface

## Contribution Guidelines

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure tests pass (`cargo test`)
6. Format code (`cargo fmt`)
7. Run clippy (`cargo clippy`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to the branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

## Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Add documentation comments for public APIs

## Testing

- Write unit tests for new functionality
- Add integration tests where appropriate
- Maintain or improve code coverage

## Questions?

Feel free to open an issue for any questions or discussions.
