use crossterm::event::KeyCode;

use crate::board::{Anchor, Board};
use crate::piece::{Piece, PieceShape};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Action {
    Placed(usize),
    Skipped(usize),
}

pub(crate) struct App {
    pub board: Board,
    pub pieces: Vec<Piece>,
    pub shapes: Vec<PieceShape>,
    pub cursor: Anchor,
    pub current: usize,
    pub actions: Vec<Action>,
    pub scroll_y: usize,
    pub done: bool,
    pub confirm_quit: bool,
}

impl App {
    pub fn new(pieces: Vec<Piece>) -> Self {
        let shapes: Vec<PieceShape> = pieces.iter().map(PieceShape::from_piece).collect();
        Self {
            board: Board::new(),
            pieces,
            shapes,
            cursor: Anchor { x: 0, y: 0 },
            current: 0,
            actions: Vec::new(),
            scroll_y: 0,
            done: false,
            confirm_quit: false,
        }
    }

    pub fn current_shape(&self) -> Option<&PieceShape> {
        self.shapes.get(self.current)
    }

    pub fn is_valid_placement(&self) -> bool {
        self.current_shape()
            .is_some_and(|s| s.placeable && self.board.can_place(s, self.cursor))
    }

    pub fn update_scroll(&mut self, visible_rows: usize) {
        let cursor_bottom = self.cursor.y + self.current_shape().map(|s| s.height()).unwrap_or(0);

        if cursor_bottom > self.scroll_y + visible_rows {
            self.scroll_y = cursor_bottom.saturating_sub(visible_rows);
        } else if self.cursor.y < self.scroll_y {
            self.scroll_y = self.cursor.y;
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        if self.confirm_quit {
            match key {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    self.done = true;
                }
                _ => {
                    self.confirm_quit = false;
                }
            }
            return;
        }

        match key {
            KeyCode::Up if self.cursor.y > 0 => self.cursor.y -= 1,
            KeyCode::Down => self.cursor.y += 1,
            KeyCode::Left if self.cursor.x > 0 => self.cursor.x -= 1,
            KeyCode::Right => self.cursor.x += 1,
            KeyCode::Enter => self.place_current(),
            KeyCode::Char('u') | KeyCode::Char('U') => self.undo(),
            KeyCode::Char('s') | KeyCode::Char('S') => self.skip_current(),
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                if self.current >= self.pieces.len() {
                    self.done = true;
                } else {
                    self.confirm_quit = true;
                }
            }
            _ => {}
        }
    }

    fn place_current(&mut self) {
        if !self.is_valid_placement() {
            return;
        }
        let idx = self.current;
        let shape = self.shapes[idx].clone();
        self.board.place(idx, shape, self.cursor);
        self.actions.push(Action::Placed(idx));
        self.advance();
    }

    fn skip_current(&mut self) {
        if self.current >= self.pieces.len() {
            return;
        }
        self.actions.push(Action::Skipped(self.current));
        self.advance();
    }

    fn undo(&mut self) {
        if let Some(action) = self.actions.pop() {
            match action {
                Action::Placed(idx) => {
                    self.board.undo();
                    self.current = idx;
                    self.done = false;
                }
                Action::Skipped(idx) => {
                    self.current = idx;
                    self.done = false;
                }
            }
            self.cursor = Anchor {
                x: 0,
                y: self.board.bottom(),
            };
        }
    }

    fn advance(&mut self) {
        self.current += 1;
        if self.current < self.pieces.len() {
            self.cursor = Anchor {
                x: 0,
                y: self.board.bottom(),
            };
        } else {
            self.done = true;
        }
    }
}
