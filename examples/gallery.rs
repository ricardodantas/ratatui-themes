use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use ratatui_themes::{Theme, ThemeName};
use std::io;

struct App {
    theme: Theme,
    exit: bool,
}

impl App {
    const fn new() -> Self {
        Self {
            theme: Theme::new(ThemeName::TokyoNight),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => self.exit = true,
            KeyCode::Left => self.theme.prev(),
            KeyCode::Right => self.theme.next(),
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let palette = self.theme.palette();

        // Create styled block
        let block = Block::bordered()
            .title(Line::from("Ratatui Theme Gallery").centered())
            .title_bottom(Line::from("Previous <Left> Next <Right> Quit <Q>").centered())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.muted))
            .title_style(Style::default().fg(palette.accent).bold());

        // Create paragraph with theme colors
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

        let text = Paragraph::new(text).block(block);

        frame.render_widget(text, frame.area());
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
