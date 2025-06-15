use crate::block_document::block::BlockType;
use crate::block_document::geometry::Bounds;

#[derive(Debug, Clone)]
pub struct FlexibleItem {
    pub block: BlockType,
    pub bounds: Option<Bounds>,
}

impl FlexibleItem {
    pub fn new(block: BlockType) -> FlexibleItem {
        FlexibleItem {
            block,
            bounds: None,
        }
    }

    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = Some(bounds);
    }
}
