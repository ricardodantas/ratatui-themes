use crate::{Theme, ThemeName};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// A widget for displaying and selecting themes.
///
/// Shows the current theme's color palette with visual examples
/// of each semantic color.
///
/// # Example
///
/// ```rust
/// use ratatui_themes::{ThemeName, ThemePicker};
/// use ratatui::Frame;
///
/// let picker = ThemePicker::new(ThemeName::TokyoNight)
///     .title("Theme Gallery");
///
/// // In your render function:
/// // frame.render_widget(picker, area);
/// ```
#[derive(Debug, Clone)]
pub struct ThemePicker {
    theme: Theme,
    title: Option<String>,
    instructions: Option<String>,
}

impl ThemePicker {
    /// Create a new theme picker with the given theme.
    #[must_use]
    pub const fn new(theme: ThemeName) -> Self {
        Self {
            theme: Theme::new(theme),
            title: None,
            instructions: None,
        }
    }

    /// Set the title for the widget.
    ///
    /// Pass an empty string to remove the title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        let title = title.into();
        self.title = if title.is_empty() { None } else { Some(title) };
        self
    }

    /// Set instructions text shown at the bottom.
    ///
    /// Pass an empty string to remove instructions.
    #[must_use]
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        let instructions = instructions.into();
        self.instructions = if instructions.is_empty() {
            None
        } else {
            Some(instructions)
        };
        self
    }
}

impl Widget for ThemePicker {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let palette = self.theme.palette();

        let mut block = Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.muted))
            .title_style(Style::default().fg(palette.accent).bold());

        if let Some(title) = self.title {
            block = block.title(Line::from(title).centered());
        }

        if let Some(instructions) = self.instructions {
            block = block.title_bottom(Line::from(instructions).centered());
        }

        let text = vec![
            Line::from(self.theme.name.display_name()).style(Style::default().fg(palette.fg)),
            Line::from("  - accent").style(Style::default().fg(palette.accent)),
            Line::from("  - secondary").style(Style::default().fg(palette.secondary)),
            Line::from("  - bg (on fg)").style(Style::default().fg(palette.bg).bg(palette.fg)),
            Line::from("  - fg (on bg)").style(Style::default().fg(palette.fg).bg(palette.bg)),
            Line::from("  - muted").style(Style::default().fg(palette.muted)),
            Line::from("  - selection (as bg)").style(Style::default().bg(palette.selection)),
            Line::from("  - error").style(Style::default().fg(palette.error)),
            Line::from("  - warning").style(Style::default().fg(palette.warning)),
            Line::from("  - success").style(Style::default().fg(palette.success)),
            Line::from("  - info").style(Style::default().fg(palette.info)),
        ];

        Paragraph::new(text).block(block).render(area, buf);
    }
}
