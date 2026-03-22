use crate::parse::types::*;

use super::types::*;

pub(crate) fn render_piece(section: &Section, section_index: usize, max_width: usize) -> Piece {
    let mut lines = Vec::new();

    if let Some(title) = &section.title {
        lines.push(RenderedLine::single(title.clone(), TextStyle::Bold));
    }

    for item in &section.items {
        render_item(item, max_width, &mut lines, 0);
    }

    Piece {
        section_index,
        title: section.title.clone(),
        lines,
    }
}

fn render_item(item: &ContentItem, max_width: usize, lines: &mut Vec<RenderedLine>, indent: usize) {
    let available = max_width.saturating_sub(indent);

    match item {
        ContentItem::Paragraph { runs } => {
            render_paragraph(runs, available, indent, lines);
        }
        ContentItem::CodeBlock { code } => {
            for line in code.lines() {
                lines.push(RenderedLine::single(line, TextStyle::Code));
            }
        }
        ContentItem::Heading { text, .. } => {
            lines.push(RenderedLine::single(text.clone(), TextStyle::Bold));
        }
        ContentItem::List { items, ordered } => {
            for (i, entry) in items.iter().enumerate() {
                let bullet = if *ordered {
                    format!("{}. ", i + 1)
                } else {
                    "• ".to_string()
                };
                render_list_entry(entry, max_width, lines, indent, &bullet);
            }
        }
        ContentItem::Table { headers, rows } => {
            render_table(headers, rows, lines);
        }
        ContentItem::Rule => {
            lines.push(RenderedLine::single("───", TextStyle::Normal));
        }
        ContentItem::BlockQuote { items } => {
            for child in items {
                render_item(child, max_width, lines, indent + 2);
            }
        }
    }
}

fn render_paragraph(
    runs: &[TextRun],
    available: usize,
    indent: usize,
    lines: &mut Vec<RenderedLine>,
) {
    let wrapped = wrap_runs(runs, available);
    if indent == 0 {
        lines.extend(wrapped);
        return;
    }

    let prefix = " ".repeat(indent);
    for line in wrapped {
        let mut segs = vec![LineSegment {
            text: prefix.clone(),
            style: TextStyle::Normal,
        }];
        segs.extend(line.segments);
        lines.push(RenderedLine::new(segs));
    }
}

fn render_table(headers: &[String], rows: &[Vec<String>], lines: &mut Vec<RenderedLine>) {
    lines.push(RenderedLine::single(headers.join(" │ "), TextStyle::Bold));
    for row in rows {
        lines.push(RenderedLine::single(row.join(" │ "), TextStyle::Normal));
    }
}

fn render_list_entry(
    entry: &ListEntry,
    max_width: usize,
    lines: &mut Vec<RenderedLine>,
    indent: usize,
    bullet: &str,
) {
    if !entry.runs.is_empty() {
        let available = max_width.saturating_sub(indent + bullet.chars().count());
        let prefix = " ".repeat(indent);
        let continuation_pad = " ".repeat(bullet.chars().count());

        for (j, line) in wrap_runs(&entry.runs, available).into_iter().enumerate() {
            let pfx = if j == 0 {
                format!("{}{}", prefix, bullet)
            } else {
                format!("{}{}", prefix, continuation_pad)
            };
            let mut segs = vec![LineSegment {
                text: pfx,
                style: TextStyle::Normal,
            }];
            segs.extend(line.segments);
            lines.push(RenderedLine::new(segs));
        }
    }

    for child in &entry.children {
        render_list_entry(child, max_width, lines, indent + 2, "· ");
    }
}

// --- Word wrapping ---

struct Token {
    text: String,
    style: TextStyle,
    space_before: bool,
}

fn tokenize_runs(runs: &[TextRun]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut trailing_space = false;

    for run in runs {
        let has_leading_space = run.text.starts_with(char::is_whitespace);
        let has_trailing_space = run.text.ends_with(char::is_whitespace);
        let words: Vec<&str> = run.text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let space_before = if i == 0 {
                !tokens.is_empty() && (trailing_space || has_leading_space)
            } else {
                true
            };
            tokens.push(Token {
                text: word.to_string(),
                style: run.style,
                space_before,
            });
        }

        trailing_space = has_trailing_space || words.is_empty();
    }

    tokens
}

fn wrap_runs(runs: &[TextRun], max_width: usize) -> Vec<RenderedLine> {
    let tokens = tokenize_runs(runs);
    if tokens.is_empty() {
        return vec![];
    }

    let mut result: Vec<RenderedLine> = Vec::new();
    let mut segments: Vec<LineSegment> = Vec::new();
    let mut width: usize = 0;

    for token in &tokens {
        let space = if token.space_before && !segments.is_empty() {
            1
        } else {
            0
        };
        let token_len = token.text.chars().count();

        if width + space + token_len > max_width && !segments.is_empty() {
            result.push(RenderedLine::new(std::mem::take(&mut segments)));
            segments.push(LineSegment {
                text: token.text.clone(),
                style: token.style,
            });
            width = token_len;
            continue;
        }

        if let Some(last) = segments.last_mut() {
            if last.style == token.style {
                if space > 0 {
                    last.text.push(' ');
                }
                last.text.push_str(&token.text);
                width += space + token_len;
                continue;
            }
        }

        let text = if space > 0 {
            format!(" {}", token.text)
        } else {
            token.text.clone()
        };
        width += space + token_len;
        segments.push(LineSegment {
            text,
            style: token.style,
        });
    }

    if !segments.is_empty() {
        result.push(RenderedLine::new(segments));
    }

    result
}
