pub(crate) mod app;
mod board_view;
pub mod palette;
mod sidebar;

use std::io;

use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;

use crate::board::Board;
use crate::piece::Piece;

use app::App;

fn render_frame(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(60), Constraint::Length(25)])
        .split(frame.area());

    board_view::render(frame, app, chunks[0]);
    sidebar::render(frame, app, chunks[1]);
}

pub fn run(pieces: Vec<Piece>) -> io::Result<Board> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(pieces);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        loop {
            let visible_rows = terminal.size().map(|s| s.height as usize).unwrap_or(24);
            app.update_scroll(visible_rows.saturating_sub(2));

            terminal.draw(|frame| render_frame(frame, &app)).unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                app.handle_key(key.code);
                if app.done {
                    break;
                }
            }
        }
        app.board.clone()
    }));

    let _ = disable_raw_mode();
    let _ = io::stdout().execute(LeaveAlternateScreen);

    match result {
        Ok(board) => Ok(board),
        Err(e) => std::panic::resume_unwind(e),
    }
}
