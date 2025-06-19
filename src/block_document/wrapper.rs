use crate::block_document::block::BlockType;
use crate::block_document::geometry::GeoRect;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Wrapper {
    pub block: BlockType,
    pub frame: Option<GeoRect>,
    pub styles: Vec<Style>,
}

impl Wrapper {
    pub fn new(block: BlockType) -> Wrapper {
        Wrapper {
            block,
            frame: None,
            styles: vec![],
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn set_frame(&mut self, frame: GeoRect) {
        self.frame = Some(frame);
    }
}
