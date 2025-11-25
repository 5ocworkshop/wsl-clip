// <FILE>src/debug_config.rs</FILE> - <DESC>Module registry configuration</DESC>
// <VERS>VERSION: 1.2.0 - 2025-11-25T16:34:29Z</VERS>
// <WCTX>Registered classifier module.</WCTX>
// <CLOG>Added classifier entry.</CLOG>

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Off,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub level: LogLevel,
    pub description: String,
}
pub fn module_registry() -> HashMap<String, ModuleConfig> {
    let mut registry = HashMap::new();
    // CLI Entry Point
    registry.insert(
        "main".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "CLI argument parsing and dispatch".to_string(),
        },
    );
    // Path Conversion
    registry.insert(
        "paths".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "WSL to Windows path conversion logic".to_string(),
        },
    );
    // Clipboard Backend
    registry.insert(
        "clipboard".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "PowerShell and clip.exe interaction".to_string(),
        },
    );
    // Text Processing
    registry.insert(
        "text_processor".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "File reading, headers, and ANSI stripping".to_string(),
        },
    );
    // File Classifier
    registry.insert(
        "classifier".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Magic-byte based file type detection".to_string(),
        },
    );
    registry
}

// <FILE>src/debug_config.rs</FILE> - <DESC>Module registry configuration</DESC>
// <VERS>END OF VERSION: 1.2.0 - 2025-11-25T16:34:29Z</VERS>
