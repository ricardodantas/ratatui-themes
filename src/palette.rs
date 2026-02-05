//! Color palette definitions for themes.
//!
//! This module contains the [`ThemePalette`] struct which holds all the semantic
//! colors for a theme. Each theme provides the same set of colors with consistent
//! meanings, making it easy to build UIs that look good across all themes.

use ratatui::style::Color;

/// A semantic color palette for a theme.
///
/// Each theme defines these colors with consistent meanings, allowing you to
/// build UIs that automatically adapt to any theme. The colors are organized
/// into two categories:
///
/// ## Core Colors
/// - [`accent`](Self::accent) — Primary accent for highlights, active elements
/// - [`secondary`](Self::secondary) — Secondary accent for less prominent elements
/// - [`bg`](Self::bg) — Main background color
/// - [`fg`](Self::fg) — Primary foreground/text color
/// - [`muted`](Self::muted) — Dimmed text, comments, placeholders
/// - [`selection`](Self::selection) — Selection/highlight background
///
/// ## Semantic Colors
/// - [`error`](Self::error) — Errors, deletions, critical alerts (typically red)
/// - [`warning`](Self::warning) — Warnings, cautions, pending states (typically yellow/orange)
/// - [`success`](Self::success) — Success, additions, confirmations (typically green)
/// - [`info`](Self::info) — Information, links, neutral highlights (typically blue/cyan)
///
/// # Example
///
/// ```rust
/// use ratatui_themes::{ThemeName, ThemePalette};
/// use ratatui::style::Style;
///
/// let palette = ThemeName::Dracula.palette();
///
/// // Create styles using semantic colors
/// let error_style = Style::default().fg(palette.error);
/// let success_style = Style::default().fg(palette.success);
/// let muted_style = Style::default().fg(palette.muted);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemePalette {
    /// Primary accent color for highlights and active elements.
    ///
    /// Use this for:
    /// - Active/selected items
    /// - Important UI elements
    /// - Links and interactive elements
    /// - Progress indicators
    pub accent: Color,

    /// Secondary accent color for less prominent highlights.
    ///
    /// Use this for:
    /// - Secondary highlights
    /// - Hover states
    /// - Less important accents
    pub secondary: Color,

    /// Main background color.
    ///
    /// Use this for:
    /// - Application background
    /// - Panel backgrounds
    /// - Modal overlays (with transparency)
    pub bg: Color,

    /// Primary foreground/text color.
    ///
    /// Use this for:
    /// - Main text content
    /// - Icons
    /// - Primary UI elements
    pub fg: Color,

    /// Muted/dimmed text color.
    ///
    /// Use this for:
    /// - Comments and annotations
    /// - Placeholder text
    /// - Disabled elements
    /// - Secondary information
    /// - Timestamps and metadata
    pub muted: Color,

    /// Selection/highlight background color.
    ///
    /// Use this for:
    /// - Text selection background
    /// - Highlighted rows in lists
    /// - Focused elements
    pub selection: Color,

    /// Error/red color for critical states.
    ///
    /// Use this for:
    /// - Error messages
    /// - Validation failures
    /// - Deletion confirmations
    /// - Critical alerts
    pub error: Color,

    /// Warning/yellow color for caution states.
    ///
    /// Use this for:
    /// - Warning messages
    /// - Deprecation notices
    /// - Pending operations
    /// - Items needing attention
    pub warning: Color,

    /// Success/green color for positive states.
    ///
    /// Use this for:
    /// - Success messages
    /// - Confirmations
    /// - Additions/insertions
    /// - Completed operations
    pub success: Color,

    /// Info/blue color for informational states.
    ///
    /// Use this for:
    /// - Informational messages
    /// - Tips and hints
    /// - External links
    /// - Neutral highlights
    pub info: Color,
}

impl ThemePalette {
    /// Check if this is a light theme based on background brightness.
    ///
    /// Uses the perceived brightness formula (ITU-R BT.601) to determine
    /// if the background color is light enough to be considered a "light theme".
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// assert!(ThemeName::CatppuccinLatte.palette().is_light());
    /// assert!(!ThemeName::Dracula.palette().is_light());
    /// ```
    #[must_use]
    pub fn is_light(&self) -> bool {
        if let Color::Rgb(r, g, b) = self.bg {
            // ITU-R BT.601 perceived brightness formula
            let brightness = (u32::from(r) * 299 + u32::from(g) * 587 + u32::from(b) * 114) / 1000;
            brightness > 127
        } else {
            false
        }
    }

    /// Check if this is a dark theme.
    ///
    /// This is the inverse of [`is_light()`](Self::is_light).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// assert!(ThemeName::Dracula.palette().is_dark());
    /// assert!(!ThemeName::SolarizedLight.palette().is_dark());
    /// ```
    #[must_use]
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }
}

impl Default for ThemePalette {
    /// Returns the default palette (Dracula theme).
    fn default() -> Self {
        crate::ThemeName::default().palette()
    }
}
