use crate::block_document::block::{BlockType};

#[derive(Debug)]
pub struct Container {
    pub blocks: Vec<BlockType>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: BlockType) {
        self.blocks.push(block);
    }
}
