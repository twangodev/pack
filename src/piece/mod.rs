pub mod outline;
pub(crate) mod render;
pub mod shape;
pub mod types;

pub use outline::{OutlineGroup, compute_outline_groups};
pub use shape::PieceShape;
pub use types::{LineSegment, Piece, RenderedLine, TextStyle};
