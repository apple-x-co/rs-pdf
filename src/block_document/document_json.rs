use crate::block_document::rectangle::Rectangle;
use crate::block_document::container::Container;
use crate::block_document::document::Document;
use crate::block_document::text::Text;

// TODO: JSON ファイルをパースして Document 構造体を返す
pub fn parse() -> Document {
    let mut doc = Document::new(String::from("HELLO"), 210.0, 297.0);

    let mut container = Container::new();

    let rectangle = Rectangle::new();
    container.add_block(Box::new(rectangle));

    let text = Text::new(String::from("HELLO WORLD"), None);
    container.add_block(Box::new(text));

    doc.add_container(container);

    doc
}

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
