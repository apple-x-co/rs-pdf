use crate::block_document::geometry::{GeoRect, GeoSize};
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub font_size: f32, // NOTE: PT
    pub font_path: Option<String>,
    pub frame: Option<GeoRect>,
    pub text_size: Option<GeoSize>,
    pub styles: Vec<Style>,
}

impl Text {
    pub fn new(
        text: String,
        font_size: f32,
        font_path: Option<String>,
        frame: Option<GeoRect>,
    ) -> Text {
        Text {
            text,
            font_size,
            font_path,
            frame,
            text_size: None,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn set_frame(&mut self, frame: GeoRect) {
        self.frame = Some(frame);
    }
    
    pub fn set_text_size(&mut self, size: GeoSize) {
        self.text_size = Some(size);
    }
}
