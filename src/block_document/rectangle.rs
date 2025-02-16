use crate::block_document::block::Block;
use crate::block_document::geometry::Bounds;
use crate::block_document::style::Style;

#[derive(Debug)]
pub struct Rectangle {
    pub bounds: Option<Bounds>,
    pub styles: Vec<Style>,
}

impl Block for Rectangle {}

impl Rectangle {
    pub fn new(bounds: Option<Bounds>) -> Rectangle {
        Rectangle {
            bounds,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}
