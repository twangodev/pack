use printpdf::*;

use crate::piece::Piece;

use super::metrics::PageLayout;

/// Emit PDF ops for visible text lines of a piece.
pub(crate) fn render_text_ops(
    piece: &Piece,
    first_vis: usize,
    last_vis: usize,
    box_left: f32,
    box_top: f32,
    layout: &PageLayout,
    font_handle: &PdfFontHandle,
) -> Vec<Op> {
    let mut ops = Vec::new();

    for i in first_vis..last_vis {
        let text = piece.lines[i].text();
        if text.is_empty() {
            continue;
        }
        let text_y = box_top
            - (i - first_vis) as f32 * layout.font.line_height_mm
            - layout.font.baseline_offset_mm;

        ops.push(Op::StartTextSection);
        ops.push(Op::SetFont {
            font: font_handle.clone(),
            size: Pt(layout.font.font_size_pt),
        });
        ops.push(Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                icc_profile: None,
            }),
        });
        ops.push(Op::SetTextCursor {
            pos: Point::new(Mm(box_left), Mm(text_y)),
        });
        ops.push(Op::ShowText {
            items: vec![TextItem::Text(text)],
        });
        ops.push(Op::EndTextSection);
    }

    ops
}
