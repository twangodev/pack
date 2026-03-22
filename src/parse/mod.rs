mod markdown;
pub mod types;

pub use types::{ContentItem, ListEntry, Section, TextRun};

use crate::piece::Piece;

pub struct ParseConfig {
    pub split_level: u8,
    pub max_width: usize,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            split_level: 2,
            max_width: 40,
        }
    }
}

pub fn extract_pieces(input: &str, config: &ParseConfig) -> Vec<Piece> {
    let sections = markdown::parse_markdown(input, config.split_level);
    sections
        .iter()
        .enumerate()
        .map(|(i, s)| crate::piece::render::render_piece(s, i, config.max_width))
        .collect()
}
