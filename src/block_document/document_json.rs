use crate::block_document::document::Document;

// TODO: JSON ファイルをパースして Document 構造体を返す
pub fn parse() -> Document {
    Document::new(String::from("HELLO"), 210.0, 297.0)
}

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
