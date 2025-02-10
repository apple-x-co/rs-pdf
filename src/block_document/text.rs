use crate::block_document::block::Block;
use crate::block_document::bounds::Bounds;

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub bounds: Option<Bounds>,
}

impl Block for Text {}

impl Text {
    pub fn new(text: String, bounds: Option<Bounds>) -> Text {
        Text {
            text,
            bounds,
        }
    }
}