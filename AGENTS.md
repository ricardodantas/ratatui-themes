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
├── lib.rs              # Public exports, crate docs (incl. the rustdoc theme table)
├── palette.rs          # ThemePalette struct and is_light()/is_dark()
├── theme.rs            # ThemeName enum (every theme's colors live here), Theme wrapper, tests
└── widgets/            # Only with the `widgets` feature
    ├── mod.rs
    └── theme_picker.rs # ThemePicker widget
examples/
└── gallery.rs          # Interactive gallery; needs `--features widgets`
```

## Key Types

| Type | Description |
|------|-------------|
| `ThemeName` | `#[non_exhaustive]` enum of all themes; `palette()`, `all()`, `next()`/`prev()`, `display_name()`, `slug()`, `FromStr` |
| `Theme` | Wrapper with a public `name: ThemeName` field; `palette()`, `is_light()`/`is_dark()`, in-place `next()`/`prev()` |
| `ThemePalette` | The ten semantic colors of a theme; light/dark comes from `bg` brightness |
| `ThemePicker` | Widget that previews a theme's palette (`widgets` feature) |
| `Color`, `Style` | Re-exported from `ratatui::style` |

## Common Tasks

### Adding a New Theme

Everything lives in `src/theme.rs`. Append the theme at the end; the order of `ThemeName::all()` is the cycling order.

1. Add the variant, with a `///` doc comment (`missing_docs` is on), to the `ThemeName` enum.
2. Add it to `ThemeName::all()`.
3. Add arms to `display_name()` (human name, e.g. `"MSX"`) and `slug()` (kebab-case, e.g. `"msx"`).
4. Add the palette arm in `ThemeName::palette()`: all ten `ThemePalette` fields as `Color::Rgb`, with a source-link comment. `is_light()` reads only `Color::Rgb` backgrounds, so use RGB.
5. Add a `FromStr` arm. Input is lowercased and stripped to alphanumerics before matching, so arms use that form (`"midnightcommander" | "mc"`).
6. Bump the theme count in the `all()` doctest and `test_theme_count`. Point the `next()` wrap-around doctest (`last`) and the `prev()` doctest (`first.prev()`) at the new last theme.
7. Docs: add a row to the README theme table and the `src/lib.rs` rustdoc table, add the variant to the `SKILL.md` theme list, update the theme count in README, `src/lib.rs`, `SKILL.md` and `llms.txt`, and add a `CHANGELOG.md` entry under `[Unreleased]`.

`next()`/`prev()` need no change: they walk `all()`.

**Casing rule.** Serde names come from `#[serde(rename_all = "kebab-case")]` on the enum, which puts a hyphen before every capital letter. Write acronyms in PascalCase (`Msx`, not `MSX`, which would serialize as `"m-s-x"`), so the serde name equals `slug()`. The test `serde_name_should_match_slug_and_round_trip_for_every_theme` goes red when they differ; run it with `cargo test --features serde`.

### Testing

CI (`.github/workflows/ci.yml`) runs these; run them before pushing:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo clippy --features serde -- -D warnings
cargo test
cargo test --features serde
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

Add `cargo clippy --all-targets --all-features -- -D warnings` and `cargo test --all-features` to cover the `widgets` feature, the gallery example and the `ThemePicker` doctest.

Known failures on `main`, unrelated to theme work: `cargo test --no-default-features` fails one doctest in `src/lib.rs` (it imports serde), and `cargo doc` without `--all-features` warns about the `ThemePicker` link.

## Dependencies and Features

- `ratatui` — `Color`/`Style` types and the widget traits
- `serde` — optional, behind the `serde` feature, which is **on by default**
- `widgets` — feature that enables `widgets::ThemePicker`; no extra dependencies
- `serde_json` — dev-dependency for the serde tests

## Release Process

Releases run through the `Release` workflow (`.github/workflows/release.yml`), started by hand:

```bash
gh workflow run release.yml -f bump=minor   # none | patch | minor | major
```

It bumps `Cargo.toml`/`Cargo.lock` with `cargo set-version` and commits `chore: bump version to X.Y.Z` to `main`. It then runs `cargo test` and `cargo test --features serde`, tags and pushes `vX.Y.Z`, creates the GitHub release with generated notes, and publishes to crates.io. `bump=none` releases the version already in `Cargo.toml`.

The version in `Cargo.toml` changes only through this workflow, which leaves `CHANGELOG.md` alone. Before triggering a release, open a PR that moves the `[Unreleased]` entries under a dated `## [X.Y.Z] - YYYY-MM-DD` heading, leaves an empty `## [Unreleased]` above it, and updates the compare links at the bottom.

## Code Style

- Use `rustfmt` for formatting
- Document all public items
- Prefer semantic color names over raw hex values in documentation

## Related Projects

- [Feedo](https://github.com/ricardodantas/feedo) — RSS reader using this crate
- [Hazelnut](https://github.com/ricardodantas/hazelnut) — File organizer using this crate
