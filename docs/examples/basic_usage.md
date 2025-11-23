# 基本使用

## Python 示例

**注意**: Python 绑定目前处于早期开发阶段。以下示例展示了计划中的 API。

### 安装

使用 UV (推荐):

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

# 创建客户端
client = EwsClient(
    endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
    username="user@example.com",
    password="password"
)
```

### 计划中的异步操作

所有操作都将是异步的:

```python
import asyncio
from ews_client import EwsClient

async def main():
    client = EwsClient(
        endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
        username="user@example.com",
        password="password"
    )

    # 测试连接
    await client.check_connectivity()
    print("连接成功!")

    # 同步文件夹
    result = await client.sync_folder_hierarchy(sync_state=None)
    print(f"创建了 {len(result.created_folders)} 个文件夹")

    # 同步消息
    if result.well_known_folders and "inbox" in result.well_known_folders:
        inbox_id = result.well_known_folders["inbox"]
        message_result = await client.sync_messages(
            folder_id=inbox_id,
            sync_state=None
        )
        print(f"新消息: {len(message_result.created)}")

if __name__ == "__main__":
    asyncio.run(main())
```

### 文件夹操作 (计划中)

```python
import asyncio
from ews_client import EwsClient

async def main():
    client = EwsClient(
        endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
        username="user@example.com",
        password="password"
    )

    # 创建文件夹
    folder_id = await client.create_folder(
        parent_id="parent_folder_id",
        name="New Folder"
    )
    print(f"创建文件夹: {folder_id}")

    # 更新文件夹名称
    await client.update_folder(
        folder_id=folder_id,
        folder_name="Renamed Folder"
    )

    # 移动文件夹
    new_ids = await client.move_folders(
        destination_folder_id="dest_folder_id",
        folder_ids=[folder_id]
    )
    print(f"移动后的 ID: {new_ids}")

    # 删除文件夹
    await client.delete_folder(folder_ids=[folder_id])

asyncio.run(main())
```

### 消息操作 (计划中)

```python
import asyncio
from ews_client import EwsClient

async def main():
    client = EwsClient(
        endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
        username="user@example.com",
        password="password"
    )

    # 获取消息内容
    mime_content = await client.get_message(message_id="message_id")
    print(f"消息大小: {len(mime_content)} 字节")

    # 创建消息
    mime_data = b"From: user@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nBody"
    result = await client.create_message(
        folder_id="folder_id",
        content=mime_data,
        is_draft=False,
        is_read=True
    )
    print(f"创建消息: {result.item_id}")

    # 标记为已读
    updated_ids = await client.change_read_status(
        item_ids=["msg_id_1", "msg_id_2"],
        is_read=True
    )
    print(f"已更新 {len(updated_ids)} 条消息")

    # 删除消息
    await client.delete_messages(item_ids=["msg_id_1", "msg_id_2"])

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
    except ValueError as e:
        print(f"无效的端点 URL: {e}")
        return
    except RuntimeError as e:
        print(f"客户端创建失败: {e}")
        return

    try:
        await client.check_connectivity()
        print("连接成功!")
    except Exception as e:
        print(f"连接失败: {e}")

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

### 当前可用功能

目前 Python 绑定仅支持:

```python
from ews_client import EwsClient

# 创建客户端 (已实现)
client = EwsClient(
    endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
    username="user@example.com",
    password="password"
)

# 其他操作正在开发中
```

### 临时方案

在 Python 绑定完全实现之前,如果需要使用完整功能:

1. **使用 Rust 库**: 参考 Rust 示例使用 `ews-client-core`
2. **等待更新**: Python 绑定正在积极开发中
3. **贡献代码**: 欢迎贡献 Python 绑定的实现

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
use ews_client_core::{Credentials, EwsClient};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    // 测试连接
    client.check_connectivity().await?;
    println!("连接成功!");

    // 检查是否为 Office365
    if client.is_office365() {
        println!("连接到 Office365 服务器");
    }

    // 同步文件夹
    let result = client.sync_folder_hierarchy(None).await?;
    println!("创建了 {} 个文件夹", result.created_folders.len());

    Ok(())
}
```

### 文件夹操作

```rust
use ews_client_core::{Credentials, EwsClient};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    // 创建文件夹
    let folder_id = client.create_folder("parent_folder_id", "New Folder").await?;
    println!("创建文件夹: {}", folder_id);

    // 更新文件夹名称
    client.update_folder(&folder_id, "Renamed Folder").await?;

    // 移动文件夹
    let new_ids = client.move_folders("dest_folder_id", &[&folder_id]).await?;
    println!("移动后的 ID: {:?}", new_ids);

    // 删除文件夹
    client.delete_folder(&[&folder_id]).await?;

    Ok(())
}
```

### 消息操作

```rust
use ews_client_core::{Credentials, EwsClient};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    // 同步消息
    let result = client.sync_messages("folder_id", None).await?;
    println!("新消息: {}", result.created.len());

    // 获取消息内容
    let mime_content = client.get_message("message_id").await?;
    println!("消息大小: {} 字节", mime_content.len());

    // 标记为已读
    let updated_ids = client.change_read_status(&["msg_id_1", "msg_id_2"], true).await?;
    println!("已更新 {} 条消息", updated_ids.len());

    // 删除消息
    client.delete_messages(&["msg_id_1", "msg_id_2"]).await?;

    Ok(())
}
```

### 错误处理

使用 `?` 操作符:

```rust
use ews_client_core::{Credentials, EwsClient};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    // 操作会自动传播错误
    client.check_connectivity().await?;
    let result = client.sync_folder_hierarchy(None).await?;

    Ok(())
}
```

或使用 `match`:

```rust
use ews_client_core::{Credentials, EwsClient};
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

    let client = match EwsClient::new(endpoint, credentials) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("创建客户端失败: {}", e);
            return;
        }
    };

    // 测试连接
    match client.check_connectivity().await {
        Ok(_) => println!("连接成功"),
        Err(e) => eprintln!("连接失败: {}", e),
    }
}
```

## 更多示例

查看 `examples/` 目录获取更多完整示例:

- `examples/python/basic_usage.py` - Python basic usage
- `examples/python/sync_folders.py` - Folder synchronization (Phase 2)
- `examples/python/oauth2_example.py` - OAuth2 authentication (Phase 5)
- `examples/rust/basic_client.rs` - Rust basic usage
