# Contributing to ratatui-themes

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ratatui-themes.git
   cd ratatui-themes
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b add-my-theme
   ```

## Adding a New Theme

### Step 1: Add the Theme Variant

In `src/theme.rs`, add your theme to the `ThemeName` enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ThemeName {
    // ... existing themes ...
    
    #[cfg_attr(feature = "serde", serde(rename = "my-theme"))]
    MyTheme,
}
```

### Step 2: Define the Colors

In `src/palette.rs`, add a match arm in `ThemePalette::from_name()`:

```rust
ThemeName::MyTheme => Self {
    accent: Color::Rgb(r, g, b),      // Primary accent
    secondary: Color::Rgb(r, g, b),   // Secondary accent
    bg: Color::Rgb(r, g, b),          // Background
    fg: Color::Rgb(r, g, b),          // Foreground text
    muted: Color::Rgb(r, g, b),       // Dimmed text
    selection: Color::Rgb(r, g, b),   // Selection background
    error: Color::Rgb(r, g, b),       // Error/red
    warning: Color::Rgb(r, g, b),     // Warning/yellow
    success: Color::Rgb(r, g, b),     // Success/green
    info: Color::Rgb(r, g, b),        // Info/blue
},
```

### Step 3: Update Theme Lists

1. Add to `ThemeName::all()` array
2. Update `next()` and `prev()` cycling methods
3. Update `is_light()` if your theme is a light theme

### Step 4: Update Documentation

1. Add to the theme table in `README.md`
2. Add to the theme list in `SKILL.md`
3. Add to `CHANGELOG.md` under `[Unreleased]`

## Code Guidelines

- **Format code** with `cargo fmt`
- **Run clippy** with `cargo clippy`
- **Run tests** with `cargo test` and `cargo test --features serde`
- **Document** all public items with doc comments
- **Use RGB colors** for consistency (avoid terminal color names)

## Commit Messages

Use clear, descriptive commit messages:

- `feat: add Ayu theme`
- `fix: correct Nord accent color`
- `docs: improve theme table formatting`
- `chore: update dependencies`

## Pull Request Process

1. Ensure all tests pass
2. Update documentation as needed
3. Add a changelog entry
4. Submit a PR with a clear description of changes

## Theme Selection Criteria

When proposing new themes, consider:

- **Popularity**: Is it a well-known theme used in editors/terminals?
- **Quality**: Does it have well-defined, consistent colors?
- **Uniqueness**: Does it offer something different from existing themes?

## Questions?

Open an issue for discussion before starting work on major changes.
