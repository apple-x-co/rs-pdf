use crate::block_document::container::Container;
use crate::block_document::geometry::Size;

pub const DPI: f32 = 300.0;

pub fn px_to_mm(pixel: f32) -> f32 {
    pixel / (DPI / 25.4)
}

pub struct Document {
    pub title: String,
    pub size: Size,
    pub containers: Vec<Container>,
}

impl Document {
    pub fn new(title: String, width: f32, height: f32) -> Document {
        Document {
            title,
            size: Size { width, height },
            containers: Vec::new(),
        }
    }

    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }
}
