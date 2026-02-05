# SKILL.md — AI Skill Definition

## Skill: ratatui-themes

### Description
A Rust library providing pre-defined color themes for terminal UI applications built with `ratatui`. Includes 15+ popular themes like Dracula, Nord, Catppuccin, Tokyo Night, and more.

### Use Cases

1. **Theming TUI Applications**
   - Apply consistent color schemes to ratatui widgets
   - Use semantic colors (error, warning, success) for status indicators

2. **Theme Switching**
   - Implement theme cycling with `ThemeName::next()` and `prev()`
   - Save user theme preferences with serde support

3. **Cross-Project Consistency**
   - Share themes between multiple ratatui-based projects
   - Ensure uniform look and feel across applications

### Quick Integration

```rust
// Add to Cargo.toml:
// ratatui-themes = "0.1"

use ratatui_themes::{Theme, ThemeName};
use ratatui::style::Style;

let theme = Theme::new(ThemeName::Dracula);
let palette = theme.palette();

// Use in widgets
let style = Style::default()
    .fg(palette.fg)
    .bg(palette.bg);
```

### Available Themes

Dark themes: Dracula, OneDarkPro, Nord, CatppuccinMocha, GruvboxDark, TokyoNight, SolarizedDark, MonokaiPro, RosePine, Kanagawa, Everforest, Cyberpunk

Light themes: CatppuccinLatte, GruvboxLight, SolarizedLight

### Palette Colors

Each theme provides:
- `accent`, `secondary` — Highlight colors
- `bg`, `fg`, `muted` — Base colors
- `selection` — Selection background
- `error`, `warning`, `success`, `info` — Semantic status colors

### Links

- **Crate**: https://crates.io/crates/ratatui-themes
- **Docs**: https://docs.rs/ratatui-themes
- **Source**: https://github.com/ricardodantas/ratatui-themes
