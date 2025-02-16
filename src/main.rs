mod block_document;
use block_document::document_json;
use block_document::pdf_writer;
use std::fs::File;

fn main() {
    let document = document_json::parse(); // TODO: 実行時パラメータの JSON ファイル名を渡す
    pdf_writer::save(
        document,
        File::create("output/printpdf_output.pdf").unwrap(),
    ); // TODO: 実行時パラメータの 出力ファイル名 を渡す
}
