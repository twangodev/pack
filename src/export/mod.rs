pub mod metrics;
mod outline;
mod text;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use crate::board::Board;
use crate::piece::{Piece, compute_outline_groups};

use metrics::PageLayout;
use outline::{build_outline_path, clip_groups};

const BORDER_WIDTH_PT: f32 = 0.25;

static GEIST_MONO: &[u8] = include_bytes!("../../assets/fonts/GeistMono-Regular.otf");

pub fn to_pdf(
    board: &Board,
    pieces: &[Piece],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let layout = PageLayout::us_letter_landscape(6.0, 5.0);

    let mut doc = PdfDocument::new("Pack Output");
    let mut warnings = Vec::new();
    let parsed_font =
        ParsedFont::from_bytes(GEIST_MONO, 0, &mut warnings).ok_or("Failed to parse font")?;
    let font_id = doc.add_font(&parsed_font);
    let font_handle = PdfFontHandle::External(font_id);

    let total_rows = board.bottom();
    let num_pages = if total_rows == 0 {
        1
    } else {
        (total_rows + layout.rows_per_page - 1) / layout.rows_per_page
    };

    let mut all_pages: Vec<PdfPage> = Vec::new();

    for page_idx in 0..num_pages {
        let page_start = page_idx * layout.rows_per_page;
        let page_end = page_start + layout.rows_per_page;
        let mut page_ops: Vec<Op> = Vec::new();

        for placed in &board.placed {
            let piece = &pieces[placed.piece_index];
            let groups = compute_outline_groups(piece);
            if groups.is_empty() {
                continue;
            }

            let piece_start = placed.anchor.y;
            let piece_end = piece_start + placed.shape.height();

            if piece_end <= page_start || piece_start >= page_end {
                continue;
            }

            let first_vis = page_start.saturating_sub(piece_start).max(0);
            let last_vis = (piece_end.min(page_end)) - piece_start;

            let box_left = layout.margin_mm + placed.anchor.x as f32 * layout.font.char_width_mm;
            let first_vis_page_row = (piece_start + first_vis).saturating_sub(page_start) as f32;
            let box_top = layout.height_mm
                - layout.margin_mm
                - first_vis_page_row * layout.font.line_height_mm;

            // Outline
            let clipped = clip_groups(&groups, first_vis, last_vis, box_left, box_top, &layout);
            if clipped.is_empty() {
                continue;
            }

            let path_points = build_outline_path(&clipped, box_left);

            page_ops.push(Op::SetOutlineColor {
                col: Color::Rgb(Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    icc_profile: None,
                }),
            });
            page_ops.push(Op::SetOutlineThickness {
                pt: Pt(BORDER_WIDTH_PT),
            });
            page_ops.push(Op::DrawLine {
                line: Line {
                    points: path_points,
                    is_closed: true,
                },
            });

            // Text
            page_ops.extend(text::render_text_ops(
                piece,
                first_vis,
                last_vis,
                box_left,
                box_top,
                &layout,
                &font_handle,
            ));
        }

        all_pages.push(PdfPage::new(
            Mm(layout.width_mm),
            Mm(layout.height_mm),
            page_ops,
        ));
    }

    let bytes = doc
        .with_pages(all_pages)
        .save(&PdfSaveOptions::default(), &mut Vec::new());

    let mut writer = BufWriter::new(File::create(output_path)?);
    std::io::Write::write_all(&mut writer, &bytes)?;

    Ok(())
}
