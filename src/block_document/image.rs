use crate::block_document::block::Block;
use crate::block_document::bounds::Bounds;

#[derive(Debug)]
pub struct Image {
    pub path: String,
    pub bounds: Option<Bounds>,
}

impl Block for Image {}

impl Image {
    pub fn new(path: String, bounds: Option<Bounds>) -> Image {
        Image {
            path,
            bounds,
        }
    }
}