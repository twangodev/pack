use crate::piece::PieceShape;

#[derive(Debug, Clone, Copy)]
pub struct Anchor {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct PlacedPiece {
    pub piece_index: usize,
    pub anchor: Anchor,
    pub shape: PieceShape,
}
