# 开发指南

## 环境要求

### 必需工具

- Rust 1.70+
- Python 3.10+
- UV (Python 包管理器)

### 初始设置

```bash
# 克隆仓库
git clone https://github.com/a76yyyy/ews-client.git
cd ews-client

# 安装所有依赖并设置 pre-commit
make install
```

这会自动:

- 创建虚拟环境
- 安装所有开发依赖
- 安装 pre-commit hooks

## 构建

### 开发构建

```bash
make build-dev
```

构建 Python 扩展(debug 模式),适合日常开发。

### 生产构建

```bash
make build-prod
```

构建 Python 扩展(release 模式),启用优化。

### 其他构建选项

```bash
make build-profiling    # 启用性能分析
make build-coverage     # 启用代码覆盖率
make build-pgo          # 使用 PGO 优化
```

### 仅构建 Rust

```bash
cargo build              # Debug 模式
cargo build --release    # Release 模式
```

## 测试

### 运行所有测试

```bash
make test
```

使用 pytest 并行运行 Python 测试。

### Rust 测试

```bash
make test-rust
# 或
cargo test
cargo test -- --nocapture  # 显示输出
```

### Python 测试

```bash
uv run pytest tests/python/ -v
uv run pytest tests/python/ -v -s  # 显示输出
```

### 运行特定测试

```bash
uv run pytest tests/python/test_client.py::test_client_creation -v
```

### 代码覆盖率

```bash
make testcov
```

生成 Python 和 Rust 的代码覆盖率报告,输出到 `htmlcov/` 目录。

## 代码质量

### 格式化代码

```bash
make format
```

自动格式化所有代码:

- `cargo fmt` - Rust 代码
- `ruff format` - Python 代码

### 代码检查

```bash
make lint
```

运行所有检查:

- `cargo clippy` - Rust linting
- `cargo fmt --check` - Rust 格式检查
- `ruff check` - Python linting
- `ruff format --check` - Python 格式检查
- `mypy stubtest` - 类型存根验证

### 单独运行检查

```bash
make lint-rust      # 仅 Rust
make lint-python    # 仅 Python
make check-rust     # Rust 编译检查(不构建)
```

### Pre-commit Hooks

Pre-commit hooks 在 `make install` 时自动安装,包含:

- 通用文件检查(YAML, TOML, 行尾等)
- Ruff 自动修复和格式化
- Mypy 类型检查
- Rust 格式化和 clippy
- Cargo check 和 test(仅 pre-push)

手动运行:

```bash
uv run pre-commit run --all-files
```

## 项目结构

