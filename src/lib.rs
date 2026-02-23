//! # ratatui-themes
//!
//! A collection of popular color themes for [ratatui](https://ratatui.rs) terminal UI applications.
//!
//! This crate provides a consistent, easy-to-use API for applying beautiful color schemes
//! to your terminal user interfaces. Each theme includes semantic colors for common UI
//! elements like errors, warnings, and highlights.
//!
//! ## Features
//!
//! - **15+ popular themes** — Dracula, Nord, Catppuccin, Gruvbox, Tokyo Night, and more
//! - **Semantic colors** — Consistent `error`, `warning`, `success`, `info` across all themes
//! - **Easy theme cycling** — Built-in `next()`/`prev()` methods for theme switchers
//! - **Light/dark detection** — Programmatically determine if a theme is light or dark
//! - **Serde support** — Optional serialization for saving theme preferences (enabled by default)
//!
//! ## Quick Start
//!
//! ```rust
//! use ratatui_themes::{Theme, ThemeName};
//! use ratatui::style::Style;
//!
//! // Create a theme
//! let theme = Theme::new(ThemeName::Dracula);
//!
//! // Access the color palette
//! let palette = theme.palette();
//!
//! // Apply colors to styles
//! let title_style = Style::default()
//!     .fg(palette.accent)
//!     .bg(palette.bg);
//!
//! let error_style = Style::default().fg(palette.error);
//! let muted_text = Style::default().fg(palette.muted);
//! ```
//!
//! ## Theme Cycling
//!
//! Easily implement theme switching in your application:
//!
//! ```rust
//! use ratatui_themes::ThemeName;
//!
//! let mut current = ThemeName::Dracula;
//!
//! // Cycle forward through all themes
//! current = current.next();  // -> OneDarkPro
//!
//! // Cycle backward
//! current = current.prev();  // -> Dracula
//!
//! // Get all available themes for a selection menu
//! let all_themes = ThemeName::all();
//! ```
//!
//! ## Configuration with Serde
//!
//! Save and load theme preferences (requires the `serde` feature, enabled by default):
//!
//! ```rust
//! use ratatui_themes::ThemeName;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct AppConfig {
//!     theme: ThemeName,
//!     // ... other settings
//! }
//!
//! // Theme names serialize as kebab-case strings:
//! // { "theme": "tokyo-night" }
//! ```
//!
//! ## Available Themes
//!
//! | Theme | Type | Description |
//! |-------|------|-------------|
//! | Dracula | Dark | Iconic dark purple aesthetic |
//! | One Dark Pro | Dark | Atom's beloved dark theme |
//! | Nord | Dark | Arctic, bluish color palette |
//! | Catppuccin Mocha | Dark | Warm, soothing pastel dark |
//! | Catppuccin Latte | Light | Warm pastel light variant |
//! | Gruvbox Dark | Dark | Retro groove colors |
//! | Gruvbox Light | Light | Retro groove, light mode |
//! | Tokyo Night | Dark | Futuristic Tokyo cityscape |
//! | Solarized Dark | Dark | Precision-engineered colors |
//! | Solarized Light | Light | Solarized for bright rooms |
//! | Monokai Pro | Dark | Classic syntax colors |
//! | Rosé Pine | Dark | Natural, muted elegance |
//! | Kanagawa | Dark | Inspired by Hokusai's art |
//! | Everforest | Dark | Comfortable forest green |
//! | Cyberpunk | Dark | Neon-soaked futuristic |
//!
//! ## Feature Flags
//!
//! - **`serde`** (enabled by default) — Enables serialization/deserialization of theme names
//! - **`widgets`** — Provides ready-to-use widgets like [`ThemePicker`]
//!
//! To disable serde support:
//!
//! ```toml
//! [dependencies]
//! ratatui-themes = { version = "0.1", default-features = false }
//! ```

#![doc(html_root_url = "https://docs.rs/ratatui-themes/0.1.1")]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    clippy::all,
    clippy::pedantic
)]
#![allow(clippy::module_name_repetitions)]

mod palette;
mod theme;

pub use palette::ThemePalette;
pub use theme::{Theme, ThemeName};

/// Re-export ratatui's [`Color`] type for convenience.
///
/// This allows you to use `ratatui_themes::Color` without adding ratatui
/// as a direct dependency just for the `Color` type.
pub use ratatui::style::Color;

/// Re-export ratatui's [`Style`] type for convenience.
///
/// This allows you to use `ratatui_themes::Style` without additional imports.
pub use ratatui::style::Style;

/// Enable included widgets.
#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(feature = "widgets")]
pub use widgets::ThemePicker;
