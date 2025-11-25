// <FILE>src/debug_logger.rs</FILE> - <DESC>Fixed Clippy lints (Default impl, useless vec)</DESC>
// <VERS>VERSION: 1.3.0 - 2025-11-24T15:14:44Z</VERS>
// <WCTX>Added Default impl for DebugLogger; optimized array allocation.</WCTX>
// <CLOG>Fixed clippy::new-without-default, clippy::useless-vec.</CLOG>

use crate::debug_config::{module_registry, LogLevel, ModuleConfig};
use colored::Colorize;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: String,
    pub namespace: String,
    pub message: String,
}
pub struct DebugLogger {
    enabled_namespaces: Arc<Mutex<HashSet<String>>>,
    log_history: Arc<Mutex<Vec<LogEntry>>>,
    max_history_size: usize,
    module_registry: HashMap<String, ModuleConfig>,
}
impl Default for DebugLogger {
    fn default() -> Self {
        Self::new()
    }
}
impl DebugLogger {
    pub fn new() -> Self {
        let module_registry = module_registry();
        let mut enabled = HashSet::new();
        for (namespace, config) in &module_registry {
            if config.level != LogLevel::Off {
                enabled.insert(namespace.clone());
            }
        }
        DebugLogger {
            enabled_namespaces: Arc::new(Mutex::new(enabled)),
            log_history: Arc::new(Mutex::new(Vec::new())),
            max_history_size: 1000,
            module_registry,
        }
    }
    pub fn enable(&self, namespace: &str) {
        let mut enabled = self.enabled_namespaces.lock().unwrap();
        enabled.insert(namespace.to_string());
    }
    pub fn is_enabled(&self, namespace: &str) -> bool {
        let enabled = self.enabled_namespaces.lock().unwrap();
        if let Some(config) = self.module_registry.get(namespace) {
            if config.level == LogLevel::Off
                && !enabled.contains(namespace)
                && !enabled.contains("*")
            {
                return false;
            }
        }
        if enabled.contains("*") || enabled.contains(namespace) {
            return true;
        }
        for e in enabled.iter() {
            if e.ends_with('*') {
                let prefix = &e[..e.len() - 1];
                if namespace.starts_with(prefix) {
                    return true;
                }
            }
        }
        false
    }
    pub fn should_log(&self, namespace: &str, level: &LogLevel) -> bool {
        if !self.is_enabled(namespace) {
            return false;
        }
        if let Some(config) = self.module_registry.get(namespace) {
            let levels = [
                LogLevel::Debug,
                LogLevel::Info,
                LogLevel::Warn,
                LogLevel::Error,
            ];
            let config_idx = levels.iter().position(|l| l == &config.level);
            let level_idx = levels.iter().position(|l| l == level);
            if let (Some(c), Some(l)) = (config_idx, level_idx) {
                return l >= c;
            }
        }
        true
    }
    fn log(&self, level: LogLevel, namespace: &str, message: &str) {
        if !self.should_log(namespace, &level) {
            return;
        }
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let entry = LogEntry {
            timestamp,
            level: format!("{:?}", level),
            namespace: namespace.to_string(),
            message: message.to_string(),
        };
        let mut history = self.log_history.lock().unwrap();
        history.push(entry);
        if history.len() > self.max_history_size {
            let len = history.len();
            *history = history.split_off(len - self.max_history_size);
        }
        let level_str = match level {
            LogLevel::Debug => "DEBUG".purple().bold(),
            LogLevel::Info => "INFO".blue().bold(),
            LogLevel::Warn => "WARN".yellow().bold(),
            LogLevel::Error => "ERROR".red().bold(),
            _ => "".normal(),
        };
        eprintln!(
            "[{}] {} {} {}",
            chrono::Local::now().format("%H:%M:%S").to_string().dimmed(),
            level_str,
            namespace.bold(),
            message
        );
    }
    pub fn create_logger(&self, namespace: &str) -> Logger {
        Logger {
            namespace: namespace.to_string(),
        }
    }
    pub fn export(&self, filename: Option<&str>) {
        let history = self.log_history.lock().unwrap();
        let filename = filename.unwrap_or("wsl-clip-debug.json");
        if let Ok(json) = serde_json::to_string_pretty(&*history) {
            if let Ok(mut file) = File::create(filename) {
                let _ = file.write_all(json.as_bytes());
                eprintln!("[EXPORT] Exported debug logs to {}", filename);
            }
        }
    }
}
pub struct Logger {
    namespace: String,
}
impl Logger {
    pub fn debug(&self, message: &str) {
        if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_ref() {
            logger.log(LogLevel::Debug, &self.namespace, message);
        }
    }
    pub fn info(&self, message: &str) {
        if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_ref() {
            logger.log(LogLevel::Info, &self.namespace, message);
        }
    }
    pub fn warn(&self, message: &str) {
        if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_ref() {
            logger.log(LogLevel::Warn, &self.namespace, message);
        }
    }
    pub fn error(&self, message: &str) {
        if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_ref() {
            logger.log(LogLevel::Error, &self.namespace, message);
        }
    }
}
lazy_static! {
    pub static ref GLOBAL_LOGGER: Arc<Mutex<Option<Arc<DebugLogger>>>> =
        Arc::new(Mutex::new(Some(Arc::new(DebugLogger::new()))));
}
pub fn create_logger(namespace: &str) -> Logger {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    if let Some(ref l) = *logger {
        l.create_logger(namespace)
    } else {
        panic!("Logger not initialized");
    }
}
pub fn enable_all() {
    if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_ref() {
        logger.enable("*");
    }
}

// <FILE>src/debug_logger.rs</FILE> - <DESC>Fixed Clippy lints (Default impl, useless vec)</DESC>
// <VERS>END OF VERSION: 1.3.0 - 2025-11-24T15:14:44Z</VERS>
