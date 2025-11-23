# EWS Client

Fast EWS (Exchange Web Services) client implementation using Rust with Python bindings.

## Features

- 🚀 High-performance Rust core
- 🐍 Python bindings with full type hints
- ⚡ Async/await support
- 🔒 Basic and OAuth2 authentication
- 📦 Pure Rust implementation (no XPCOM dependencies)

## Project Structure

```
ews-client/
├── ews-client-core/      # Pure Rust async EWS client
├── ews-client-python/    # Python bindings (PyO3)
├── python/ews_client/    # Python package
├── tests/                # Tests (Rust + Python)
└── docs/                 # Documentation
```

## Development Status

🚧 **Under Active Development** 🚧

See [docs/roadmap/implementation-plan.md](docs/roadmap/implementation-plan.md) for details.

## Installation

```bash
# Install with uv
uv pip install ews-client

# Or build from source
uv pip install maturin
maturin develop
```

## Quick Start

```python
from ews_client import EwsClient

# Create client
client = EwsClient(
    endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
    username="user@example.com",
    password="password"
)

# Use async methods
# (Implementation in progress)
```

## Development

```bash
# Setup development environment
uv venv
source .venv/bin/activate  # or `.venv\Scripts\activate` on Windows
uv pip install -e ".[dev]"

# Build Rust code
cargo build

# Build Python bindings
maturin develop

# Run tests
cargo test                    # Rust tests
pytest tests/python/          # Python tests

# Code quality
cargo clippy                  # Rust linting
cargo fmt                     # Rust formatting
ruff check python/            # Python linting
mypy python/                  # Python type checking
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## References

Based on:

- [ews-rs](https://github.com/thunderbird/ews-rs) - EWS protocol implementation
- [thunderbird-desktop](https://github.com/thunderbird/thunderbird-desktop) - Reference implementation
