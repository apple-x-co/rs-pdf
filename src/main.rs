mod box_document;
use box_document::pdf_generator;
use box_document::document::Document;

fn main() {
    let document = Document::parse(); // TODO: 実行時パラメータの JSON ファイル名を渡す

    pdf_generator::generate(document);
}
