use crate::block_document::block::Block;
use crate::block_document::direction::Direction;
use crate::block_document::bounds::Bounds;

#[derive(Debug)]
pub struct Rectangle {
    blocks: Vec<Rectangle>,
    direction: Direction,
    bounds: Option<Bounds>,
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
