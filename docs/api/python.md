# Python API Documentation

## Overview

`ews_client` 提供基于 Rust 的高性能 EWS 客户端 Python 绑定。

**注意**: Python 绑定目前处于早期开发阶段。核心 Rust 库已实现完整功能,Python 绑定将在后续阶段逐步完善。

## 安装

使用 UV 安装 (推荐):

```bash
uv add ews-client
```

或使用 pip:

```bash
pip install ews-client
```

## 当前状态

### 已实现

- ✅ 基本客户端创建
- ✅ 基本认证支持

### 开发中

Python 绑定正在积极开发中,将逐步暴露以下 Rust 核心功能:

- 🔄 连接测试
- 🔄 文件夹同步
- 🔄 消息同步
- 🔄 文件夹操作 (创建、更新、删除、移动、复制)
- 🔄 消息操作 (获取、创建、发送、删除、标记已读、移动、复制)
- 🔄 垃圾邮件标记
- 🔄 类型定义和类型提示

## 核心类型

### EwsClient

与 Exchange Web Services 交互的主客户端。

```python
class EwsClient:
    def __init__(self, endpoint: str, username: str, password: str) -> None:
        """创建 EWS 客户端。

        Args:
            endpoint: EWS 端点 URL (例如 "https://outlook.office365.com/EWS/Exchange.asmx")
            username: 认证用户名
            password: 认证密码

        Raises:
            ValueError: 如果端点 URL 无效
            RuntimeError: 如果客户端创建失败
        """
```

### 计划中的数据类型

以下类型将在后续版本中提供:

#### FolderHierarchySyncResult

```python
@dataclass
class FolderHierarchySyncResult:
    """文件夹层次结构同步结果。"""
    sync_state: str
    created_folders: list[FolderInfo]
    updated_folders: list[FolderInfo]
    deleted_folder_ids: list[str]
    well_known_folders: dict[str, str] | None
```

#### FolderInfo

```python
@dataclass
class FolderInfo:
    """EWS 文件夹信息。"""
    folder_id: str
    parent_folder_id: str
    display_name: str
    folder_class: str | None
    total_count: int | None
    unread_count: int | None
    child_folder_count: int | None
```

#### SyncMessagesResult

```python
@dataclass
class SyncMessagesResult:
    """消息同步结果。"""
    created: list[str]
    updated: list[str]
    deleted: list[str]
    read_status_changed: list[tuple[str, bool]]
    sync_state: str
    includes_last_item: bool
```

#### CreateMessageResult

```python
@dataclass
class CreateMessageResult:
    """创建消息的结果。"""
    item_id: str
```

## 使用示例

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

### 计划中的 API

以下是计划实现的 API 示例:

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

    # 同步文件夹
    result = await client.sync_folder_hierarchy(sync_state=None)
    print(f"创建了 {len(result.created_folders)} 个文件夹")

    # 同步消息
    message_result = await client.sync_messages(
        folder_id="inbox_id",
        sync_state=None
    )
    print(f"新消息: {len(message_result.created)}")

    # 创建文件夹
    folder_id = await client.create_folder(
        parent_id="parent_id",
        name="New Folder"
    )

    # 标记消息为已读
    updated_ids = await client.change_read_status(
        item_ids=["msg_id_1", "msg_id_2"],
        is_read=True
    )

asyncio.run(main())
```

## 错误处理

操作失败时会抛出异常,使用标准 Python 异常处理:

```python
try:
    client = EwsClient(
        endpoint="https://outlook.office365.com/EWS/Exchange.asmx",
        username="user@example.com",
        password="password"
    )
except ValueError as e:
    print(f"无效的端点 URL: {e}")
except RuntimeError as e:
    print(f"客户端创建失败: {e}")
```

异步操作的错误处理:

```python
import asyncio

async def main():
    client = EwsClient(...)

    try:
        await client.check_connectivity()
    except Exception as e:
        print(f"连接失败: {e}")

asyncio.run(main())
```

## 类型检查

Python 绑定将提供完整的类型提示 (`.pyi` 文件)。

使用 mypy 进行类型检查:

```bash
uv run mypy your_script.py
```

使用 ruff 进行代码检查和格式化:

```bash
uv run ruff check your_script.py
uv run ruff format your_script.py
```

## 开发要求

- Python >= 3.10
- 支持类型提示 (PEP 604: `str | None`)
- 使用 UV 作为包管理器
- mypy 用于类型检查
- ruff 用于代码检查和格式化

## 临时方案

在 Python 绑定完全实现之前,如果需要使用完整功能,可以:

1. **使用 Rust 库**: 直接使用 `ews-client-core` Rust crate
2. **等待更新**: Python 绑定正在积极开发中,将很快提供完整功能
3. **贡献代码**: 欢迎贡献 Python 绑定的实现

## 开发路线图

Python 绑定的开发计划:

1. **Phase 1** (当前): 基本客户端创建
2. **Phase 2**: 连接测试和文件夹同步
3. **Phase 3**: 消息同步和基本操作
4. **Phase 4**: 完整的文件夹和消息操作
5. **Phase 5**: 类型定义和文档完善

详见项目 [roadmap](../roadmap/implementation-plan.md) 文档。
