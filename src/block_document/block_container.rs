use crate::block_document::block::Block;
use crate::block_document::direction::Direction;
use crate::block_document::geometry::GeoRect;

#[derive(Debug, Clone)]
pub struct BlockContainer {
    pub blocks: Vec<Block>,
    pub frame: Option<GeoRect>,
    pub direction: Direction,
}

impl BlockContainer {
    pub fn new(frame: Option<GeoRect>) -> BlockContainer {
        BlockContainer {
            blocks: Vec::new(),
            frame,
            direction: Direction::Horizontal,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_frame(&mut self, frame: GeoRect) {
        self.frame = Some(frame);
    }
}
