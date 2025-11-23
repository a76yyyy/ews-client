# Python API Documentation

## Overview

`ews_client` 提供基于 Rust 的高性能 EWS 客户端 Python 绑定。

## 安装

使用 UV 安装:

```bash
uv add ews-client
```

或使用 pip:

```bash
pip install ews-client
```

## 核心类型

### EwsClient

与 Exchange Web Services 交互的主客户端。

```python
class EwsClient:
    def __init__(self, endpoint: str, username: str, password: str) -> None:
        """创建 EWS 客户端。

        Args:
            endpoint: EWS 端点 URL
            username: 认证用户名
            password: 认证密码
        """
```

### 数据类型

`ews_client.types` 模块提供以下数据类:

#### FolderInfo

```python
@dataclass
class FolderInfo:
    """EWS 文件夹信息。"""
    id: str
    parent_id: str | None
    display_name: str
```

#### FolderSyncResult

```python
@dataclass
class FolderSyncResult:
    """文件夹同步结果。"""
    folders: list[FolderInfo]
    sync_state: str
```

#### MessageInfo

```python
@dataclass
class MessageInfo:
    """EWS 消息信息。"""
    id: str
    subject: str | None
    from_: str | None
    is_read: bool
```

#### MessageSyncResult

```python
@dataclass
class MessageSyncResult:
    """消息同步结果。"""
    messages: list[MessageInfo]
    sync_state: str
```

## 使用方法

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

所有操作都是异步的,必须使用 `await`:

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

asyncio.run(main())
```

## 错误处理

操作失败时会抛出异常,使用标准 Python 异常处理:

```python
try:
    result = await client.some_operation()
except Exception as e:
    print(f"错误: {e}")
```

## 类型检查

提供完整的类型提示(`.pyi` 文件)。使用 mypy 进行类型检查:

```bash
uv run mypy your_script.py
```

使用 ruff 进行代码检查:

```bash
uv run ruff check your_script.py
```

## 开发要求

- Python >= 3.10
- 支持类型提示 (PEP 604: `str | None`)
- 使用 UV 作为包管理器
