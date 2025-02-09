use crate::block_document::block::Block;
use crate::block_document::direction::Direction;
use crate::block_document::bounds::Bounds;

pub struct Rectangle {
    blocks: Vec<Rectangle>,
    direction: Direction,
    bounds: Option<Bounds>,
}

impl Block for Rectangle {}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
            blocks: Vec::new(),
            direction: Direction::Horizontal,
            bounds: None,
        }
    }
}
