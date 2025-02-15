use crate::block_document::block::Block;
use crate::block_document::geometry::Bounds;

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub font_size: f32, // NOTE: PT
    pub font_path: String,
    pub bounds: Option<Bounds>,
}

impl Block for Text {}

impl Text {
    pub fn new(text: String, font_size: f32, font_path: String, bounds: Option<Bounds>) -> Text {
        Text {
            text,
            font_size,
            font_path,
            bounds,
        }
    }
}