use crate::block_document::block::BlockType;
use crate::block_document::geometry::GeoRect;

#[derive(Debug, Clone)]
pub struct FlexibleItem {
    pub block: BlockType,
    pub frame: Option<GeoRect>,
}

impl FlexibleItem {
    pub fn new(block: BlockType) -> FlexibleItem {
        FlexibleItem {
            block,
            frame: None,
        }
    }

    pub fn set_frame(&mut self, frame: GeoRect) {
        self.frame = Some(frame);
    }
}
