// <FILE>src/text_processor.rs</FILE> - <DESC>Streaming text processor with security sanitization</DESC>
// <VERS>VERSION: 2.2.0 - 2025-11-25T17:17:02Z</VERS>
// <WCTX>Implemented Safe Text whitelist (strip \b, \a, etc., keep \t) in default mode.</WCTX>
// <CLOG>Added char filtering logic to write_line; added security test case.</CLOG>

use crate::debug_logger::create_logger;
use anyhow::{Context, Result};
use chrono::Utc;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
pub struct TextOptions {
    pub no_header: bool,
    pub strip_ansi: bool,
    pub use_markdown: bool,
    pub use_crlf: bool,
}
/// Streams processed content directly to the writer (clipboard pipe)
/// This avoids loading entire files into memory.
pub fn process_input<W: Write>(
    files: Option<Vec<PathBuf>>,
    opts: &TextOptions,
    writer: &mut W,
) -> Result<()> {
    let log = create_logger("text_processor");
    // Pre-compile regex if needed
    let ansi_regex = if opts.strip_ansi {
        Some(Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})*)?[m|K]").unwrap())
    } else {
        None
    };
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    // Helper to write a line with transforms
    let write_line = |w: &mut W, line: &str| -> Result<()> {
        let mut processed = line.to_string();
        // 1. Strip ANSI Sequences first (so we don't leave dangling brackets)
        if let Some(re) = &ansi_regex {
            processed = re.replace_all(&processed, "").to_string();
            // 2. Security Sanitization (Pastejacking prevention)
            // Strip all control characters except Tab (\t).
            // Note: Newlines are handled structurally by the loop, so they aren't in 'line'.
            // This removes \b (backspace), \r (stray carriage return), \a (bell), etc.
            processed = processed
                .chars()
                .filter(|&c| !c.is_control() || c == '\t')
                .collect();
        }
        if opts.use_crlf {
            // Normalize to LF then CRLF?
            // Simple approach: BufRead::lines() strips the newline.
            // We just append \r\n.
            w.write_all(processed.as_bytes())?;
            w.write_all(b"\r\n")?;
        } else {
            w.write_all(processed.as_bytes())?;
            w.write_all(b"\n")?;
        }
        Ok(())
    };
    if let Some(mut file_list) = files {
        if file_list.is_empty() {
            // Should have been caught by caller, but handle gracefully
            return Ok(());
        }
        file_list.sort();
        log.debug(&format!("Processing {} files (streaming)", file_list.len()));
        let total_files = file_list.len();
        let mut processed_list = Vec::new();
        for path in file_list {
            if !path.exists() || !path.is_file() {
                log.warn(&format!("Skipped invalid file: {:?}", path));
                continue;
            }
            processed_list.push(path.to_string_lossy().to_string());
            // Header
            if !opts.no_header {
                let header = format!("# FILE: {} READ: {}\n", path.display(), timestamp);
                if opts.use_crlf {
                    writer.write_all(header.replace("\n", "\r\n").as_bytes())?;
                } else {
                    writer.write_all(header.as_bytes())?;
                }
            }
            // Markdown Start
            if opts.use_markdown {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let md_block = format!("```{}\n", ext);
                if opts.use_crlf {
                    writer.write_all(md_block.replace("\n", "\r\n").as_bytes())?;
                } else {
                    writer.write_all(md_block.as_bytes())?;
                }
            }
            // Stream Content
            let file =
                File::open(&path).with_context(|| format!("Failed to read file: {:?}", path))?;
            let reader = BufReader::new(file);
            for line_res in reader.lines() {
                let line = line_res.context("Failed to read line")?;
                write_line(writer, &line)?;
            }
            // Markdown End
            if opts.use_markdown {
                let md_end = "```\n";
                if opts.use_crlf {
                    writer.write_all(md_end.replace("\n", "\r\n").as_bytes())?;
                } else {
                    writer.write_all(md_end.as_bytes())?;
                }
            }
            // Spacer between files
            if !opts.no_header {
                if opts.use_crlf {
                    writer.write_all(b"\r\n")?;
                } else {
                    writer.write_all(b"\n")?;
                }
            }
        }
        if !opts.no_header && total_files > 1 {
            let footer = format!("# End of FILES. SENT: {}\n", processed_list.join(" "));
            if opts.use_crlf {
                writer.write_all(footer.replace("\n", "\r\n").as_bytes())?;
            } else {
                writer.write_all(footer.as_bytes())?;
            }
        }
    } else {
        // Stdin Mode
        log.debug("Reading from Stdin (Streaming)");
        if atty::is(atty::Stream::Stdin) {
            anyhow::bail!("No input provided. Pipe data or specify files.");
        }
        let stdin = io::stdin();
        let reader = stdin.lock();
        for line_res in reader.lines() {
            let line = line_res.context("Failed to read line from stdin")?;
            write_line(writer, &line)?;
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn test_process_streaming() -> Result<()> {
        let mut file1 = NamedTempFile::new()?;
        writeln!(file1, "Line 1")?;
        let path1 = file1.path().to_path_buf();
        let opts = TextOptions {
            no_header: false,
            strip_ansi: false,
            use_markdown: false,
            use_crlf: false,
        };
        let mut buffer = Vec::new();
        process_input(Some(vec![path1]), &opts, &mut buffer)?;
        let output = String::from_utf8(buffer)?;
        assert!(output.contains("# FILE:"));
        assert!(output.contains("Line 1"));
        Ok(())
    }
    #[test]
    fn test_safe_text_sanitization() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        // Contains: ANSI color, Backspace (\x08), Bell (\x07), Tab (\t), and Text
        writeln!(file, "\x1B[31mRed\x1B[0m\x08\x08Good\tText\x07")?;
        let path = file.path().to_path_buf();
        let opts = TextOptions {
            no_header: true,
            strip_ansi: true, // Should enable sanitization
            use_markdown: false,
            use_crlf: false,
        };
        let mut buffer = Vec::new();
        process_input(Some(vec![path]), &opts, &mut buffer)?;
        let output = String::from_utf8(buffer)?;
        // Expected:
        // ANSI removed ("Red" remains)
        // \x08 removed (Backspaces gone)
        // \x07 removed (Bell gone)
        // \t kept
        // "RedGood\tText\n"
        assert_eq!(output, "RedGood\tText\n");
        Ok(())
    }
}

// <FILE>src/text_processor.rs</FILE> - <DESC>Streaming text processor with security sanitization</DESC>
// <VERS>END OF VERSION: 2.2.0 - 2025-11-25T17:17:02Z</VERS>
