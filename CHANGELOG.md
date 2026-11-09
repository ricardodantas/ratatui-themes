# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Midnight Commander theme (`ThemeName::MidnightCommander`, slug `midnight-commander`)
- MSX theme (`ThemeName::Msx`, slug `msx`)
- Test that every theme's serde name matches `slug()` and round-trips

## [0.1.0] - 2026-02-05

### Added
- Initial release
- 15 color themes: Dracula, One Dark Pro, Nord, Catppuccin Mocha, Catppuccin Latte, Gruvbox Dark, Gruvbox Light, Tokyo Night, Solarized Dark, Solarized Light, Monokai Pro, Rosé Pine, Kanagawa, Everforest, Cyberpunk
- `ThemePalette` struct with semantic colors (accent, secondary, bg, fg, muted, selection, error, warning, success, info)
- `ThemeName` enum with `next()`/`prev()` for theme cycling
- `Theme` wrapper with `is_light()`/`is_dark()` detection
- Optional serde support for configuration files
- Re-export of `ratatui::style::Color` for convenience
