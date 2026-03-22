use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use super::app::App;
use super::palette::PIECE_COLORS;

pub(crate) fn render(frame: &mut Frame, app: &App, area: Rect) {
    let title = if app.confirm_quit {
        " Board — Export with unplaced pieces? [Y/N] "
    } else {
        " Board "
    };
    let board_block = Block::bordered().title(title);
    let inner = board_block.inner(area);
    frame.render_widget(board_block, area);

    let visible_rows = inner.height as usize;
    let visible_cols = inner.width as usize;

    let ghost_valid = app.is_valid_placement();
    let ghost_color = if ghost_valid {
        Color::Green
    } else {
        Color::Red
    };

    for vy in 0..visible_rows {
        let board_y = app.scroll_y + vy;
        let mut spans: Vec<Span> = Vec::new();

        for vx in 0..visible_cols.min(app.board.width) {
            let in_ghost = if let Some(shape) = app.current_shape() {
                let gy = board_y as isize - app.cursor.y as isize;
                if gy >= 0 && (gy as usize) < shape.height() {
                    let row_w = shape.row_widths[gy as usize];
                    vx >= app.cursor.x && vx < app.cursor.x + row_w
                } else {
                    false
                }
            } else {
                false
            };

            if in_ghost {
                spans.push(Span::styled(" ", Style::default().bg(ghost_color)));
            } else if let Some(piece_idx) = app.board.get(vx, board_y) {
                let color = PIECE_COLORS[piece_idx % PIECE_COLORS.len()];
                spans.push(Span::styled(" ", Style::default().bg(color)));
            } else {
                spans.push(Span::raw(" "));
            }
        }

        frame.render_widget(
            Paragraph::new(ratatui::text::Line::from(spans)),
            Rect::new(inner.x, inner.y + vy as u16, inner.width, 1),
        );
    }
}
