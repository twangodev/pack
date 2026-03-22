#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextStyle {
    Normal,
    Bold,
    Italic,
    BoldItalic,
    Code,
}

#[derive(Debug, Clone)]
pub struct LineSegment {
    pub text: String,
    pub style: TextStyle,
}

#[derive(Debug, Clone)]
pub struct RenderedLine {
    pub segments: Vec<LineSegment>,
    pub width: usize,
}

impl RenderedLine {
    pub fn new(segments: Vec<LineSegment>) -> Self {
        let width = segments.iter().map(|s| s.text.chars().count()).sum();
        Self { segments, width }
    }

    pub fn single(text: impl Into<String>, style: TextStyle) -> Self {
        Self::new(vec![LineSegment {
            text: text.into(),
            style,
        }])
    }

    pub fn text(&self) -> String {
        self.segments.iter().map(|s| s.text.as_str()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub section_index: usize,
    pub title: Option<String>,
    pub lines: Vec<RenderedLine>,
}
