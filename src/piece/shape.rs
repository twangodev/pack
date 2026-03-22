use super::outline::compute_outline_groups;
use super::types::Piece;

/// The silhouette of a piece on the board: per-row widths derived from outline groups.
/// Since all rows share the same left edge (anchor.x), contiguity is guaranteed
/// when both consecutive rows have width > 0.
#[derive(Debug, Clone)]
pub struct PieceShape {
    pub row_widths: Vec<usize>,
    pub placeable: bool,
}

impl PieceShape {
    pub fn from_piece(piece: &Piece) -> Self {
        let groups = compute_outline_groups(piece);
        if groups.is_empty() {
            return Self {
                row_widths: vec![],
                placeable: false,
            };
        }

        let num_lines = groups.last().unwrap().end + 1;
        let mut row_widths = vec![0usize; num_lines];
        for g in &groups {
            for li in g.start..=g.end {
                row_widths[li] = g.border_width;
            }
        }

        let placeable = row_widths.iter().all(|&w| w > 0);

        Self {
            row_widths,
            placeable,
        }
    }

    pub fn height(&self) -> usize {
        self.row_widths.len()
    }

    pub fn max_width(&self) -> usize {
        self.row_widths.iter().copied().max().unwrap_or(0)
    }
}
