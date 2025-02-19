use crate::block_document::block::{BlockType};
use crate::block_document::geometry::Bounds;

#[derive(Debug, Clone)]
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

    pub fn apply_constraints(&mut self, bounds: &Bounds) {
        // FIXME
    }
}
