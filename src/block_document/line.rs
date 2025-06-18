// use crate::block_document::block::Block;
use crate::block_document::geometry::GeoRect;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Line {
    pub frame: GeoRect,
    pub styles: Vec<Style>,
}

impl Line {
    pub fn new(frame: GeoRect) -> Line {
        if frame.size.is_none() || frame.point.is_none() {
            panic!("The frame and points are not supported!");
        }

        Line {
            frame,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}
