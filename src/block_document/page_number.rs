use crate::block_document::geometry::GeoRect;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct PageNumber {
    pub format: String,
    pub font_size: f32, // NOTE: PT
    pub font_path: Option<String>,
    pub frame: Option<GeoRect>,
    pub styles: Vec<Style>,
}

impl PageNumber {
    pub fn new(format: String, font_size: f32, font_path: Option<String>, frame: Option<GeoRect>) -> PageNumber {
        PageNumber {
            format,
            font_size,
            font_path,
            frame,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}