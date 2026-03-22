use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, Options, parse_document};

use super::types::*;
use crate::piece::TextStyle;

pub(crate) fn parse_markdown(input: &str, split_level: u8) -> Vec<Section> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;

    let root = parse_document(&arena, input, &options);

    let mut sections: Vec<Section> = Vec::new();
    let mut current_items: Vec<ContentItem> = Vec::new();
    let mut current_title: Option<String> = None;

    for child in root.children() {
        if is_split_heading(child, split_level) {
            if current_title.is_some() || !current_items.is_empty() {
                sections.push(Section {
                    title: current_title.take(),
                    items: std::mem::take(&mut current_items),
                });
            }
            current_title = Some(collect_text(child));
        } else if let Some(item) = node_to_content_item(child) {
            current_items.push(item);
        }
    }

    if current_title.is_some() || !current_items.is_empty() {
        sections.push(Section {
            title: current_title,
            items: current_items,
        });
    }

    sections.retain(|s| s.title.is_some() || !s.items.is_empty());
    sections
}

fn is_split_heading<'a>(node: &'a AstNode<'a>, split_level: u8) -> bool {
    matches!(
        &node.data.borrow().value,
        NodeValue::Heading(h) if h.level == split_level
    )
}

fn collect_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    collect_text_recursive(node, &mut text);
    text
}

fn collect_text_recursive<'a>(node: &'a AstNode<'a>, out: &mut String) {
    match &node.data.borrow().value {
        NodeValue::Text(t) => out.push_str(t),
        NodeValue::Code(c) => out.push_str(&c.literal),
        NodeValue::SoftBreak | NodeValue::LineBreak => out.push(' '),
        _ => {
            for child in node.children() {
                collect_text_recursive(child, out);
            }
        }
    }
}

fn node_to_content_item<'a>(node: &'a AstNode<'a>) -> Option<ContentItem> {
    match &node.data.borrow().value {
        NodeValue::Heading(h) => Some(ContentItem::Heading {
            level: h.level,
            text: collect_text(node),
        }),
        NodeValue::Paragraph => {
            let runs = collect_inline_runs(node, TextStyle::Normal);
            Some(ContentItem::Paragraph { runs })
        }
        NodeValue::CodeBlock(cb) => Some(ContentItem::CodeBlock {
            code: cb.literal.to_string(),
        }),
        NodeValue::List(list) => {
            let ordered = list.list_type == comrak::nodes::ListType::Ordered;
            let items = node.children().filter_map(build_list_entry).collect();
            Some(ContentItem::List { items, ordered })
        }
        NodeValue::Table(_) => Some(parse_table(node)),
        NodeValue::BlockQuote => {
            let items = node.children().filter_map(node_to_content_item).collect();
            Some(ContentItem::BlockQuote { items })
        }
        NodeValue::ThematicBreak => Some(ContentItem::Rule),
        _ => None,
    }
}

fn parse_table<'a>(node: &'a AstNode<'a>) -> ContentItem {
    let mut rows_iter = node.children();
    let headers = rows_iter
        .next()
        .map(|row| row.children().map(|cell| collect_text(cell)).collect())
        .unwrap_or_default();

    let rows = rows_iter
        .map(|row| row.children().map(|cell| collect_text(cell)).collect())
        .collect();

    ContentItem::Table { headers, rows }
}

fn build_list_entry<'a>(node: &'a AstNode<'a>) -> Option<ListEntry> {
    match &node.data.borrow().value {
        NodeValue::Item(_) => {
            let mut runs = Vec::new();
            let mut children = Vec::new();

            for child in node.children() {
                match &child.data.borrow().value {
                    NodeValue::Paragraph => {
                        runs.extend(collect_inline_runs(child, TextStyle::Normal));
                    }
                    NodeValue::List(_) => {
                        children.extend(child.children().filter_map(build_list_entry));
                    }
                    _ => {}
                }
            }

            Some(ListEntry { runs, children })
        }
        _ => None,
    }
}

fn collect_inline_runs<'a>(node: &'a AstNode<'a>, parent_style: TextStyle) -> Vec<TextRun> {
    let mut runs = Vec::new();
    for child in node.children() {
        collect_inline_recursive(child, parent_style, &mut runs);
    }
    runs
}

fn collect_inline_recursive<'a>(
    node: &'a AstNode<'a>,
    parent_style: TextStyle,
    runs: &mut Vec<TextRun>,
) {
    match &node.data.borrow().value {
        NodeValue::Text(t) => {
            runs.push(TextRun {
                text: t.to_string(),
                style: parent_style,
            });
        }
        NodeValue::Code(c) => {
            runs.push(TextRun {
                text: c.literal.to_string(),
                style: TextStyle::Code,
            });
        }
        NodeValue::Strong => {
            let style = match parent_style {
                TextStyle::Italic => TextStyle::BoldItalic,
                _ => TextStyle::Bold,
            };
            for child in node.children() {
                collect_inline_recursive(child, style, runs);
            }
        }
        NodeValue::Emph => {
            let style = match parent_style {
                TextStyle::Bold => TextStyle::BoldItalic,
                _ => TextStyle::Italic,
            };
            for child in node.children() {
                collect_inline_recursive(child, style, runs);
            }
        }
        NodeValue::SoftBreak | NodeValue::LineBreak => {
            runs.push(TextRun {
                text: " ".to_string(),
                style: parent_style,
            });
        }
        _ => {
            for child in node.children() {
                collect_inline_recursive(child, parent_style, runs);
            }
        }
    }
}
