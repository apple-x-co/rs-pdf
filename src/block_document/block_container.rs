use crate::block_document::block::{Block, BlockType};
use crate::block_document::direction::Direction;

#[derive(Debug)]
pub struct BlockContainer {
    pub blocks: Vec<BlockType>,
    pub direction: Direction,
}

impl Block for BlockContainer {}

impl BlockContainer {
    pub fn new() -> BlockContainer {
        BlockContainer {
            blocks: Vec::new(),
            direction: Direction::Horizontal,
        }
    }
}
