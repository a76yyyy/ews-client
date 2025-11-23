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
│   └── src/
│       ├── client/           # 客户端实现
│       │   ├── mod.rs        # 主客户端
│       │   ├── credentials.rs # 认证凭据
│       │   ├── error.rs      # 错误类型
│       │   ├── types.rs      # 数据类型
│       │   └── operations/   # EWS 操作
│       │       └── mod.rs
│       ├── headers.rs        # 消息头处理
│       └── lib.rs            # 库入口
│
├── ews-client-python/        # Python 绑定
│   └── src/
│       ├── lib.rs            # PyO3 模块入口
│       ├── client.rs         # Python 客户端包装
│       ├── error.rs          # 错误映射
│       └── types.rs          # 类型转换
│
├── python/ews_client/        # Python 包
│   ├── __init__.py           # 包入口
│   ├── _ews_client.pyi       # 类型存根
│   ├── types.py              # Python 数据类型
│   └── py.typed              # PEP 561 标记
│
├── tests/
│   ├── python/               # Python 测试
│   │   ├── conftest.py
│   │   └── test_client.py
│   └── rust/                 # Rust 测试
│       └── mod.rs
│
├── examples/
│   ├── python/               # Python 示例
│   └── rust/                 # Rust 示例
│
├── docs/
│   ├── api/                  # API 文档
│   ├── examples/             # 使用示例
│   └── roadmap/              # 开发路线图
│
├── Makefile                  # 构建脚本
├── pyproject.toml            # Python 项目配置
├── Cargo.toml                # Rust 工作空间配置
└── .pre-commit-config.yaml   # Pre-commit 配置
```

## 添加新操作

1. 在 `ews-client-core/src/client/operations/` 中实现操作
2. 在 `ews-client-core/src/client/operations/mod.rs` 中导出
3. 在 `tests/rust/` 中添加测试
4. 在 `ews-client-python/src/client.rs` 中添加 Python 绑定
5. 在 `python/ews_client/_ews_client.pyi` 中更新类型提示
6. 在 `tests/python/` 中添加 Python 测试
7. 更新文档

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

主要目标:

- `install` - 安装依赖和设置环境
- `build-dev` - 开发构建
- `build-prod` - 生产构建
- `test` - 运行测试
- `test-rust` - 运行 Rust 测试
- `lint` - 代码检查
- `lint-rust` - Rust 代码检查
- `lint-python` - Python 代码检查
- `format` - 格式化代码
- `testcov` - 生成覆盖率报告
- `clean` - 清理构建产物
- `all` - 完整开发流程

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
