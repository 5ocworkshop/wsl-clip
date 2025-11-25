// <FILE>wsl-clip/src/paths.rs</FILE> - <DESC>Instrumented with debug logging</DESC>
// <VERS>VERSION: 1.2.0 - 2025-11-24T14:52:13Z</VERS>
// <WCTX>Added logging to wslpath conversion.</WCTX>
// <CLOG>Added logging.</CLOG>

use crate::debug_logger::create_logger;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
pub fn to_windows_path(path: &Path) -> Result<String> {
    let log = create_logger("paths");
    let abs_path =
        dunce::canonicalize(path).with_context(|| format!("Failed to resolve path: {:?}", path))?;
    log.debug(&format!("Canonicalized path: {:?}", abs_path));
    let output = Command::new("wslpath")
        .arg("-w")
        .arg(&abs_path)
        .output()
        .with_context(|| "Failed to execute wslpath")?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        log.error(&format!("wslpath failed: {}", err.trim()));
        anyhow::bail!("wslpath failed: {}", err.trim());
    }
    let win_path = String::from_utf8(output.stdout)
        .with_context(|| "wslpath output returned invalid UTF-8")?;
    let trimmed = win_path.trim().to_string();
    log.debug(&format!("Windows path: {}", trimmed));
    Ok(trimmed)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_wslpath_resolution() {
        let p = PathBuf::from("/bin/sh");
        if p.exists() {
            let res = to_windows_path(&p);
            assert!(res.is_ok());
        }
    }
}

// <FILE>wsl-clip/src/paths.rs</FILE> - <DESC>Instrumented with debug logging</DESC>
// <VERS>END OF VERSION: 1.2.0 - 2025-11-24T14:52:13Z</VERS>
