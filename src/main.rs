// <FILE>src/main.rs</FILE> - <DESC>Integrated streaming and security fixes</DESC>
// <VERS>VERSION: 2.3.0 - 2025-11-25T17:09:34Z</VERS>
// <WCTX>Wired main to use start_text_stream and process_input(writer).</WCTX>
// <CLOG>Updated Text Mode handling to use streaming pipeline.</CLOG>

pub mod classifier;
pub mod clipboard;
pub mod debug_config;
pub mod debug_logger;
pub mod paths;
pub mod text_processor;
use anyhow::Result;
use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    Parser, Subcommand,
};
use classifier::ClipboardStrategy;
use clipboard::ClipboardMode;
use debug_logger::create_logger;
use std::path::PathBuf;
use text_processor::TextOptions;
fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .usage(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Yellow.on_default())
}
#[derive(Parser)]
#[command(
    name = "wsl-clip",
    version,
    about = "The Ultimate WSL2 Clipboard Utility",
    long_about = "Delightfully smart clipboard integration for WSL2.\n\nAuto-detects content types, supports images, binaries, and text piping.\n\nBy default, ANSI color codes are stripped from text to ensure clean pasting.",
    styles = get_styles(),
    override_usage = "wsl-clip [OPTIONS] <COMMAND> | [FILES]...",
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}
{all-args}{after-help}
",
    after_help = "\
EXAMPLES:
  wsl-clip image.png       # Auto-detects Image mode
  wsl-clip doc.pdf         # Auto-detects File object
  wsl-clip src/*.rs        # Copies text (ANSI stripped by default)
  ls --color | wsl-clip    # Pipes clean text (colors removed)
  ls --color | wsl-clip --no-strip  # Pipes raw text (colors preserved)
"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Files to copy (Text Mode or Smart Mode). If empty, reads from Stdin.
    #[arg()]
    files: Option<Vec<PathBuf>>,
    /// Suppress file headers in Text Mode
    #[arg(short = 'n', long, global = true)]
    no_header: bool,
    /// Disable ANSI color stripping (Default: stripping is ON)
    #[arg(long, global = true)]
    no_strip: bool,
    /// Convert Linux line endings (LF) to Windows (CRLF)
    #[arg(long, global = true)]
    crlf: bool,
    /// Wrap content in Markdown code blocks
    #[arg(long, global = true)]
    code: bool,
    /// Enable debug logging
    #[arg(long, global = true)]
    debug: bool,
}
#[derive(Subcommand)]
enum Commands {
    /// Force Image Mode (copy pixels)
    Img { file: PathBuf },
    /// Force File Object Mode (copy as attachment)
    File { files: Vec<PathBuf> },
    /// Copy the Windows path string
    Path { file: PathBuf },
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.debug {
        debug_logger::enable_all();
    }
    let log = create_logger("main");
    log.debug("wsl-clip started");
    match cli.command {
        Some(Commands::Img { file }) => {
            log.debug(&format!("Command: Img, File: {:?}", file));
            let win_path = paths::to_windows_path(&file)?;
            clipboard::set_complex(&[win_path], ClipboardMode::Image)?;
            println!("[OK] Copied Image to Clipboard");
        }
        Some(Commands::File { files }) => {
            log.debug(&format!("Command: File, Files: {} count", files.len()));
            let mut win_paths = Vec::new();
            for f in files {
                win_paths.push(paths::to_windows_path(&f)?);
            }
            clipboard::set_complex(&win_paths, ClipboardMode::File)?;
            println!(
                "[OK] Copied {} File Object(s) to Clipboard",
                win_paths.len()
            );
        }
        Some(Commands::Path { file }) => {
            log.debug(&format!("Command: Path, File: {:?}", file));
            let win_path = paths::to_windows_path(&file)?;
            clipboard::set_text_content(&win_path)?;
            println!("[OK] Copied Path to Clipboard");
        }
        None => {
            // Smart Mode Dispatch
            if let Some(files) = &cli.files {
                if !files.is_empty() {
                    let mut img_count = 0;
                    let mut file_count = 0;
                    let mut text_count = 0;
                    for f in files {
                        match classifier::inspect(f) {
                            Ok(ClipboardStrategy::Image) => img_count += 1,
                            Ok(ClipboardStrategy::File) => file_count += 1,
                            Ok(ClipboardStrategy::Text) => text_count += 1,
                            Err(e) => {
                                log.warn(&format!("Classification failed for {:?}: {}", f, e));
                                anyhow::bail!("Failed to read file: {:?}", f);
                            }
                        }
                    }
                    // 1. Mixed Content Check
                    let categories_present =
                        (img_count > 0) as u8 + (file_count > 0) as u8 + (text_count > 0) as u8;
                    if categories_present > 1 {
                        anyhow::bail!(
                            "Mixed content detected! ({} images, {} files/assets, {} text). \
                            Please run separate commands for each type.",
                            img_count,
                            file_count,
                            text_count
                        );
                    }
                    // 2. Image Mode
                    if img_count > 0 {
                        if files.len() == 1 {
                            log.debug("Smart Mode: Single Image");
                            let win_path = paths::to_windows_path(&files[0])?;
                            clipboard::set_complex(&[win_path], ClipboardMode::Image)?;
                            println!("[OK] Copied Image to Clipboard");
                            return Ok(());
                        } else {
                            log.debug("Smart Mode: Multiple Images -> File Mode");
                            let mut win_paths = Vec::new();
                            for f in files {
                                win_paths.push(paths::to_windows_path(f)?);
                            }
                            clipboard::set_complex(&win_paths, ClipboardMode::File)?;
                            println!("[OK] Copied {} Images as Files", win_paths.len());
                            return Ok(());
                        }
                    }
                    // 3. File/Asset Mode
                    if file_count > 0 {
                        log.debug("Smart Mode: Files/Assets detected");
                        let mut win_paths = Vec::new();
                        for f in files {
                            win_paths.push(paths::to_windows_path(f)?);
                        }
                        clipboard::set_complex(&win_paths, ClipboardMode::File)?;
                        println!("[OK] Copied {} Files", win_paths.len());
                        return Ok(());
                    }
                    log.debug("Smart Mode: Text Mode");
                }
            }
            // 4. Default / Text Mode (Streaming)
            log.debug("Command: Default (Text Mode)");
            let opts = TextOptions {
                no_header: cli.no_header,
                strip_ansi: !cli.no_strip,
                use_markdown: cli.code,
                use_crlf: cli.crlf,
            };
            // Start the clip.exe process first to get the pipe
            let mut stream = clipboard::start_text_stream()?;
            if let Some(writer) = &mut stream.stdin {
                // Stream content directly to the pipe
                text_processor::process_input(cli.files, &opts, writer)?;
            } else {
                anyhow::bail!("Failed to acquire stdin for clip.exe");
            }
            // Wait for clip.exe to finish
            stream.wait()?;
            let mut msg = "[OK] Copied Text".to_string();
            if cli.no_strip {
                msg.push_str(" (Raw ANSI)");
            }
            if opts.use_crlf {
                msg.push_str(" (CRLF)");
            }
            println!("{}", msg);
        }
    }
    Ok(())
}

// <FILE>src/main.rs</FILE> - <DESC>Integrated streaming and security fixes</DESC>
// <VERS>END OF VERSION: 2.3.0 - 2025-11-25T17:09:34Z</VERS>
