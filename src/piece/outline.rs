use super::types::Piece;

/// A contiguous group of lines sharing one border width.
#[derive(Debug, Clone)]
pub struct OutlineGroup {
    pub start: usize,
    pub end: usize,
    pub border_width: usize,
}

/// Partition a piece's lines into contiguous groups via DP, minimizing total area.
pub fn compute_outline_groups(piece: &Piece) -> Vec<OutlineGroup> {
    let widths: Vec<usize> = piece.lines.iter().map(|l| l.width).collect();
    let n = widths.len();
    if n == 0 {
        return vec![];
    }

    let choice = compute_dp(&widths);
    backtrack_groups(&widths, &choice)
}

fn compute_dp(widths: &[usize]) -> Vec<usize> {
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

    choice
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
