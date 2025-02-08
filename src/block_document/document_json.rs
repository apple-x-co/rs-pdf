use crate::block_document::block::Block;
use crate::block_document::container::Container;
use crate::block_document::document::Document;

// TODO: JSON ファイルをパースして Document 構造体を返す
pub fn parse() -> Document {
    let mut doc = Document::new(String::from("HELLO"), 210.0, 297.0);

    let mut container = Container::new();

    let block = Block::new();
    container.add_block(block);

    doc.add_container(container);

    doc
}

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
