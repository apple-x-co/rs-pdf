use crate::block_document::geometry::{Bounds, Size};
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub font_size: f32, // NOTE: PT
    pub font_path: Option<String>,
    pub bounds: Option<Bounds>,
    pub text_size: Option<Size>,
    pub styles: Vec<Style>,
}

impl Text {
    pub fn new(
        text: String,
        font_size: f32,
        font_path: Option<String>,
        bounds: Option<Bounds>,
    ) -> Text {
        Text {
            text,
            font_size,
            font_path,
            bounds,
            text_size: None,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = Some(bounds);
    }
    
    pub fn set_text_size(&mut self, size: Size) {
        self.text_size = Some(size);
    }
}
