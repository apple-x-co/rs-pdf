use crate::block_document::block::Block;

pub struct Container {
    blocks: Vec<Box<dyn Block>>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Box<dyn Block>) {
        self.blocks.push(block);
    }
}
