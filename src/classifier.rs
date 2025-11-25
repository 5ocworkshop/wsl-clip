// <FILE>src/classifier.rs</FILE> - <DESC>Consolidated file classification logic</DESC>
// <VERS>VERSION: 1.4.0 - 2025-11-25T16:55:29Z</VERS>
// <WCTX>Moved ASSET_EXTS and override logic here. Added high-level inspection.</WCTX>
// <CLOG>Added inspect() function; merged extension overrides.</CLOG>

use crate::debug_logger::create_logger;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
#[derive(Debug, PartialEq, Eq)]
pub enum ClipboardStrategy {
    Image, // Bitmaps
    File,  // File Objects (Binary, Assets, Archives)
    Text,  // Raw Text
}
// Extensions that should ALWAYS be treated as File Objects (Assets), even if they are technically text.
const ASSET_EXTS: &[&str] = &[
    "dxf", "obj", "stl", "ply", "gcode", "svg", "eps", "ai", "psd", "pdf", "zip", "7z", "tar",
    "gz", "rar", "iso", "dll", "bin", "exe", "jar", "class",
];
fn is_asset_extension(p: &Path) -> bool {
    if let Some(ext) = p.extension() {
        if let Some(s) = ext.to_str() {
            return ASSET_EXTS.contains(&s.to_lowercase().as_str());
        }
    }
    false
}
/// Determines the best clipboard strategy for a given file.
/// Checks extension overrides first (fast), then falls back to magic bytes (robust).
pub fn inspect(path: &Path) -> Result<ClipboardStrategy> {
    let log = create_logger("classifier");
    // 1. Extension Override (Fast Path)
    if is_asset_extension(path) {
        log.debug(&format!(
            "Extension override detected (Asset/Binary): {:?}",
            path
        ));
        return Ok(ClipboardStrategy::File);
    }
    // 2. Open file for Magic Byte detection
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file for classification: {:?}", path))?;
    let mut buffer = [0u8; 262];
    let n = file.read(&mut buffer).unwrap_or(0);
    let buffer = &buffer[..n];
    // 3. Check Image
    if infer::is_image(buffer) {
        log.debug(&format!("Detected IMAGE signature: {:?}", path));
        return Ok(ClipboardStrategy::Image);
    }
    // 4. Check Binary Signatures
    if infer::is_archive(buffer) || infer::is_app(buffer) || infer::doc::is_doc(buffer) {
        log.debug(&format!("Detected BINARY signature: {:?}", path));
        return Ok(ClipboardStrategy::File);
    }
    // 5. Heuristic: Null bytes
    if buffer.contains(&0) {
        log.debug(&format!(
            "Detected NULL bytes (Binary heuristic): {:?}",
            path
        ));
        return Ok(ClipboardStrategy::File);
    }
    // 6. Default
    log.debug(&format!("Classified as TEXT: {:?}", path));
    Ok(ClipboardStrategy::Text)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    #[test]
    fn test_asset_extension() {
        assert_eq!(
            inspect(&PathBuf::from("model.dxf")).unwrap(),
            ClipboardStrategy::File
        );
        assert_eq!(
            inspect(&PathBuf::from("image.SVG")).unwrap(),
            ClipboardStrategy::File
        );
    }
    #[test]
    fn test_classify_text() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        write!(file, "Hello World")?;
        assert_eq!(inspect(file.path())?, ClipboardStrategy::Text);
        Ok(())
    }
    #[test]
    fn test_classify_binary_nulls() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        file.write_all(&[0x00, 0x01, 0x02])?;
        assert_eq!(inspect(file.path())?, ClipboardStrategy::File);
        Ok(())
    }
}

// <FILE>src/classifier.rs</FILE> - <DESC>Consolidated file classification logic</DESC>
// <VERS>END OF VERSION: 1.4.0 - 2025-11-25T16:55:29Z</VERS>
