use eyre::Result;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::events;
use crate::key::Key;
use crate::ui::{ui, Tabss};
pub fn run() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = events::Events::new();
    let mut tabs = Tabss::new();

    loop {
        terminal.draw(|f| ui(f, &mut tabs))?;
        if let events::Event::Input(event) = events.next()? {
            match event {
                //KeyCode::Char('c') | KeyCode::Char('q') => {break;}
                Key::Ctrl('c') | Key::Char('q') => {
                    break;
                }
                Key::Right => {
                    tabs.next();
                }
                Key::Left => {
                    tabs.previous();
                }
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
