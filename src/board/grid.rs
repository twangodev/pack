use crate::piece::PieceShape;

use super::placement::{Anchor, PlacedPiece};

const BOARD_WIDTH: usize = 131; // (279.4 - 10.0) / 1.9 ≈ usable columns at 6pt

/// The packing board. Each cell is `None` (empty) or `Some(piece_index)`.
#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Vec<Option<usize>>>,
    pub placed: Vec<PlacedPiece>,
    pub width: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            placed: Vec::new(),
            width: BOARD_WIDTH,
        }
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn can_place(&self, shape: &PieceShape, anchor: Anchor) -> bool {
        for (r, &w) in shape.row_widths.iter().enumerate() {
            let y = anchor.y + r;
            let x_end = anchor.x + w;
            if x_end > self.width {
                return false;
            }
            if y < self.cells.len() {
                for x in anchor.x..x_end {
                    if self.cells[y][x].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn place(&mut self, piece_index: usize, shape: PieceShape, anchor: Anchor) {
        assert!(self.can_place(&shape, anchor));
        let needed = anchor.y + shape.height();
        self.ensure_height(needed);
        for (r, &w) in shape.row_widths.iter().enumerate() {
            let y = anchor.y + r;
            for x in anchor.x..(anchor.x + w) {
                self.cells[y][x] = Some(piece_index);
            }
        }
        self.placed.push(PlacedPiece {
            piece_index,
            anchor,
            shape,
        });
    }

    pub fn undo(&mut self) -> Option<PlacedPiece> {
        let placed = self.placed.pop()?;
        for (r, &w) in placed.shape.row_widths.iter().enumerate() {
            let y = placed.anchor.y + r;
            for x in placed.anchor.x..(placed.anchor.x + w) {
                self.cells[y][x] = None;
            }
        }
        Some(placed)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.cells.get(y)?.get(x)?.as_ref().copied()
    }

    pub fn bottom(&self) -> usize {
        for y in (0..self.cells.len()).rev() {
            if self.cells[y].iter().any(|c| c.is_some()) {
                return y + 1;
            }
        }
        0
    }

    fn ensure_height(&mut self, h: usize) {
        while self.cells.len() < h {
            self.cells.push(vec![None; self.width]);
        }
    }
}
