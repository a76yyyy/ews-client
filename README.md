# EWS Client

[中文文档](README_zh.md)

High-performance EWS (Exchange Web Services) client based on Rust with Python bindings.

## Features

- 🚀 **High-performance Rust core** - Built on tokio async runtime
- 🐍 **Python bindings with full type hints** - Via PyO3
- ⚡ **Async support** - Native async/await
- 🔒 **Multiple authentication methods** - Basic auth (OAuth2 planned)
- 📦 **Pure Rust implementation** - No XPCOM dependencies
- 🔄 **Automatic throttling handling** - Smart retry and backoff
- 📊 **Server version detection** - Auto-adapt to different Exchange versions
- 🎯 **Batch operation optimization** - Automatic batching for performance

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

### Implemented Features

**Rust Core Library** (`ews-client-core`):

- ✅ Connection testing and authentication
- ✅ Folder operations (sync, create, update, delete, move, copy)
- ✅ Message operations (sync, get, create, send, delete, move, copy)
- ✅ Read status management
- ✅ Junk mail marking
- ✅ Server version detection
- ✅ Automatic throttling handling
- ✅ Batch operation optimization

**Python Bindings** (`ews-client-python`):

- ✅ Basic client creation
- 🔄 Operation method bindings (in progress)
- 🔄 Complete type definitions (in progress)

See [Implementation Plan](docs/roadmap/implementation-plan.md) for detailed roadmap.

## Installation

```bash
# Install with UV
uv add ews-client

# Or with pip
pip install ews-client

# Or build from source
uv pip install maturin
maturin develop
```

## Quick Start

### Python

```python
from ews_client import EwsClient

# Create client
client = EwsClient(
    endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
    username="user@example.com",
    password="password"
)

# Use async methods
# (Features under implementation)
```

### Rust

```rust
use ews_client_core::{EwsClient, Credentials};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    // Test connection
    client.check_connectivity().await?;
    println!("Connected successfully!");

    // Sync folders
    let result = client.sync_folder_hierarchy(None).await?;
    println!("Created {} folders", result.created_folders.len());

    Ok(())
}
```

## Development

### Environment Setup

```bash
# Clone repository
git clone https://github.com/a76yyyy/ews-client.git
cd ews-client

# Install dependencies and setup pre-commit
make install
```

### Build

```bash
# Development build
make build-dev

# Production build
make build-prod

# Rust only
cargo build
cargo build --release
```

### Testing

```bash
# Run all tests
make test

# Rust tests
make test-rust
cargo test

# Python tests
uv run pytest tests/python/ -v
```

### Code Quality

```bash
# Format code
make format

# Lint code
make lint

# Individual checks
make lint-rust      # Rust
make lint-python    # Python
```

### Complete Development Workflow

```bash
make all  # Format, build, lint, and test
```

See [docs/development.md](docs/development.md) for detailed development guide.

## Documentation

- [Python API Documentation](docs/api/python.md)
- [Rust API Documentation](docs/api/rust.md)
- [Basic Usage Examples](docs/examples/basic_usage.md)
- [Development Guide](docs/development.md)

## License

Mozilla Public License Version 2.0 (MPL-2.0) - See [LICENSE](LICENSE) for details.

## References

Based on the following projects:

- [ews-rs](https://github.com/thunderbird/ews-rs) - EWS protocol implementation
- [thunderbird-desktop](https://github.com/thunderbird/thunderbird-desktop) - Reference implementation
