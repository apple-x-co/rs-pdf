use crate::block_document::geometry::Bounds;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub bounds: Option<Bounds>,
    pub styles: Vec<Style>,
}

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
