use crate::block_document::block::BlockType;
use crate::block_document::direction::Direction;
use crate::block_document::geometry::Bounds;

#[derive(Debug, Clone)]
pub struct BlockContainer {
    pub blocks: Vec<BlockType>,
    pub bounds: Option<Bounds>,
    pub direction: Direction,
}

impl BlockContainer {
    pub fn new(bounds: Option<Bounds>) -> BlockContainer {
        BlockContainer {
            blocks: Vec::new(),
            bounds,
            direction: Direction::Horizontal,
        }
    }

    pub fn add_block(&mut self, block: BlockType) {
        self.blocks.push(block);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = Some(bounds);
    }
}
