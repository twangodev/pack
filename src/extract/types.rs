pub struct ExtractConfig {
    pub split_level: u8,
    pub max_width: usize,
}

impl Default for ExtractConfig {
    fn default() -> Self {
        Self {
            split_level: 2,
            max_width: 40,
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextStyle {
    Normal,
    Bold,
    Italic,
    BoldItalic,
    Code,
}

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub runs: Vec<TextRun>,
    pub children: Vec<ListEntry>,
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
pub struct LineSegment {
    pub text: String,
    pub style: TextStyle,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub section_index: usize,
    pub title: Option<String>,
    pub lines: Vec<RenderedLine>,
}

/// A contiguous group of lines sharing one border width.
#[derive(Debug, Clone)]
pub struct OutlineGroup {
    pub start: usize, // first line index (inclusive)
    pub end: usize,   // last line index (inclusive)
    pub border_width: usize,
}

impl Piece {
    pub fn widths(&self) -> Vec<usize> {
        self.lines.iter().map(|l| l.width).collect()
    }

    /// Partition lines into contiguous groups via DP, minimizing total area.
    /// Each group pads its lines to the group's max width. Step rows between
    /// groups of different widths cost one row each.
    pub fn outline_groups(&self) -> Vec<OutlineGroup> {
        let widths = self.widths();
        let n = widths.len();
        if n == 0 {
            return vec![];
        }

        let (_, choice) = Self::compute_dp(&widths);
        Self::backtrack_groups(&widths, &choice)
    }

    fn compute_dp(widths: &[usize]) -> (Vec<usize>, Vec<usize>) {
        let n = widths.len();
        let mut dp = vec![usize::MAX; n];
        let mut choice = vec![0usize; n];

        for i in 0..n {
            let mut group_max = 0usize;
            for j in (0..=i).rev() {
                group_max = group_max.max(widths[j]);
                let group_area = group_max * (i - j + 1);
                let step_cost = if j > 0 { group_max } else { 0 };
                let prev_cost = if j > 0 { dp[j - 1] } else { 0 };

                if prev_cost == usize::MAX {
                    continue;
                }

                let total = prev_cost + group_area + step_cost;
                if total < dp[i] {
                    dp[i] = total;
                    choice[i] = j;
                }
            }
        }

        (dp, choice)
    }

    fn backtrack_groups(widths: &[usize], choice: &[usize]) -> Vec<OutlineGroup> {
        let mut groups = Vec::new();
        let mut i = widths.len();
        while i > 0 {
            let start = choice[i - 1];
            let end = i - 1;
            let border_width = widths[start..=end].iter().copied().max().unwrap().max(1);
            groups.push(OutlineGroup {
                start,
                end,
                border_width,
            });
            i = start;
        }
        groups.reverse();
        groups
    }
}
