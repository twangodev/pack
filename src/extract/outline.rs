use super::types::Piece;

pub fn render_outline(piece: &Piece) -> Vec<String> {
    let groups = piece.outline_groups();
    if groups.is_empty() {
        return vec![];
    }

    let mut out: Vec<String> = Vec::new();

    out.push(format!("┌{}┐", "─".repeat(groups[0].border_width)));

    for (gi, group) in groups.iter().enumerate() {
        if gi > 0 {
            let prev_w = groups[gi - 1].border_width;
            if prev_w != group.border_width {
                out.push(step_row(prev_w, group.border_width));
            }
        }

        for li in group.start..=group.end {
            let text = piece.lines[li].text();
            let pad = group.border_width.saturating_sub(piece.lines[li].width);
            out.push(format!("│{}{}│", text, " ".repeat(pad)));
        }
    }

    out.push(format!(
        "└{}┘",
        "─".repeat(groups.last().unwrap().border_width)
    ));
    out
}

/// Step row connecting two border widths with box-drawing characters.
///
/// Step down (next < prev):    Step up (next > prev):
///   │     ┌──────┘              │     └──────┐
///   0  next_w   prev_w          0  prev_w   next_w
fn step_row(prev_w: usize, next_w: usize) -> String {
    if next_w < prev_w {
        let gap = prev_w - next_w - 1;
        format!("│{}┌{}┘", " ".repeat(next_w), "─".repeat(gap))
    } else {
        let gap = next_w - prev_w - 1;
        format!("│{}└{}┐", " ".repeat(prev_w), "─".repeat(gap))
    }
}
