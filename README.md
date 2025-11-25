<!-- <FILE>README.md</FILE> - <DESC>Documentation for v2.0 Smart Mode (ASCII Safe)</DESC> -->
<!-- <VERS>VERSION: 2.2.0 - 2025-11-25T17:57:58Z</VERS> -->
<!-- <WCTX>Added Path mode to introduction.</WCTX> -->
<!-- <CLOG>Added Paths bullet point to feature overview.</CLOG> -->

# wsl-clip
**The Ultimate Clipboard Bridge for WSL2.**
`wsl-clip` is a high-performance, smart clipboard utility that bridges the gap between Linux (WSL2) and Windows. It automatically detects content types to provide the most useful clipboard format:
*   **Text** -> Copies clean, safe text (pasted into Notepad/Code).
*   **Images** -> Copies the bitmap (pasted into Slack/Paint).
*   **Files** -> Copies the file object (pasted into Explorer/Outlook).
*   **Paths** -> Copies the translated Windows path (pasted into File Dialogs).
## Features
*   **Smart Mode:** Just run `wsl-clip <file>`. The tool detects:
    *   **Magic Bytes:** Recognizes PNG, JPG, PDF, ZIP, etc. regardless of extension.
    *   **Assets:** Forces "File Object" mode for 3D models (DXF, STL) and Archives.
    *   **Text:** Defaults to text for source code and logs.
*   **Secure by Default:**
    *   **Pastejacking Protection:** Strips invisible control characters (backspace, bell) that can hide malicious commands.
    *   **ANSI Stripping:** Automatically removes terminal color codes for clean pasting.
*   **Streaming Architecture:** Uses O(1) memory. Pipe gigabytes of logs (`cat huge.log | wsl-clip`) without crashing your RAM.
*   **Multi-File Support:** `wsl-clip *.pdf` copies multiple files as a single drop list.
## Installation
### From Source
```bash
# 1. Clone
git clone https://github.com/yourusername/wsl-clip
cd wsl-clip
# 2. Build & Install
# Requires Rust toolchain
just build
# Or: cargo install --path .
```
*Ensure `~/.cargo/bin` is in your `$PATH`.*
## Usage
### 1. Smart Mode (Recommended)
Let `wsl-clip` decide the best format.
```bash
# Copy a screenshot (Bitmap)
wsl-clip screenshot.png
# Copy a PDF attachment (File Object)
wsl-clip invoice.pdf
# Copy source code (Text)
wsl-clip src/main.rs
# Copy multiple files (File Object Drop List)
wsl-clip *.png
```
### 2. Text Piping
Behaves like a "Smart Cat". Reads from stdin automatically.
```bash
# Copy directory listing (ANSI colors stripped automatically)
ls -la --color | wsl-clip
# Copy with Markdown wrapping
wsl-clip src/lib.rs --code
```
### 3. Explicit Modes (Overrides)
Force a specific behavior if Smart Mode guesses wrong.
```bash
# Force copy as a file object (e.g., to attach a .rs file to an email)
wsl-clip file src/main.rs
# Force copy as an image
wsl-clip img logo.png
# Copy the Windows path string (e.g., "\\wsl.localhost\...")
wsl-clip path document.pdf
```
## Configuration Flags
| Flag | Description |
| :--- | :--- |
| `--no-strip` | **Raw Mode.** Preserves ANSI colors and control characters. |
| `--crlf` | Convert Linux (`\n`) line endings to Windows (`\r\n`). |
| `--code` | Wrap text content in Markdown \`\`\` blocks. |
| `--no-header` | Suppress the file name header when copying multiple text files. |
| `--debug` | Enable verbose logging to stderr. |
## Security & Architecture
`wsl-clip` is built with a security-first architecture:
1.  **Injection Proof:** All filenames are passed to PowerShell via parameterized arguments (`$args`), avoiding shell injection vulnerabilities.
2.  **Memory Safe:** File content is streamed line-by-line. It never loads full files into memory.
3.  **Sanitization:**
    *   **Default:** Strips ANSI codes + Unsafe Control Chars (Backspace, Bell, Escape).
    *   **Preserves:** Tab (`\t`), Newline (`\n`), and Carriage Return (`\r`).
## License
MIT

<!-- <FILE>README.md</FILE> - <DESC>Documentation for v2.0 Smart Mode (ASCII Safe)</DESC> -->
<!-- <VERS>END OF VERSION: 2.2.0 - 2025-11-25T17:57:58Z</VERS> -->
