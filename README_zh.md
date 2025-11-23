# EWS Client

[English](README.md)

基于 Rust 的高性能 EWS (Exchange Web Services) 客户端,提供 Python 绑定。

## 特性

- 🚀 高性能 Rust 核心
- 🐍 完整类型提示的 Python 绑定
- ⚡ 异步支持 (async/await)
- 🔒 Basic 和 OAuth2 认证
- 📦 纯 Rust 实现(无 XPCOM 依赖)

## 项目结构

```
ews-client/
├── ews-client-core/      # 纯 Rust 异步 EWS 客户端
├── ews-client-python/    # Python 绑定 (PyO3)
├── python/ews_client/    # Python 包
├── tests/                # 测试 (Rust + Python)
└── docs/                 # 文档
```

## 开发状态

🚧 **开发中** 🚧

查看 [docs/roadmap/implementation-plan.md](docs/roadmap/implementation-plan.md) 了解详情。

查看 [实现计划](docs/roadmap/implementation-plan.md) 了解详细路线图。

## 安装

```bash
# 使用 UV 安装
uv add ews-client

# 或使用 pip
pip install ews-client

# 或从源码构建
uv pip install maturin
maturin develop
```

## 快速开始

### Python

```python
from ews_client import EwsClient

# 创建客户端
client = EwsClient(
    endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
    username="user@example.com",
    password="password"
)

# 使用异步方法
# (功能实现中)
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

    // 检查是否为 Office365
    if client.is_office365() {
        println!("连接到 Office365 服务器");
    }

    Ok(())
}
```

## 开发

### 环境设置

```bash
# 克隆仓库
git clone https://github.com/a76yyyy/ews-client.git
cd ews-client

# 安装依赖并设置 pre-commit
make install
```

### 构建

```bash
# 开发构建
make build-dev

# 生产构建
make build-prod

# 仅构建 Rust
cargo build
cargo build --release
```

### 测试

```bash
# 运行所有测试
make test

# Rust 测试
make test-rust
cargo test

# Python 测试
uv run pytest tests/python/ -v
```

### 代码质量

```bash
# 格式化代码
make format

# 代码检查
make lint

# 单独检查
make lint-rust      # Rust
make lint-python    # Python
```

### 完整开发流程

```bash
make all  # 格式化、构建、检查、测试
```

查看 [docs/development.md](docs/development.md) 了解详细开发指南。

## 文档

- [Python API 文档](docs/api/python.md)
- [Rust API 文档](docs/api/rust.md)
- [基本使用示例](docs/examples/basic_usage.md)
- [开发指南](docs/development.md)

## 许可证

Mozilla Public License Version 2.0 (MPL-2.0) - 查看 [LICENSE](LICENSE) 了解详情。

## 参考

基于以下项目:

- [ews-rs](https://github.com/thunderbird/ews-rs) - EWS 协议实现
- [thunderbird-desktop](https://github.com/thunderbird/thunderbird-desktop) - 参考实现
