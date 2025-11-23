# 基本使用

## Python 示例

### 安装

使用 UV:

```bash
uv add ews-client
```

或使用 pip:

```bash
pip install ews-client
```

### 创建客户端

```python
from ews_client import EwsClient

client = EwsClient(
    endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
    username="user@example.com",
    password="password"
)
```

### 异步操作

所有操作都是异步的:

```python
import asyncio
from ews_client import EwsClient

async def main():
    client = EwsClient(
        endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
        username="user@example.com",
        password="password"
    )

    # 操作将在后续阶段实现
    # result = await client.sync_folder_hierarchy()
    # print(result)

if __name__ == "__main__":
    asyncio.run(main())
```

### 错误处理

```python
import asyncio
from ews_client import EwsClient

async def main():
    try:
        client = EwsClient(
            endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
            username="user@example.com",
            password="password"
        )
        # 执行操作
    except Exception as e:
        print(f"错误: {e}")

asyncio.run(main())
```

### 类型检查

使用 mypy 进行类型检查:

```bash
uv run mypy your_script.py
```

使用 ruff 进行代码检查和格式化:

```bash
uv run ruff check your_script.py
uv run ruff format your_script.py
```

## Rust 示例

### 添加依赖

```toml
[dependencies]
ews-client-core = "0.1"
tokio = { version = "1", features = ["full"] }
url = "2.5"
```

### 基本使用

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

    // 操作将在后续阶段实现
    // let result = client.sync_folder_hierarchy(None).await?;
    // println!("{:?}", result);

    Ok(())
}
```

### 错误处理

使用 `?` 操作符:

```rust
use ews_client_core::{EwsClient, Credentials};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    // 操作会自动传播错误

    Ok(())
}
```

或使用 `match`:

```rust
use ews_client_core::{EwsClient, Credentials};
use url::Url;

#[tokio::main]
async fn main() {
    let endpoint = match Url::parse("https://outlook.office365.com/EWS/Exchange.asmx") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("无效的 URL: {}", e);
            return;
        }
    };

    let credentials = Credentials::basic("user@example.com", "password");

    match EwsClient::new(endpoint, credentials) {
        Ok(client) => {
            println!("客户端创建成功");
            // 使用客户端
        }
        Err(e) => {
            eprintln!("创建客户端失败: {}", e);
        }
    }
}
```

## 更多示例

查看 `examples/` 目录获取更多完整示例:

- `examples/python/basic_usage.py` - Python basic usage
- `examples/python/sync_folders.py` - Folder synchronization (Phase 2)
- `examples/python/oauth2_example.py` - OAuth2 authentication (Phase 5)
- `examples/rust/basic_client.rs` - Rust basic usage
