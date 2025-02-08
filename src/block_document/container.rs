use crate::block_document::rectangle::Rectangle;

pub struct Container {
    blocks: Vec<Rectangle>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Rectangle) {
        self.blocks.push(block);
    }
}
