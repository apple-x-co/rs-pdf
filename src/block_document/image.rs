use crate::block_document::block::Block;
use crate::block_document::bounds::Bounds;

#[derive(Debug)]
pub struct Image {
    pub location: String,
    pub bounds: Option<Bounds>,
}

impl Block for Image {}

impl Image {
    pub fn new(location: String, bounds: Option<Bounds>) -> Image {
        Image {
            location,
            bounds,
        }
    }
}