use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListItem, Paragraph};

use super::app::{Action, App};

pub(crate) fn render(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(9)])
        .split(area);

    render_piece_list(frame, app, chunks[0]);
    render_controls(frame, chunks[1]);
}

fn render_piece_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .pieces
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let title = p.title.as_deref().unwrap_or("(untitled)");
            let prefix = if app
                .actions
                .iter()
                .any(|a| matches!(a, Action::Placed(idx) if *idx == i))
            {
                "✓"
            } else if app
                .actions
                .iter()
                .any(|a| matches!(a, Action::Skipped(idx) if *idx == i))
            {
                "–"
            } else if i == app.current {
                "►"
            } else {
                " "
            };
            let style = if i == app.current {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default()
            };
            ListItem::new(format!("{} {}. {}", prefix, i + 1, title)).style(style)
        })
        .collect();

    let list = List::new(items).block(Block::bordered().title(" Pieces "));
    frame.render_widget(list, area);
}

fn render_controls(frame: &mut Frame, area: Rect) {
    let lines: Vec<ratatui::text::Line> = [
        "↑↓←→  Move",
        "Enter  Place",
        "U      Undo",
        "S      Skip",
        "Q      Export",
    ]
    .iter()
    .map(|&s| ratatui::text::Line::from(s))
    .collect();

    let widget = Paragraph::new(lines).block(Block::bordered().title(" Controls "));
    frame.render_widget(widget, area);
}
