use std::fs::File;
use std::io::Read;
use crate::block_document::bounds::Bounds;
use rusttype::{point, Font, Scale};

pub fn measure_text(text: &String, size: f32, font_path: &String) -> Bounds {
    let mut file = File::open(font_path).unwrap();
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data).expect("Cannot read font");
    let font = Font::try_from_bytes(&font_data[..]).unwrap();

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
