use crate::block_document::geometry::Size;
use ab_glyph::{Font, FontVec, ScaleFont};
use std::fs::File;
use std::io::Read;

pub fn measure_text(text: &String, font_size: f32, font_path: &String) -> Size {
    let mut file = File::open(font_path).unwrap();
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data).expect("Cannot read font");
    let font = FontVec::try_from_vec(font_data).unwrap();
    let scaled_font = font.as_scaled(font.pt_to_px_scale(font_size).unwrap());

    let mut width_px = 0.0;
    let mut prev_glyph = None;
    for c in text.chars() {
        let glyph_id = scaled_font.glyph_id(c);
        if let Some(prev) = prev_glyph {
            width_px += scaled_font.kern(prev, glyph_id); // カーニングを加算
        }
        width_px += scaled_font.h_advance(glyph_id); // 文字の横幅
        prev_glyph = Some(glyph_id);
    }
    let height_px: f32 = scaled_font.ascent() + scaled_font.descent() + scaled_font.line_gap();

    // let width_px: f32 = text.chars().map(|c| scaled_font.h_advance(scaled_font.glyph_id(c))).sum();
    // let height_px: f32 = scaled_font.ascent() - scaled_font.descent() + scaled_font.line_gap();

    // 1 px = 0.75 Pt
    let width_pt = width_px * 0.75;
    let height_pt = height_px * 0.75;

    // 1 Pt = 0.35278 Mm
    let width_mm = width_pt * 0.35278;
    let height_mm = height_pt * 0.35278;

    Size {
        width: width_mm,
        height: height_mm,
    }
}
