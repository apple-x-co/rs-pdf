use crate::block_document::block::Block;
use crate::block_document::geometry::GeoRect;

#[derive(Debug, Clone)]
pub struct FlexibleItem {
    pub block: Block,
    pub frame: Option<GeoRect>,
    pub basis: Option<f32>
}

impl FlexibleItem {
    pub fn new(block: Block, basis: Option<f32>) -> FlexibleItem {
        FlexibleItem {
            block,
            frame: None,
            basis
        }
    }

    pub fn set_frame(&mut self, frame: GeoRect) {
        self.frame = Some(frame);
    }
}
