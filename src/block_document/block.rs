use crate::block_document::block_container::BlockContainer;
use crate::block_document::flexible_container::FlexibleContainer;
use crate::block_document::image::Image;
use crate::block_document::line::Line;
use crate::block_document::rectangle::Rectangle;
use crate::block_document::text::Text;

#[derive(Debug, Clone)]
pub enum BlockType {
    Container(BlockContainer),
    Flexible(FlexibleContainer),
    Line(Line),
    Rectangle(Rectangle),
    Text(Text),
    Image(Image),
}