```
ews-client/
├── ews-client-core/          # Rust 核心库
│   ├── Cargo.toml
│   ├── src/
│   │   ├── client/           # 客户端实现
│   │   │   ├── credentials.rs # 认证凭据
│   │   │   ├── error.rs      # 错误类型 (EwsError)
│   │   │   ├── headers.rs    # 消息头处理 (MessageHeaders trait)
│   │   │   ├── mod.rs        # 主客户端 (EwsClient)
│   │   │   ├── operations/   # EWS 操作实现
│   │   │   │   ├── change_read_status.rs
│   │   │   │   ├── check_connectivity.rs
│   │   │   │   ├── copy_move_operations/
│   │   │   │   │   ├── folder.rs
│   │   │   │   │   ├── item.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── create_folder.rs
│   │   │   │   ├── create_message.rs
│   │   │   │   ├── delete_folder.rs
│   │   │   │   ├── delete_messages.rs
│   │   │   │   ├── get_message.rs
│   │   │   │   ├── mark_as_junk.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── send_message.rs
│   │   │   │   ├── sync_folder_hierarchy.rs
│   │   │   │   ├── sync_messages.rs
│   │   │   │   ├── update_folder.rs
│   │   │   │   └── update_item.rs
│   │   │   ├── server_version.rs # 服务器版本检测
│   │   │   └── types.rs      # 数据类型
│   │   └── lib.rs            # 库入口
│   └── tests/                # 测试
│       ├── README.md
│       ├── common/           # 测试公共工具
│       │   ├── fixtures.rs
│       │   ├── image/
│       │   ├── mock_server.rs
│       │   ├── mod.rs
│       │   └── test_utils.rs
│       ├── integration/      # 集成测试
│       │   ├── mock/
│       │   │   ├── folder_operations.rs
│       │   │   ├── infrastructure.rs
│       │   │   └── item_operations.rs
│       │   └── real/
│       │       ├── folder_operations.rs
│       │       └── item_operations.rs
│       ├── integration_mock.rs
│       ├── integration_real.rs
│       ├── lib.rs
│       ├── unit/             # 单元测试
│       │   └── operations.rs
│       └── unit.rs
│
├── ews-client-python/        # Python 绑定 (PyO3)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # PyO3 模块入口
│       ├── client.rs         # Python 客户端包装
│       ├── error.rs          # 错误映射
│       └── types.rs          # 类型转换
│
├── python/ews_client/        # Python 包
│   ├── __init__.py           # 包入口
│   ├── _ews_client.pyi       # 类型存根 (完整 API)
│   ├── types.py              # Python 数据类型
│   └── py.typed              # PEP 561 标记
│
├── tests/
│   ├── python/               # Python 测试
│   │   ├── conftest.py
│   │   ├── test_client.py
│   │   └── test_types.py
│   └── rust/                 # Rust 单元测试
│       └── mod.rs
│
├── examples/
│   ├── python/               # Python 示例
│   │   └── basic_usage.py
│   └── rust/                 # Rust 示例
│       └── basic_client.rs
│
├── docs/
│   ├── api/                  # API 文档
│   │   ├── rust.md           # Rust API 文档
│   │   └── python.md         # Python API 文档
│   ├── examples/             # 使用示例
│   │   └── basic_usage.md
│   ├── roadmap/              # 开发路线图
│   │   ├── implementation-plan.md
│   │   ├── architecture.md
│   │   └── reference-mapping.md
│   └── development.md        # 本文档
│
├── reference/                # 参考实现 (Thunderbird)
│   └── thunderbird-desktop/
│
├── Makefile                  # 构建脚本
├── pyproject.toml            # Python 项目配置
├── uv.lock                   # UV 锁文件
├── Cargo.toml                # Rust 工作空间配置
├── Cargo.lock                # Rust 锁文件
├── build.rs                  # 构建脚本
└── .pre-commit-config.yaml   # Pre-commit 配置
```

## 添加新操作

### 步骤

1. **实现 Rust 操作**
   - 在 `ews-client-core/src/client/operations/` 中创建新文件
   - 实现 `impl EwsClient` 方法
   - 在 `operations/mod.rs` 中导出

2. **添加 Rust 测试**
   - 在 `ews-client-core/tests/` 中添加集成测试
   - 运行 `cargo test` 验证

3. **添加 Python 绑定** (可选,如需暴露给 Python)
   - 在 `ews-client-python/src/client.rs` 中添加 `#[pymethods]`
   - 处理类型转换和错误映射

4. **更新类型定义**
   - 在 `python/ews_client/types.py` 中添加数据类型
   - 在 `python/ews_client/_ews_client.pyi` 中添加类型存根

5. **添加 Python 测试**
   - 在 `tests/python/` 中添加测试
   - 运行 `make test` 验证

6. **更新文档**
   - 更新 `docs/api/rust.md`
   - 更新 `docs/api/python.md`
   - 添加使用示例到 `docs/examples/`

### 示例: 添加新操作

```rust
// ews-client-core/src/client/operations/my_operation.rs
use crate::client::{EwsClient, EwsError};

impl EwsClient {
    /// 我的新操作
    pub async fn my_operation(&self, param: &str) -> Result<String, EwsError> {
        // 实现逻辑
        Ok(param.to_string())
    }
}
```

```rust
// ews-client-core/src/client/operations/mod.rs
mod my_operation;
```

