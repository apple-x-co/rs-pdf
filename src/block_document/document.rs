use crate::block_document::container::Container;
use crate::block_document::geometry::Size;

pub const DPI: f32 = 300.0;

pub fn px_to_mm(pixel: f32) -> f32 {
    pixel / (DPI / 25.4)
}

#[derive(Clone)]
pub struct Document {
    pub title: String,
    pub page_size: Size,
    pub font_path: String,
    pub containers: Vec<Container>,
}

impl Document {
    pub fn new(title: String, page_size: Size, font_path: String) -> Document {
        Document {
            title,
            page_size,
            font_path,
            containers: Vec::new(),
        }
    }

    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }
}
