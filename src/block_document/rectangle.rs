use crate::block_document::block::{Block, BlockType};
use crate::block_document::direction::Direction;
use crate::block_document::bounds::Bounds;

#[derive(Debug)]
pub struct Rectangle {
    pub blocks: Vec<BlockType>,
    pub direction: Direction,
    pub bounds: Option<Bounds>,
}

impl Block for Rectangle {}

impl Rectangle {
    pub fn new(bounds: Option<Bounds>) -> Rectangle {
        Rectangle {
            blocks: Vec::new(),
            direction: Direction::Horizontal,
            bounds,
        }
    }
}
