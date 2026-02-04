//! Hazelnut - Terminal-based automated file organizer
//!
//! A Hazel-like file organization tool with a TUI interface.

pub mod app;
pub mod config;
pub mod ipc;
pub mod rules;
pub mod theme;
pub mod watcher;

pub use config::Config;
pub use rules::{Action, Condition, Rule, RuleEngine};
pub use theme::Theme;
pub use watcher::Watcher;

/// Expand ~ in a path to the user's home directory
pub fn expand_path(path: &std::path::Path) -> std::path::PathBuf {
    let path_str = path.to_string_lossy();

    if path_str.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path_str[2..]);
        }
    } else if path_str == "~" {
        if let Some(home) = dirs::home_dir() {
            return home;
        }
    }

    path.to_path_buf()
}
