use crate::block_document::block::BlockType;
use crate::block_document::direction::Direction;
use crate::block_document::geometry::GeoRect;

#[derive(Debug, Clone)]
pub struct FlexibleContainer {
    pub blocks: Vec<BlockType>,
    pub frame: Option<GeoRect>,
    pub direction: Direction,
}

impl FlexibleContainer {
    pub fn new(frame: Option<GeoRect>) -> FlexibleContainer {
        FlexibleContainer {
            blocks: Vec::new(),
            frame,
            direction: Direction::Horizontal,
        }
    }

    pub fn add_block(&mut self, block: BlockType) {
        self.blocks.push(block);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_frame(&mut self, frame: GeoRect) {
        self.frame = Some(frame);
    }
}
