use printpdf::{LinePoint, Mm, Op, PaintMode, Point, Polygon, PolygonRing, WindingOrder};

use crate::piece::outline::OutlineGroup;

use super::metrics::PageLayout;

pub(crate) fn pt(x_mm: f32, y_mm: f32) -> LinePoint {
    LinePoint {
        p: Point::new(Mm(x_mm), Mm(y_mm)),
        bezier: false,
    }
}

/// A clipped outline group with pre-computed PDF coordinates.
pub(crate) struct ClippedGroup {
    pub right: f32,
    pub gy_top: f32,
    pub gy_bot: f32,
}

/// Clip outline groups to a visible line range and compute PDF y-coordinates.
pub(crate) fn clip_groups(
    groups: &[OutlineGroup],
    first_vis: usize,
    last_vis: usize,
    box_left: f32,
    box_top: f32,
    layout: &PageLayout,
) -> Vec<ClippedGroup> {
    let mut clipped = Vec::new();
    for g in groups {
        let g_start = g.start.max(first_vis);
        let g_end = (g.end + 1).min(last_vis);
        if g_start >= g_end {
            continue;
        }

        let right = box_left + g.border_width as f32 * layout.font.char_width_mm;
        let gy_top = box_top - (g_start - first_vis) as f32 * layout.font.line_height_mm;
        let gy_bot = box_top - (g_end - first_vis) as f32 * layout.font.line_height_mm;
        clipped.push(ClippedGroup {
            right,
            gy_top,
            gy_bot,
        });
    }
    clipped
}

/// Build a closed polygon path from clipped outline groups.
pub(crate) fn build_outline_path(groups: &[ClippedGroup], box_left: f32) -> Vec<LinePoint> {
    let mut path = Vec::new();
    if groups.is_empty() {
        return path;
    }

    let box_top_y = groups[0].gy_top;
    let box_bot_y = groups.last().unwrap().gy_bot;

    path.push(pt(box_left, box_top_y));
    path.push(pt(groups[0].right, box_top_y));

    for (i, cg) in groups.iter().enumerate() {
        if i > 0 {
            let prev_right = groups[i - 1].right;
            if (cg.right - prev_right).abs() > 0.01 {
                path.push(pt(cg.right, groups[i - 1].gy_bot));
            }
        }
        path.push(pt(cg.right, cg.gy_bot));
    }

    path.push(pt(box_left, box_bot_y));
    path
}

/// Build a filled triangle marker pointing down (▼).
pub(crate) fn triangle_down(cx_mm: f32, cy_mm: f32, size_mm: f32) -> Op {
    let half = size_mm / 2.0;
    Op::DrawPolygon {
        polygon: Polygon {
            rings: vec![PolygonRing {
                points: vec![
                    pt(cx_mm - half, cy_mm + half), // top-left
                    pt(cx_mm + half, cy_mm + half), // top-right
                    pt(cx_mm, cy_mm - half),        // bottom-center
                ],
            }],
            mode: PaintMode::Fill,
            winding_order: WindingOrder::NonZero,
        },
    }
}

/// Build a filled triangle marker pointing up (▲).
pub(crate) fn triangle_up(cx_mm: f32, cy_mm: f32, size_mm: f32) -> Op {
    let half = size_mm / 2.0;
    Op::DrawPolygon {
        polygon: Polygon {
            rings: vec![PolygonRing {
                points: vec![
                    pt(cx_mm - half, cy_mm - half), // bottom-left
                    pt(cx_mm + half, cy_mm - half), // bottom-right
                    pt(cx_mm, cy_mm + half),        // top-center
                ],
            }],
            mode: PaintMode::Fill,
            winding_order: WindingOrder::NonZero,
        },
    }
}
