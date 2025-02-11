use crate::block_document::block::Block;
use crate::block_document::bounds::Bounds;

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub size: f32,
    pub font_path: String,
    pub bounds: Option<Bounds>,
}

impl Block for Text {}

impl Text {
    pub fn new(text: String, size: f32, font_path: String, bounds: Option<Bounds>) -> Text {
        Text {
            text,
            size,
            font_path,
            bounds,
        }
    }
}