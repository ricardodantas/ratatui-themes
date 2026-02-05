# ratatui-themes

A collection of popular color themes for [ratatui](https://ratatui.rs) terminal UI applications.

## Features

- 🎨 **15+ popular themes** — Dracula, Nord, Catppuccin, Gruvbox, Tokyo Night, and more
- 🔄 **Easy theme cycling** — Built-in next/prev for theme switchers
- 📦 **Serde support** — Optional serialization for config files
- 🌗 **Light/dark detection** — Know if a theme is light or dark

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ratatui-themes = "0.1"
```

## Quick Start

```rust
use ratatui_themes::{Theme, ThemeName, ThemePalette};
use ratatui::style::Style;

// Get a theme
let theme = Theme::new(ThemeName::Dracula);

// Access the color palette
let palette = theme.palette();
let style = Style::default()
    .fg(palette.fg)
    .bg(palette.bg);

// Use semantic colors
let error_style = Style::default().fg(palette.error);
let success_style = Style::default().fg(palette.success);
```

## Available Themes

| Theme | Type | Description |
|-------|------|-------------|
| Dracula | Dark | Dark purple aesthetic |
| One Dark Pro | Dark | Atom's iconic dark theme |
| Nord | Dark | Arctic, bluish color palette |
| Catppuccin Mocha | Dark | Warm pastel dark theme |
| Catppuccin Latte | Light | Warm pastel light theme |
| Gruvbox Dark | Dark | Retro groove colors |
| Gruvbox Light | Light | Retro groove, light variant |
| Tokyo Night | Dark | Futuristic dark blue |
| Solarized Dark | Dark | Precision colors, dark |
| Solarized Light | Light | Precision colors, light |
| Monokai Pro | Dark | Classic syntax highlighting |
| Rosé Pine | Dark | Natural pine vibes |
| Kanagawa | Dark | Inspired by Hokusai |
| Everforest | Dark | Comfortable green forest |
| Cyberpunk | Dark | Neon-soaked futuristic |

## Color Palette

Each theme provides these semantic colors:

```rust
pub struct ThemePalette {
    pub accent: Color,     // Primary accent (highlights)
    pub secondary: Color,  // Secondary accent
    pub bg: Color,         // Background
    pub fg: Color,         // Foreground text
    pub muted: Color,      // Dimmed text
    pub selection: Color,  // Selection background
    pub error: Color,      // Error/red
    pub warning: Color,    // Warning/yellow
    pub success: Color,    // Success/green
    pub info: Color,       // Info/blue
}
```

## Theme Cycling

```rust
use ratatui_themes::ThemeName;

let mut theme = ThemeName::Dracula;

// Cycle forward
theme = theme.next(); // -> OneDarkPro

// Cycle backward  
theme = theme.prev(); // -> Dracula
```

## Configuration with Serde

```rust
use ratatui_themes::{Theme, ThemeName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    theme: ThemeName,
}

// Serializes as: { "theme": "tokyo-night" }
```

## License

MIT
