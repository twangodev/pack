mod outline;
mod parse;
mod piece;
mod types;

pub use outline::render_outline;
pub use types::{ExtractConfig, LineSegment, OutlineGroup, Piece, RenderedLine, TextStyle};

pub fn extract_pieces(markdown: &str, config: &ExtractConfig) -> Vec<Piece> {
    let sections = parse::parse_markdown(markdown, config.split_level);
    sections
        .iter()
        .enumerate()
        .map(|(i, s)| piece::render_piece(s, i, config.max_width))
        .collect()
}
