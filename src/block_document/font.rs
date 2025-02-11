use std::fs::File;
use std::io::Read;
use crate::block_document::bounds::Bounds;
use rusttype::{point, Font, Scale};

pub fn measure_text(text: &String, size: f32) -> Bounds {
    let font_data = include_bytes!("../../assets/fonts/NotoSansJP-VariableFont_wght.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(size);
    let v_metrics = font.v_metrics(scale);
    let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, v_metrics.ascent)).collect();
    let width = if let Some(last_glyph) = glyphs.last() {
        last_glyph.position().x + last_glyph.unpositioned().h_metrics().advance_width
    } else {
        0.0
    };

    Bounds {
        width: Some(width),
        height: Some(height),
        x: None,
        y: None,
    }
}
