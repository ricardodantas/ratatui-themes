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

Everything happens in `src/theme.rs`. Add the theme at the **end** of the list: the order of `ThemeName::all()` is the order `next()`/`prev()` cycle through.

### Step 1: Add the Theme Variant

Add a documented variant to the `ThemeName` enum, then add it to `ThemeName::all()`:

```rust
pub enum ThemeName {
    // ... existing themes ...

    /// My Theme — one-line description.
    ///
    /// Source: <https://example.com/my-theme>
    MyTheme,
}
```

Don't add a `#[serde(rename)]`. The enum has `#[serde(rename_all = "kebab-case")]`, which turns `MyTheme` into `"my-theme"`. Kebab-case puts a hyphen before **every** capital letter, so write acronyms in PascalCase: `Msx` serializes as `"msx"`, but `MSX` would become `"m-s-x"`.

### Step 2: Add the Names

Add arms to these methods on `ThemeName`:

- `display_name()` — the human name, e.g. `"My Theme"`
- `slug()` — the kebab-case name, e.g. `"my-theme"`. It must match the serde name.
- `FromStr` — input is lowercased and stripped to letters and digits before matching, so write the arm as `"mytheme"`, plus any short aliases.

### Step 3: Define the Colors

Add a match arm in `ThemeName::palette()` (in `src/theme.rs`):

```rust
// My Theme: https://example.com/my-theme
Self::MyTheme => ThemePalette {
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

Light/dark detection is automatic: `is_light()` checks the brightness of `bg`, which must be a `Color::Rgb`.

### Step 4: Update the Tests

- Bump the theme count in `test_theme_count` and in the `ThemeName::all()` doctest.
- Point the wrap-around doctests of `next()` and `prev()` at your theme, which is now the last one.
- `next()` and `prev()` themselves need no change.

`serde_name_should_match_slug_and_round_trip_for_every_theme` checks that each theme's serde name equals `slug()` and parses back. Run `cargo test --features serde` to catch casing mistakes.

### Step 5: Update the Documentation

1. Add a row to the theme table in `README.md`
2. Add a row to the theme table in the crate docs at the top of `src/lib.rs`
3. Add the variant to the theme list in `SKILL.md`
4. Update the theme count in `README.md`, `src/lib.rs`, `SKILL.md` and `llms.txt`
5. Add an entry to `CHANGELOG.md` under `[Unreleased]`

## Code Guidelines

Run what CI runs before opening a PR:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo clippy --features serde -- -D warnings
cargo test
cargo test --features serde
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

- **Document** all public items with doc comments
- **Use RGB colors** (`Color::Rgb`) rather than terminal color names, so colors look the same everywhere and light/dark detection works

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

Leave the version in `Cargo.toml` alone. Maintainers release through the `Release` GitHub Actions workflow, which bumps the version, tags, and publishes to crates.io.

## Theme Selection Criteria

When proposing new themes, consider:

- **Popularity**: Is it a well-known theme used in editors/terminals?
- **Quality**: Does it have well-defined, consistent colors?
- **Uniqueness**: Does it offer something different from existing themes?

## Questions?

Open an issue for discussion before starting work on major changes.
