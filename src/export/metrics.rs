/// Converts points to millimeters.
const PT_TO_MM: f32 = 0.3528;

/// All layout metrics derived from a single font size.
pub struct FontMetrics {
    pub font_size_pt: f32,
    pub char_width_mm: f32,
    pub line_height_mm: f32,
    pub baseline_offset_mm: f32,
}

impl FontMetrics {
    pub fn new(font_size_pt: f32) -> Self {
        let char_width_mm = font_size_pt * 0.6 * PT_TO_MM;
        let line_height_mm = font_size_pt * 1.2 * PT_TO_MM;
        let baseline_offset_mm = line_height_mm * 0.8;
        Self {
            font_size_pt,
            char_width_mm,
            line_height_mm,
            baseline_offset_mm,
        }
    }
}

/// US Letter landscape page dimensions.
pub struct PageLayout {
    pub width_mm: f32,
    pub height_mm: f32,
    pub margin_mm: f32,
    pub rows_per_page: usize,
    pub font: FontMetrics,
}

impl PageLayout {
    pub fn us_letter_landscape(font_size_pt: f32, margin_mm: f32) -> Self {
        let font = FontMetrics::new(font_size_pt);
        let width_mm = 279.4;
        let height_mm = 215.9;
        let usable_height = height_mm - 2.0 * margin_mm;
        let rows_per_page = (usable_height / font.line_height_mm) as usize;
        Self {
            width_mm,
            height_mm,
            margin_mm,
            rows_per_page,
            font,
        }
    }
}
