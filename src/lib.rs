//! # ratatui-themes
//!
//! A collection of popular color themes for [ratatui](https://ratatui.rs) terminal UI applications.
//!
//! ## Features
//!
//! - 15+ popular themes (Dracula, Nord, Catppuccin, Gruvbox, Tokyo Night, etc.)
//! - Consistent color palette structure
//! - Optional serde support for configuration files
//! - Style helpers for common UI patterns
//!
//! ## Quick Start
//!
//! ```rust
//! use ratatui_themes::{Theme, ThemeName};
//! use ratatui::style::Style;
//!
//! // Get a theme by name
//! let theme = Theme::new(ThemeName::Dracula);
//!
//! // Access colors
//! let accent = theme.palette().accent;
//! let bg = theme.palette().bg;
//!
//! // Use style helpers
//! let title_style = Style::default().fg(theme.palette().accent);
//! ```
//!
//! ## Available Themes
//!
//! - Dracula
//! - One Dark Pro
//! - Nord
//! - Catppuccin Mocha
//! - Catppuccin Latte
//! - Gruvbox Dark
//! - Gruvbox Light
//! - Tokyo Night
//! - Solarized Dark
//! - Solarized Light
//! - Monokai Pro
//! - Rosé Pine
//! - Kanagawa
//! - Everforest
//! - Cyberpunk

mod palette;
mod theme;

pub use palette::ThemePalette;
pub use theme::{Theme, ThemeName};

/// Re-export ratatui's Color type for convenience.
pub use ratatui::style::Color;