```python
# ews-client-python/src/client.rs
#[pymethods]
impl PyEwsClient {
    fn my_operation<'py>(&self, py: Python<'py>, param: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client.my_operation(&param).await.map_err(Into::into)
        })
    }
}
```

## 调试

### Rust 调试

```bash
# 显示回溯
RUST_BACKTRACE=1 cargo test

# 显示完整回溯
RUST_BACKTRACE=full cargo test

# 显示日志
RUST_LOG=debug cargo test
```

### Python 调试

```bash
# 详细输出
uv run pytest tests/python/ -vv -s

# 使用 pdb
uv run pytest tests/python/ --pdb

# 仅运行失败的测试
uv run pytest tests/python/ --lf
```

### 生成 Rust 文档

```bash
make doc-rust
```

生成并在浏览器中打开 Rust API 文档。

## 常见问题

### 构建失败: "ews not found"

更新依赖:

```bash
cargo update
```

### Python 测试失败: "module not found"

重新构建 Python 绑定:

```bash
make build-dev
```

### 类型检查失败

确保已安装最新依赖:

```bash
uv sync --group all
```

### Pre-commit 检查失败

手动运行并查看详细错误:

```bash
uv run pre-commit run --all-files --verbose
```

## 清理

```bash
make clean
```

删除所有构建产物:

- Python 缓存和 `.so` 文件
- Rust `target/` 目录
- 测试缓存和覆盖率报告
- 临时文件

## 完整开发流程

```bash
# 1. 安装依赖
make install

# 2. 开发构建
make build-dev

# 3. 运行测试
make test

# 4. 格式化代码
make format

# 5. 代码检查
make lint

# 6. 提交前检查
uv run pre-commit run --all-files
```

或使用一键命令:

```bash
make all  # 格式化、构建、检查、测试
```

## Makefile 目标

运行 `make help` 查看所有可用目标:

```bash
make help
```

### 主要目标

**安装和设置**:

- `install` - 安装所有依赖并设置 pre-commit hooks
- `rebuild-lockfiles` - 重新构建锁文件,更新所有依赖

**构建目标**:

- `build-dev` - 开发构建 (debug 模式)
- `build-prod` - 生产构建 (release 模式,启用优化)
- `build-profiling` - 启用性能分析的构建
- `build-coverage` - 启用代码覆盖率的构建
- `build-pgo` - 使用 Profile-Guided Optimization 的构建

**Rust 目标**:

- `check-rust` - 检查 Rust 代码但不构建
- `test-rust` - 运行 Rust 测试
- `lint-rust` - Lint Rust 代码 (fmt + clippy)
- `doc-rust` - 生成并打开 Rust 文档

**Python 目标**:

- `lint-python` - Lint Python 代码 (ruff + mypy stubtest)
- `test` - 运行 Python 测试 (并行)
- `testcov` - 运行测试并生成覆盖率报告

**组合目标**:

- `format` - 格式化所有代码 (Python + Rust)
- `lint` - Lint 所有代码 (Python + Rust)
- `all` - 完整开发流程 (format + build + lint + test)
- `clean` - 清理所有构建产物

## 依赖管理

### 更新 Python 依赖

```bash
# 更新所有依赖
make rebuild-lockfiles

# 添加新依赖
uv add <package>

# 添加开发依赖
uv add --group dev <package>
```

### 更新 Rust 依赖

```bash
# 更新所有依赖
cargo update

# 更新特定依赖
cargo update -p <package>
```

## 性能优化

### Profile-Guided Optimization (PGO)

```bash
make build-pgo
```

使用实际测试数据优化编译,可显著提升性能。

### 性能分析

```bash
make build-profiling
uv run pytest tests/python/
# 使用 perf, flamegraph 等工具分析
```

## 参考资源

- [ews-rs 文档](https://github.com/thunderbird/ews-rs)
- [PyO3 指南](https://pyo3.rs/)
- [Tokio 文档](https://tokio.rs/)
- [Reqwest 文档](https://docs.rs/reqwest/)
- [UV 文档](https://docs.astral.sh/uv/)
- [Ruff 文档](https://docs.astral.sh/ruff/)
