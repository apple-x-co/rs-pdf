use crate::block_document::block::Block;
use crate::block_document::geometry::Bounds;
use crate::block_document::style::Style;

#[derive(Debug)]
pub struct Image {
    pub path: String,
    pub bounds: Option<Bounds>,
    pub styles: Vec<Style>,
}

impl Block for Image {}

impl Image {
    pub fn new(path: String, bounds: Option<Bounds>) -> Image {
        Image {
            path,
            bounds,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}
