---
name: ratatui-themes
description: >
  Use when building or styling a Rust terminal UI (TUI) application with ratatui
  and you need to apply a consistent color theme, switch between themes at runtime,
  or use semantic status colors (error, warning, success). Trigger terms: ratatui
  theme, TUI color scheme, Dracula terminal, Nord ratatui, Catppuccin TUI,
  Tokyo Night terminal, dark theme rust, ratatui palette, theme cycling, serde
  theme preference.
---

# ratatui-themes Skill

A Rust library providing 15+ pre-defined color themes for terminal UI applications
built with `ratatui`. Includes Dracula, Nord, Catppuccin, Tokyo Night, Gruvbox,
and more — with semantic status colors and optional serde support for persistence.

## When to Use

- You are building a `ratatui`-based TUI and want a polished color scheme out of the box
- You need semantic colors (`error`, `warning`, `success`, `info`) for status indicators
- You want users to cycle through themes at runtime
- You want to persist the user's theme choice across sessions with serde

## Quick Start

### 1. Add the dependency

```toml
# Cargo.toml
[dependencies]
ratatui-themes = "0.1"
```

### 2. Apply a theme to a widget

```rust
use ratatui_themes::{Theme, ThemeName};
use ratatui::style::Style;

let theme = Theme::new(ThemeName::Dracula);
let palette = theme.palette();

let style = Style::default()
    .fg(palette.fg)
    .bg(palette.bg);

// Use semantic colors for status indicators
let error_style = Style::default().fg(palette.error);
let success_style = Style::default().fg(palette.success);
```

### 3. Cycle through themes at runtime

```rust
use ratatui_themes::ThemeName;

let mut current = ThemeName::Dracula;

// Move to next/previous theme (wraps around)
current = current.next();
current = current.prev();
```

### 4. Persist theme preference with serde

```toml
# Cargo.toml
[dependencies]
ratatui-themes = { version = "0.1", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
```

```rust
use ratatui_themes::ThemeName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AppConfig {
    theme: ThemeName,
}

// Save / load via serde_json, toml, etc.
let config = AppConfig { theme: ThemeName::Nord };
let json = serde_json::to_string(&config)?;
```

## Available Themes

| Category | Themes |
|----------|--------|
| Dark | Dracula, OneDarkPro, Nord, CatppuccinMocha, GruvboxDark, TokyoNight, SolarizedDark, MonokaiPro, RosePine, Kanagawa, Everforest, Cyberpunk |
| Light | CatppuccinLatte, GruvboxLight, SolarizedLight |

## Palette Reference

Each theme exposes a `Palette` with these fields:

| Field | Purpose |
|-------|---------|
| `bg`, `fg` | Base background and foreground |
| `muted` | Subdued text / secondary information |
| `accent`, `secondary` | Highlight / call-to-action colors |
| `selection` | Selection / highlight background |
| `error`, `warning`, `success`, `info` | Semantic status colors |

## Links

- **Crate**: https://crates.io/crates/ratatui-themes
- **Docs**: https://docs.rs/ratatui-themes
- **Source**: https://github.com/ricardodantas/ratatui-themes
