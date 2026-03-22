use crate::piece::TextStyle;

#[derive(Debug, Clone)]
pub struct Section {
    pub title: Option<String>,
    pub items: Vec<ContentItem>,
}

#[derive(Debug, Clone)]
pub enum ContentItem {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph {
        runs: Vec<TextRun>,
    },
    CodeBlock {
        code: String,
    },
    List {
        items: Vec<ListEntry>,
        ordered: bool,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Rule,
    BlockQuote {
        items: Vec<ContentItem>,
    },
}

#[derive(Debug, Clone)]
pub struct TextRun {
    pub text: String,
    pub style: TextStyle,
}

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub runs: Vec<TextRun>,
    pub children: Vec<ListEntry>,
}
