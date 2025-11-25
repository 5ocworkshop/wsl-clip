<!-- <FILE>BUILD.md</FILE> - <DESC>Developer build instructions</DESC> -->
<!-- <VERS>VERSION: 1.0.0 - 2025-11-25T17:55:10Z</VERS> -->
<!-- <WCTX>Created standalone build guide.</WCTX> -->
<!-- <CLOG>Initial creation.</CLOG> -->

# Build Instructions
This document describes how to build, test, and install `wsl-clip` from source.
## Prerequisites
1.  **WSL2 Environment**: This tool is designed specifically for Linux running on Windows Subsystem for Linux (WSL2). It requires access to:
    *   `powershell.exe` (available in the Windows PATH).
    *   `clip.exe` (available in the Windows PATH).
    *   `wslpath` (standard in WSL distributions).
2.  **Rust Toolchain**: Ensure you have `cargo` and `rustc` installed.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
3.  **Just (Optional)**: We use `just` as a command runner.
    ```bash
    cargo install just
    ```
## Building
### Using Just (Recommended)
If you have `just` installed, you can use the pre-defined recipes:
```bash
# Run all quality checks (Format, Lint, Test)
just check
# Build the release binary
just build
```
### Using Cargo Directly
If you prefer standard Cargo commands:
**Debug Build:**
```bash
cargo build
# Artifact: target/debug/wsl-clip
```
**Release Build (Optimized):**
```bash
cargo build --release
# Artifact: target/release/wsl-clip
```
## Installation
To install the binary into your Cargo bin directory (usually `~/.cargo/bin`), run:
```bash
cargo install --path .
```
Ensure `~/.cargo/bin` is in your shell's `$PATH`.
## Testing
To run the test suite:
```bash
# Using Just
just test
# Using Cargo
cargo test
```
## Development Notes
*   **Smart Mode Logic**: The core logic resides in `src/classifier.rs` and `src/main.rs`.
*   **Security**: The project uses strict parameterization for PowerShell calls. Do not introduce string interpolation for filenames in `src/clipboard.rs`.
*   **Streaming**: Large inputs are streamed via `src/text_processor.rs`. Avoid reading full files into memory strings.

<!-- <FILE>BUILD.md</FILE> - <DESC>Developer build instructions</DESC> -->
<!-- <VERS>END OF VERSION: 1.0.0 - 2025-11-25T17:55:10Z</VERS> -->
