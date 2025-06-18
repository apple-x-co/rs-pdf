use crate::block_document::geometry::GeoRect;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub frame: Option<GeoRect>,
    pub styles: Vec<Style>,
}

impl Image {
    pub fn new(path: String, frame: Option<GeoRect>) -> Image {
        Image {
            path,
            frame,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn set_frame(&mut self, frame: GeoRect) {
        self.frame = Some(frame);
    }
}
