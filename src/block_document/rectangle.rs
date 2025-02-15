use crate::block_document::block::Block;
use crate::block_document::geometry::Bounds;

#[derive(Debug)]
pub struct Rectangle {
    pub bounds: Option<Bounds>,
}

impl Block for Rectangle {}

impl Rectangle {
    pub fn new(bounds: Option<Bounds>) -> Rectangle {
        Rectangle {
            bounds,
        }
    }
}
