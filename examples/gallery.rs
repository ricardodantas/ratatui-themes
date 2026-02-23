use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    DefaultTerminal, Frame,
};
use ratatui_themes::{Theme, ThemeName, ThemePicker};
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
        let picker = ThemePicker::new(self.theme.name)
            .title("Ratatui Theme Gallery")
            .instructions("Previous <Left> Next <Right> Quit <Q>");

        frame.render_widget(picker, frame.area());
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
