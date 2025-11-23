.DEFAULT_GOAL := all

# Python sources
sources = python/ews_client tests

# Rust workspace members
rust_packages = ews-client-core ews-client-python

mypy-stubtest = uv run python -m mypy.stubtest ews_client._ews_client

# using pip install cargo (via maturin via pip) doesn't get the tty handle
# so doesn't render color without some help
export CARGO_TERM_COLOR=$(shell (test -t 0 && echo "always") || echo "auto")
# maturin develop only makes sense inside a virtual env, is otherwise
# more or less equivalent to pip install -e just a little nicer
USE_MATURIN = $(shell [ "$$VIRTUAL_ENV" != "" ] && (which maturin))

.PHONY: .uv  ## Check that uv is installed
.uv:
	@uv -V || echo 'Please install uv: https://docs.astral.sh/uv/getting-started/installation/'

.PHONY: .pre-commit  ## Check that pre-commit is installed
.pre-commit:
	@uv run pre-commit -V || echo 'Please install pre-commit: https://pre-commit.com/'

.PHONY: install
install: .uv .pre-commit
	uv pip install -U wheel
	uv sync --frozen --group all
	uv pip install -v -e .
	uv run pre-commit install

.PHONY: rebuild-lockfiles  ## Rebuild lockfiles from scratch, updating all dependencies
rebuild-lockfiles: .uv
	uv lock --upgrade

.PHONY: install-rust-coverage
install-rust-coverage:
	cargo install rustfilt coverage-prepare
	rustup component add llvm-tools-preview

.PHONY: install-pgo
install-pgo:
	rustup component add llvm-tools-preview

.PHONY: build-dev
build-dev:
	@rm -f python/ews_client/*.so
	uv run maturin develop --uv

.PHONY: build-prod
build-prod:
	@rm -f python/ews_client/*.so
	uv run maturin develop --uv --release

.PHONY: build-profiling
build-profiling:
	@rm -f python/ews_client/*.so
	uv run maturin develop --uv --profile profiling

.PHONY: build-coverage
build-coverage:
	@rm -f python/ews_client/*.so
	RUSTFLAGS='-C instrument-coverage' uv run maturin develop --uv --release

.PHONY: build-pgo
build-pgo:
	@rm -f python/ews_client/*.so
	$(eval PROFDATA := $(shell mktemp -d))
	RUSTFLAGS='-Cprofile-generate=$(PROFDATA)' uv run maturin develop --uv --release
	pytest tests
	$(eval LLVM_PROFDATA := $(shell rustup run stable bash -c 'echo $$RUSTUP_HOME/toolchains/$$RUSTUP_TOOLCHAIN/lib/rustlib/$$(rustc -Vv | grep host | cut -d " " -f 2)/bin/llvm-profdata'))
	$(LLVM_PROFDATA) merge -o $(PROFDATA)/merged.profdata $(PROFDATA)
	RUSTFLAGS='-Cprofile-use=$(PROFDATA)/merged.profdata' uv run maturin develop --uv --release
	@rm -rf $(PROFDATA)


.PHONY: format
format:
	uv run ruff check --fix $(sources)
	uv run ruff format $(sources)
	cargo fmt

.PHONY: lint-python
lint-python:
	uv run ruff check $(sources)
	uv run ruff format --check $(sources)
	uv run griffe dump -f -d google -LWARNING -o/dev/null python/ews_client
	$(mypy-stubtest)

.PHONY: lint-rust
lint-rust:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: check-rust
check-rust:
	cargo check --workspace --all-targets --all-features

.PHONY: test-rust
test-rust:
	cargo test --workspace --all-features

.PHONY: doc-rust
doc-rust:
	cargo doc --workspace --all-features --no-deps --open

.PHONY: lint
lint: lint-python lint-rust

.PHONY: test
test:
	uv run pytest -n logical

.PHONY: testcov
testcov: build-coverage
	@rm -rf htmlcov
	@mkdir -p htmlcov
	coverage run -m pytest
	coverage report
	coverage html -d htmlcov/python
	coverage-prepare html python/ews_client/*.so

.PHONY: all
all: format build-dev lint test

.PHONY: clean
clean:
	rm -rf `find . -name __pycache__`
	rm -f `find . -type f -name '*.py[co]' `
	rm -f `find . -type f -name '*~' `
	rm -f `find . -type f -name '.*~' `
	rm -rf src/self_schema.py
	rm -rf .cache
	rm -rf htmlcov
	rm -rf .pytest_cache
	rm -rf *.egg-info
	rm -f .coverage
	rm -f .coverage.*
	rm -rf build
	rm -rf perf.data*
	rm -rf python/ews_client/*.so
	rm -rf *.profraw
	cargo clean

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  install              - Install all dependencies and setup pre-commit"
	@echo "  rebuild-lockfiles    - Rebuild lockfiles from scratch"
	@echo ""
	@echo "Build targets:"
	@echo "  build-dev            - Build Python extension in debug mode"
	@echo "  build-prod           - Build Python extension in release mode"
	@echo "  build-profiling      - Build with profiling enabled"
	@echo "  build-coverage       - Build with coverage instrumentation"
	@echo "  build-pgo            - Build with profile-guided optimization"
	@echo ""
	@echo "Rust targets:"
	@echo "  check-rust           - Check Rust code without building"
	@echo "  test-rust            - Run Rust tests"
	@echo "  lint-rust            - Lint Rust code"
	@echo "  doc-rust             - Generate and open Rust documentation"
	@echo ""
	@echo "Python targets:"
	@echo "  lint-python          - Lint Python code"
	@echo "  test                 - Run Python tests"
	@echo "  testcov              - Run tests with coverage"
	@echo ""
	@echo "Combined targets:"
	@echo "  format               - Format all code (Python + Rust)"
	@echo "  lint                 - Lint all code (Python + Rust)"
	@echo "  all                  - Format, build, lint, and test"
	@echo "  clean                - Clean all build artifacts"
