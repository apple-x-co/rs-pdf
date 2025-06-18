use crate::block_document::geometry::GeoRect;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub frame: Option<GeoRect>,
    pub styles: Vec<Style>,
}

impl Rectangle {
    pub fn new(frame: Option<GeoRect>) -> Rectangle {
        Rectangle {
            frame,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}
