<div align="center">

# 🎨 ratatui-themes

**Beautiful, consistent color themes for [ratatui](https://ratatui.rs) terminal UI applications.**

[![Crates.io](https://img.shields.io/crates/v/ratatui-themes.svg)](https://crates.io/crates/ratatui-themes)
[![Documentation](https://docs.rs/ratatui-themes/badge.svg)](https://docs.rs/ratatui-themes)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

[Features](#features) • [Installation](#installation) • [Quick Start](#quick-start) • [Themes](#available-themes) • [API](#api-reference) • [Examples](#examples)

</div>

---

## ✨ Features

- 🎨 **15+ Popular Themes** — Dracula, Nord, Catppuccin, Gruvbox, Tokyo Night, and more
- 🔄 **Easy Theme Cycling** — Built-in `next()`/`prev()` methods for theme switchers
- 📦 **Serde Support** — Optional serialization for saving theme preferences
- 🌗 **Light/Dark Detection** — Programmatically determine if a theme is light or dark
- 🎯 **Semantic Colors** — Consistent `error`, `warning`, `success`, `info` across all themes
- ⚡ **Zero Dependencies** — Only requires `ratatui` (and optionally `serde`)

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ratatui-themes = "0.1"
```

With serde support for configuration files:

```toml
[dependencies]
ratatui-themes = { version = "0.1", features = ["serde"] }
```

## 🚀 Quick Start

```rust
use ratatui_themes::{Theme, ThemeName};
use ratatui::style::Style;

// Create a theme
let theme = Theme::new(ThemeName::Dracula);

// Access the color palette
let palette = theme.palette();

// Apply colors to styles
let title_style = Style::default()
    .fg(palette.accent)
    .bg(palette.bg);

let error_style = Style::default()
    .fg(palette.error);

let muted_text = Style::default()
    .fg(palette.muted);
```

## 🎨 Available Themes

| Theme | Type | Preview | Description |
|-------|------|---------|-------------|
| **Dracula** | 🌙 Dark | `#bd93f9` | Iconic dark purple aesthetic |
| **One Dark Pro** | 🌙 Dark | `#61afef` | Atom's beloved dark theme |
| **Nord** | 🌙 Dark | `#88c0d0` | Arctic, bluish color palette |
| **Catppuccin Mocha** | 🌙 Dark | `#cba6f7` | Warm, soothing pastel dark |
| **Catppuccin Latte** | ☀️ Light | `#8839ef` | Warm pastel light variant |
| **Gruvbox Dark** | 🌙 Dark | `#fabd2f` | Retro groove colors |
| **Gruvbox Light** | ☀️ Light | `#d79921` | Retro groove, light mode |
| **Tokyo Night** | 🌙 Dark | `#7aa2f7` | Futuristic Tokyo cityscape |
| **Solarized Dark** | 🌙 Dark | `#268bd2` | Precision-engineered colors |
| **Solarized Light** | ☀️ Light | `#268bd2` | Solarized for bright rooms |
| **Monokai Pro** | 🌙 Dark | `#ffd866` | Classic syntax colors |
| **Rosé Pine** | 🌙 Dark | `#c4a7e7` | Natural, muted elegance |
| **Kanagawa** | 🌙 Dark | `#7e9cd8` | Inspired by Hokusai's art |
| **Everforest** | 🌙 Dark | `#a7c080` | Comfortable forest green |
| **Cyberpunk** | 🌙 Dark | `#ff00ff` | Neon-soaked futuristic |

## 🔧 API Reference

### ThemePalette

Every theme provides a consistent color palette with semantic meaning:

```rust
pub struct ThemePalette {
    // Core colors
    pub accent: Color,      // Primary accent for highlights, selections
    pub secondary: Color,   // Secondary accent for less prominent elements
    pub bg: Color,          // Main background color
    pub fg: Color,          // Primary foreground/text color
    pub muted: Color,       // Dimmed text, comments, placeholders
    pub selection: Color,   // Selection/highlight background
    
    // Semantic colors
    pub error: Color,       // Errors, deletions, critical alerts
    pub warning: Color,     // Warnings, cautions, pending states
    pub success: Color,     // Success, additions, confirmations
    pub info: Color,        // Information, links, neutral highlights
}
```

### Theme Cycling

Build theme switchers with ease:

```rust
use ratatui_themes::ThemeName;

let mut current = ThemeName::Dracula;

// Cycle forward through all themes
current = current.next();  // -> OneDarkPro

// Cycle backward
current = current.prev();  // -> Dracula

// Get all available themes
let all_themes = ThemeName::all();
```

### Light/Dark Detection

```rust
use ratatui_themes::{Theme, ThemeName};

let theme = Theme::new(ThemeName::CatppuccinLatte);

if theme.is_light() {
    // Adjust UI for light theme
}

if theme.is_dark() {
    // Adjust UI for dark theme
}
```

### Serde Integration

Save and load theme preferences (requires `serde` feature):

```rust
use ratatui_themes::ThemeName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AppConfig {
    theme: ThemeName,
    // ... other settings
}

// Serializes as: { "theme": "tokyo-night" }
// Theme names use kebab-case for human readability
```

## 📖 Examples

### Complete TUI App Example

```rust
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use ratatui_themes::{Theme, ThemeName};

struct App {
    theme: Theme,
}

impl App {
    fn new() -> Self {
        Self {
            theme: Theme::new(ThemeName::TokyoNight),
        }
    }

    fn render(&self, frame: &mut Frame) {
        let palette = self.theme.palette();
        
        // Create styled block
        let block = Block::default()
            .title(" My App ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.muted))
            .title_style(Style::default().fg(palette.accent).bold());

        // Create paragraph with theme colors
        let text = Paragraph::new("Welcome!")
            .style(Style::default().fg(palette.fg).bg(palette.bg))
            .block(block);

        frame.render_widget(text, frame.area());
    }

    fn cycle_theme(&mut self) {
        let next_name = self.theme.name().next();
        self.theme = Theme::new(next_name);
    }
}
```

### Status Bar with Semantic Colors

```rust
use ratatui_themes::{Theme, ThemeName};

fn render_status_bar(theme: &Theme, status: &str) -> Style {
    let palette = theme.palette();
    
    match status {
        "error" => Style::default().fg(palette.error).bold(),
        "warning" => Style::default().fg(palette.warning),
        "success" => Style::default().fg(palette.success),
        _ => Style::default().fg(palette.info),
    }
}
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

- 🐛 **Report bugs** — Open an issue describing the problem
- 💡 **Suggest themes** — Request popular themes to be added
- 🔧 **Submit PRs** — Add new themes or improve existing ones

### Adding a New Theme

1. Add the theme variant to `ThemeName` enum in `src/theme.rs`
2. Implement the palette in `ThemePalette::from_name()`
3. Add to `ThemeName::all()` list
4. Update the README theme table
5. Add tests

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

<div align="center">

**Made with ❤️ for the [ratatui](https://ratatui.rs) community**

</div>
