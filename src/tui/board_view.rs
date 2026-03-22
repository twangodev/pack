use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use crate::export::metrics::PageLayout;

use super::app::App;
use super::palette::PIECE_COLORS;

/// Convert a board row to the display row it appears at,
/// accounting for inserted page break lines.
fn board_to_display(board_y: usize, page_rows: usize) -> usize {
    if page_rows == 0 {
        return board_y;
    }
    // Number of page breaks before this board row
    let breaks = if board_y == 0 {
        0
    } else {
        (board_y - 1) / page_rows
    };
    board_y + breaks
}

/// Convert a display row back to either a board row or a page break.
enum DisplayRow {
    Board(usize),
    PageBreak(usize), // page number (1-indexed)
}

fn display_to_board(display_y: usize, page_rows: usize) -> DisplayRow {
    if page_rows == 0 {
        return DisplayRow::Board(display_y);
    }
    let stride = page_rows + 1; // page_rows board rows + 1 break line
    let cycle = display_y % stride;
    let full_pages = display_y / stride;

    if cycle == page_rows {
        // This is a page break line (after page_rows board rows)
        DisplayRow::PageBreak(full_pages + 2) // next page number
    } else {
        DisplayRow::Board(full_pages * page_rows + cycle)
    }
}

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

    let page_rows = PageLayout::us_letter_landscape(6.0, 5.0).rows_per_page;

    // Convert scroll_y (board space) to display space
    let scroll_display = board_to_display(app.scroll_y, page_rows);

    for vy in 0..visible_rows {
        let display_y = scroll_display + vy;
        let mut spans: Vec<Span> = Vec::new();

        match display_to_board(display_y, page_rows) {
            DisplayRow::PageBreak(page_num) => {
                let label = format!(" pg {} ", page_num);
                let dash_total = visible_cols.min(app.board.width);
                let label_start = dash_total.saturating_sub(label.len()) / 2;
                let break_style = Style::default().fg(Color::DarkGray);

                for col in 0..dash_total {
                    if col >= label_start && col < label_start + label.len() {
                        let ch = label.as_bytes()[col - label_start] as char;
                        spans.push(Span::styled(
                            ch.to_string(),
                            break_style.add_modifier(Modifier::DIM),
                        ));
                    } else {
                        spans.push(Span::styled("┄", break_style));
                    }
                }
            }
            DisplayRow::Board(board_y) => {
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
            }
        }

        frame.render_widget(
            Paragraph::new(ratatui::text::Line::from(spans)),
            Rect::new(inner.x, inner.y + vy as u16, inner.width, 1),
        );
    }
}
