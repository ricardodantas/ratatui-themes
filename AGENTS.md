# AGENTS.md — AI Agent Context

This file provides context for AI coding agents working with the `ratatui-themes` crate.

## Project Overview

**ratatui-themes** is a Rust library providing reusable color themes for terminal UI applications built with [ratatui](https://ratatui.rs).

### Purpose
- Eliminate theme duplication across ratatui-based projects
- Provide consistent, semantic color palettes (accent, error, warning, success, etc.)
- Support easy theme switching in TUI applications

### Architecture

```
src/
├── lib.rs        # Public API exports and crate-level documentation
├── palette.rs    # ThemePalette struct with all color definitions
└── theme.rs      # ThemeName enum and Theme wrapper
```

## Key Types

| Type | Description |
|------|-------------|
| `ThemeName` | Enum of all available themes (Dracula, Nord, etc.) |
| `Theme` | Wrapper providing palette access and metadata |
| `ThemePalette` | Struct containing all theme colors |

## Common Tasks

### Adding a New Theme

1. Add variant to `ThemeName` enum in `src/theme.rs`
2. Add serde rename attribute: `#[serde(rename = "theme-name")]`
3. Implement colors in `ThemePalette::from_name()` match arm
4. Add to `ThemeName::all()` array
5. Update `next()`/`prev()` cycling logic
6. Add to README.md theme table

### Theme Color Structure

Each theme must define these colors:
- `accent` — Primary highlight color
- `secondary` — Secondary accent
- `bg` — Background
- `fg` — Foreground text
- `muted` — Dimmed/comment text
- `selection` — Selection background
- `error` — Red/error state
- `warning` — Yellow/warning state  
- `success` — Green/success state
- `info` — Blue/info state

### Testing

```bash
cargo test              # Run all tests
cargo test --features serde  # Test with serde
cargo doc --open        # Generate and view docs
```

## Dependencies

- `ratatui` — Core TUI framework (Color type)
- `serde` — Optional, for config serialization

## Release Process

1. Update version in `Cargo.toml`
2. Commit: `git commit -m "chore: bump version to X.Y.Z"`
3. Tag: `git tag vX.Y.Z`
4. Push: `git push origin main --tags`
5. GitHub Actions publishes to crates.io automatically

## Code Style

- Use `rustfmt` for formatting
- Document all public items
- Keep themes alphabetically ordered where practical
- Prefer semantic color names over raw hex values in documentation

## Related Projects

- [Feedo](https://github.com/ricardodantas/feedo) — RSS reader using this crate
- [Hazelnut](https://github.com/ricardodantas/hazelnut) — File organizer using this crate
