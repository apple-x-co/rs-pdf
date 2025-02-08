use crate::block_document::direction::Direction;
use crate::block_document::rect::Rect;

pub struct Block {
    blocks: Vec<Block>,
    direction: Direction,
    rect: Option<Rect>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            blocks: Vec::new(),
            direction: Direction::Horizontal,
            rect: None,
        }
    }
}
