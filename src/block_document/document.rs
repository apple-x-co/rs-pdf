use crate::block_document::container::Container;

pub struct Document {
    pub title: String,
    pub width: f32,
    pub height: f32,
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
