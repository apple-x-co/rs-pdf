use crate::block_document::block_container::BlockContainer;
use crate::block_document::image::Image;
use crate::block_document::line::Line;
use crate::block_document::rectangle::Rectangle;
use crate::block_document::text::Text;
use std::any::TypeId;

pub trait Block: std::fmt::Debug {}

#[derive(Debug, Clone)]
pub enum BlockType {
    Container(BlockContainer),
    Line(Line),
    Rectangle(Rectangle),
    Text(Text),
    Image(Image),
}

impl Block for BlockType {}

impl BlockType {
    fn type_id(&self) -> TypeId {
        match self {
            BlockType::Container(_) => TypeId::of::<BlockContainer>(),
            BlockType::Line(_) => TypeId::of::<Line>(),
            BlockType::Rectangle(_) => TypeId::of::<Rectangle>(),
            BlockType::Text(_) => TypeId::of::<Text>(),
            BlockType::Image(_) => TypeId::of::<Image>(),
        }
    }
}
