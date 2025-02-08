use crate::box_document::container::Container;

pub struct Document {
    pub containers: Vec<Container>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            containers: Vec::new(),
        }
    }
}
