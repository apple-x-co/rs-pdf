use crate::block_document::geometry::{GeoRect, GeoSize};
use crate::block_document::style::{Style, TextWrap};
use crate::block_document::text_renderer::WrappedText;

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub font_size: f32, // NOTE: PT
    pub font_path: Option<String>,
    pub frame: Option<GeoRect>,
    pub text_size: Option<GeoSize>,
    pub styles: Vec<Style>,
    pub wrapped_text: Option<WrappedText>,
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
            wrapped_text: None,
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

    pub fn get_text_wrap(&self) -> TextWrap {
        for style in &self.styles {
            if let Style::TextWrap(text_wrap) = style {
                return text_wrap.clone();
            }
        }

        TextWrap::default()
    }

    pub fn needs_wrapping(&self) -> bool {
        if let Some(frame) = &self.frame {
            if frame.size.is_some() {
                let wrap = self.get_text_wrap();
                return !matches!(wrap.mode, crate::block_document::style::TextWrapMode::None);
            }
        }

        false
    }

    pub fn get_available_width(&self) -> Option<f32> {
        self.frame.as_ref()?.size.as_ref().map(|s| s.width)
    }

    pub fn get_available_height(&self) -> Option<f32> {
        self.frame.as_ref()?.size.as_ref().map(|s| s.height)
    }

    pub fn set_wrapped_text(&mut self, wrapped_text: WrappedText) {
        self.wrapped_text = Some(wrapped_text);
    }

    pub fn get_wrapped_text(&self) -> Option<&WrappedText> {
        self.wrapped_text.as_ref()
    }

    pub fn get_display_text(&self) -> Vec<String> {
        if let Some(wrapped) = &self.wrapped_text {
            wrapped.lines.iter().map(|line| line.text.clone()).collect()
        } else {
            // 折り返しがない場合は改行で分割
            self.text.lines().map(|s| s.to_string()).collect()
        }
    }
}
