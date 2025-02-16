use crate::block_document::block::Block;
use crate::block_document::geometry::Bounds;
use crate::block_document::style::Style;

#[derive(Debug)]
pub struct Line {
    pub bounds: Bounds,
    pub styles: Vec<Style>,
}

impl Block for Line {}

impl Line {
    pub fn new(bounds: Bounds) -> Line {
        Line {
            bounds,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}
