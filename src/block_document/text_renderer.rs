use crate::block_document::geometry::GeoSize;
use ab_glyph::{Font, FontVec, ScaleFont};
use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn measure_text(text: &String, font_size: f32, font_path: &String) -> GeoSize {
    let file = File::open(font_path).map_err(|_| {
        eprintln!("Failed to open font file: {}.", font_path);
    });
    let font = match file {
        Ok(mut file) => {
            let mut font_data = Vec::new();
            file.read_to_end(&mut font_data).expect("Cannot read font");
            FontVec::try_from_vec(font_data).unwrap()
        }
        Err(_) => {
            eprintln!("Failed to open font file: {}.", font_path);
            exit(1);
        }
    };
    let scaled_font = font.as_scaled(font.pt_to_px_scale(font_size).unwrap());

    let mut max_width_px: f32 = 0.0;
    let mut current_width_px: f32 = 0.0;
    let mut prev_glyph = None;

    for c in text.chars() {
        if c == '\n' {
            max_width_px = max_width_px.max(current_width_px);
            current_width_px = 0.0;
            prev_glyph = None; // NOTE: 改行されたのでカーニングをリセット
            continue;
        }

        let glyph_id = scaled_font.glyph_id(c);
        if let Some(prev) = prev_glyph {
            current_width_px += scaled_font.kern(prev, glyph_id); // NOTE: カーニングを加算
        }

        current_width_px += scaled_font.h_advance(glyph_id); // NOTE: 文字の横幅
        prev_glyph = Some(glyph_id);
    }

    max_width_px = max_width_px.max(current_width_px);

    let num_lines = text.lines().count() as f32;
    let height_px: f32 = ((scaled_font.ascent() + scaled_font.descent()) * num_lines)
        + (scaled_font.line_gap() * (num_lines - 1.0));

    // let width_px: f32 = text.chars().map(|c| scaled_font.h_advance(scaled_font.glyph_id(c))).sum();
    // let height_px: f32 = scaled_font.ascent() - scaled_font.descent() + scaled_font.line_gap();

    // NOTE: 1 px = 0.75 Pt
    let width_pt = max_width_px * 0.75;
    let height_pt = height_px * 0.75;

    // NOTE: 1 Pt = 0.35278 Mm
    let width_mm = width_pt * 0.35278;
    let height_mm = height_pt * 0.35278;

    GeoSize {
        width: width_mm,
        height: height_mm,
    }
}
