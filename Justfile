# <FILE>Justfile</FILE> - <DESC>Project workflows for wsl-clip (Rust/WSL)</DESC>
# <VERS>VERSION: 1.0.0 - 2025-11-25T18:05:53Z</VERS>
# <WCTX>Regenerating with correct OFPF Web envelope tags.</WCTX>
# <CLOG>Initial generation with check, build, test, and install recipes.</CLOG>

# Justfile for wsl-clip (Linux/WSL)
# Run commands with: just <recipe-name>
# List all recipes: just --list
# ============================================================================
# SETUP NOTES - READ THIS IF YOU'RE SETTING UP ON A NEW MACHINE
# ============================================================================
#
# This justfile works seamlessly across Windows and WSL.
#
# WSL SIDE (Linux):
# 1. Install just: cargo install just
# 2. Symlink: sudo ln -s ~/.cargo/bin/just /usr/local/bin/just
#
# WINDOWS SIDE:
# 1. Create C:\Users\<user>\bin\just.bat:
#    @echo off
#    wsl just %*
# 2. Add C:\Users\<user>\bin to Windows PATH
#
# ============================================================================
# Default recipe (shows help)
default:
    @just --list
# ============================================================================
# Common Utilities
# ============================================================================
# Strip ANSI color codes from command output (Crucial for WSL/Tooling compat)
strip-ansi CMD *ARGS:
    #!/usr/bin/env bash
    set -euo pipefail
    {{CMD}} {{ARGS}} 2>&1 | sed 's/\x1B\[[0-9;]*[JKmsu]//g'
# ============================================================================
# Rust / Cargo Workflows
# ============================================================================
# Install the binary locally (to ~/.cargo/bin)
install:
    cargo install --path .
# Build in debug mode
build:
    cargo build
# Build for release (uses [profile.release] optimization)
build-release:
    cargo build --release
# Run unit tests (non-watch mode)
test:
    cargo test
# Run tests with output filtering
test-clean:
    @just strip-ansi cargo test
# ============================================================================
# Quality & Maintenance
# ============================================================================
# Lint code with Clippy
lint:
    cargo clippy -- -D warnings
# Format code (write changes)
format:
    cargo fmt
# Verify formatting without changing files
check-format:
    cargo fmt -- --check
# The "Definition of Done" - runs Format Check, Lint, and Tests
check: check-format lint test
    @echo "Γ£à All checks passed."
# Fix auto-fixable issues (Format + Clippy Fix)
fix: format
    cargo clippy --fix --allow-dirty --allow-staged
# Clean build artifacts
clean:
    cargo clean
# Show project size statistics
stats:
    @echo "Source Lines of Code:"
    @find src -name "*.rs" | xargs wc -l | sort -n

# <FILE>Justfile</FILE> - <DESC>Project workflows for wsl-clip (Rust/WSL)</DESC>
# <VERS>END OF VERSION: 1.0.0 - 2025-11-25T18:05:53Z</VERS>
