python_packages := "my-logging python-server"
rust_packages := "rust-server"

@_default:
  just --list

# Install packages in local virtual environment
@sync:
  @uv sync
  @for package in {{ rust_packages }}; do \
    cd packages/$package && uv tool run maturin develop; \
  done

# Create python packages
@build:
  @for package in {{ python_packages }}; do \
    uv build --package $package; \
  done
  @for package in {{ rust_packages }}; do \
    uv tool run maturin build --release --strip --zig --manifest-path packages/$package/Cargo.toml && mv target/wheels/* dist/; \
  done

# Clean all build artefacts
@clean:
  @cargo clean

# Format source code
@format:
  @uv run ruff check --fix
  @uv run ruff format
  @cargo fmt
  @cargo clippy

# Run unit tests
@test:
  @cargo nextest run
  @uv run pytest

# Run integration tests
@test-integration:
  @uv run robot tests/robot

# Format code in CI pipeline
@ci_format:
  @uv run pre-commit run --all-files
  @cargo fmt --check
  @cargo clippy

# Audit the codebase
@audit:
  @uv-secure
  @cargo audit
  @cargo license

# Start HTTP server
@server:
  @uv run python-server
