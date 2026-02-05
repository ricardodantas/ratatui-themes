//! Color palette definitions for themes.

use ratatui::style::Color;

/// Color palette for a theme.
///
/// Each theme defines these semantic colors that can be used
/// consistently across your UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemePalette {
    /// Primary accent color (highlights, active elements).
    pub accent: Color,
    /// Secondary accent color.
    pub secondary: Color,
    /// Background color.
    pub bg: Color,
    /// Foreground/text color.
    pub fg: Color,
    /// Muted/dimmed text color.
    pub muted: Color,
    /// Selection/highlight background.
    pub selection: Color,
    /// Error/red color.
    pub error: Color,
    /// Warning/yellow color.
    pub warning: Color,
    /// Success/green color.
    pub success: Color,
    /// Info/blue color.
    pub info: Color,
}

impl ThemePalette {
    /// Check if this is a light theme (light background).
    #[must_use]
    pub fn is_light(&self) -> bool {
        if let Color::Rgb(r, g, b) = self.bg {
            // Calculate perceived brightness
            let brightness = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
            brightness > 127
        } else {
            false
        }
    }

    /// Check if this is a dark theme.
    #[must_use]
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }
}
