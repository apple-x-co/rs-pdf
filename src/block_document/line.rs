use crate::block_document::block::Block;
use crate::block_document::geometry::Bounds;

#[derive(Debug)]
pub struct Line {
    pub bounds: Bounds,
}

impl Block for Line {}

impl Line {
    pub fn new(bounds: Bounds) -> Line {
        Line {
            bounds,
        }
    }
}
