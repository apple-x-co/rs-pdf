// use crate::block_document::block::Block;
use crate::block_document::geometry::Bounds;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Line {
    pub bounds: Bounds,
    pub styles: Vec<Style>,
}

impl Line {
    pub fn new(bounds: Bounds) -> Line {
        if bounds.size.is_none() || bounds.point.is_none() {
            panic!("The bounds and points are not supported!");
        }

        Line {
            bounds,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}
