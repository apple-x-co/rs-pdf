use crate::block_document::block::Block;

pub struct Container {
    blocks: Vec<Block>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            blocks: Vec::new(),
        }
    }
}
