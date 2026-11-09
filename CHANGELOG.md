# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- README, crate docs and `SKILL.md`: install snippets now say 0.4, theme counts say 17, and the README `cycle_theme` example compiles

## [0.4.0] - 2026-11-09

### Added
- MSX theme (`ThemeName::Msx`, serde/slug name `msx`), using the MSX (TMS9918) palette
- Test that every theme's serde name matches `slug()` and round-trips

### Changed
- Crate docs and `SKILL.md` now list Midnight Commander (added in 0.3.0) and MSX

## [0.3.0] - 2026-08-02

### Added
- Midnight Commander theme (`ThemeName::MidnightCommander`, slug `midnight-commander`, also parses from `"mc"`)

### Changed
- Requires ratatui 0.30.2 or later

## [0.2.0] - 2026-02-23

### Added
- `widgets` feature with a `ThemePicker` widget that previews a theme's palette
- `gallery` example: `cargo run --example gallery --features widgets`

## [0.1.8] - 2026-02-06

### Changed
- Formatting and doc-comment fixes only; no API changes

## [0.1.7] - 2026-02-06

### Added
- `FromStr` for `ThemeName`: accepts kebab-case, display names, PascalCase and short aliases (`"tokyo-night"`, `"Tokyo Night"`, `"mocha"`)
- `ThemeName::slug()`: the kebab-case name used by serde

Versions 0.1.5 and 0.1.6 were never published to crates.io; their changes first shipped in 0.1.7.

## [0.1.4] - 2026-02-05

### Changed
- Requires ratatui 0.30; ratatui 0.29 is no longer supported

## [0.1.3] - 2026-02-05

### Changed
- Accepts ratatui 0.29 or 0.30

## [0.1.2] - 2026-02-05

### Added
- Re-export of `ratatui::style::Style`
- `Default` for `ThemePalette` (the Dracula palette) and `Display` for `Theme`
- Rustdoc documentation with examples for all public items
- Declared minimum supported Rust version: 1.74

### Changed
- `ThemeName` is now `#[non_exhaustive]`, so a `match` on it needs a wildcard arm

## [0.1.1] - 2026-02-05

### Changed
- Internal cleanup only; no API changes

## [0.1.0] - 2026-02-05

### Added
- Initial release
- 15 color themes: Dracula, One Dark Pro, Nord, Catppuccin Mocha, Catppuccin Latte, Gruvbox Dark, Gruvbox Light, Tokyo Night, Solarized Dark, Solarized Light, Monokai Pro, Rosé Pine, Kanagawa, Everforest, Cyberpunk
- `ThemePalette` struct with semantic colors (accent, secondary, bg, fg, muted, selection, error, warning, success, info)
- `ThemeName` enum with `next()`/`prev()` for theme cycling
- `Theme` wrapper with `is_light()`/`is_dark()` detection
- Optional serde support for configuration files
- Re-export of `ratatui::style::Color` for convenience

[Unreleased]: https://github.com/ricardodantas/ratatui-themes/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/ricardodantas/ratatui-themes/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ricardodantas/ratatui-themes/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ricardodantas/ratatui-themes/compare/v0.1.8...v0.2.0
[0.1.8]: https://github.com/ricardodantas/ratatui-themes/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/ricardodantas/ratatui-themes/compare/v0.1.4...v0.1.7
[0.1.4]: https://github.com/ricardodantas/ratatui-themes/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/ricardodantas/ratatui-themes/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/ricardodantas/ratatui-themes/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ricardodantas/ratatui-themes/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ricardodantas/ratatui-themes/releases/tag/v0.1.0
