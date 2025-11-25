// <FILE>src/clipboard.rs</FILE> - <DESC>Fixed PowerShell argument passing logic</DESC>
// <VERS>VERSION: 1.6.0 - 2025-11-25T17:32:57Z</VERS>
// <WCTX>Wrapped script body in "& { ... }" to correctly capture CLI arguments into $args.</WCTX>
// <CLOG>Fixed PS injection by using call operator block; removed args_placeholder.</CLOG>

use crate::debug_logger::create_logger;
use anyhow::{Context, Result};
use std::process::{Child, ChildStdin, Command, Stdio};
pub enum ClipboardMode {
    Image,
    File,
}
/// Uses PowerShell for complex types (Images, File Objects)
/// SECURITY: Paths are passed as arguments to avoid injection vulnerabilities.
pub fn set_complex(win_paths: &[String], mode: ClipboardMode) -> Result<()> {
    let log = create_logger("clipboard");
    if let ClipboardMode::Image = mode {
        if win_paths.len() != 1 {
            anyhow::bail!("Image mode currently supports exactly one file at a time.");
        }
    }
    // Header executes in the global scope to load assemblies
    let header =
        "Add-Type -AssemblyName System.Windows.Forms; Add-Type -AssemblyName System.Drawing;";
    // Body uses $args, so it must be wrapped in a ScriptBlock "& { ... }"
    // to accept the arguments passed to powershell.exe
    let body = match mode {
        ClipboardMode::Image => {
            // $args[0] is the first argument passed after the command string
            "$img = [System.Drawing.Image]::FromFile($args[0]); [System.Windows.Forms.Clipboard]::SetImage($img);"
        }
        ClipboardMode::File => {
            // Iterate all args
            "$files = New-Object System.Collections.Specialized.StringCollection; $args | ForEach-Object { [void]$files.Add($_) }; [System.Windows.Forms.Clipboard]::SetFileDropList($files);"
        }
    };
    // Construct command: Header; & { Body }
    // The '&' operator executes the following block, passing trailing CLI args into it.
    let script = format!("{} & {{ {} }}", header, body);
    log.debug("Executing PowerShell clipboard script (Parameterized)...");
    let status = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&script)
        // Note: In PowerShell, the first argument after the command string is $args[0].
        // We do NOT need a placeholder like in bash -c.
        .args(win_paths)
        .status()
        .with_context(|| "Failed to execute powershell.exe")?;
    if !status.success() {
        log.error("PowerShell exited with error status");
        anyhow::bail!("PowerShell exited with error status");
    }
    Ok(())
}
pub struct ClipboardStream {
    child: Child,
    pub stdin: Option<ChildStdin>,
}
impl ClipboardStream {
    pub fn wait(mut self) -> Result<()> {
        // Drop stdin to close the pipe so clip.exe knows input is done
        drop(self.stdin.take());
        let status = self.child.wait().context("Failed to wait for clip.exe")?;
        if !status.success() {
            anyhow::bail!("clip.exe exited with error status");
        }
        Ok(())
    }
}
/// Starts a streaming session to clip.exe
pub fn start_text_stream() -> Result<ClipboardStream> {
    let log = create_logger("clipboard");
    log.debug("Spawning clip.exe for streaming...");
    let mut child = Command::new("clip.exe")
        .stdin(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to spawn clip.exe")?;
    let stdin = child.stdin.take();
    Ok(ClipboardStream { child, stdin })
}
/// Legacy helper for one-shot strings (retained for Path mode simplicity)
pub fn set_text_content(content: &str) -> Result<()> {
    use std::io::Write;
    let mut stream = start_text_stream()?;
    if let Some(mut stdin) = stream.stdin.take() {
        stdin.write_all(content.as_bytes())?;
    }
    stream.wait()
}

// <FILE>src/clipboard.rs</FILE> - <DESC>Fixed PowerShell argument passing logic</DESC>
// <VERS>END OF VERSION: 1.6.0 - 2025-11-25T17:32:57Z</VERS>
