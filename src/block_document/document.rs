use crate::block_document::container::Container;

pub const DPI: f32 = 300.0;

pub fn pixel_to_mm(pixel: f32) -> f32 {
    pixel / (DPI / 25.4)
}

pub struct Document {
    pub title: String,
    pub width: f32, // NOTE: mm
    pub height: f32, // NOTE: mm
    pub containers: Vec<Container>,
}

impl Document {
    pub fn new(title: String, width: f32, height: f32) -> Document {
        Document {
            title,
            width,
            height,
            containers: Vec::new(),
        }
    }

    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }
}
