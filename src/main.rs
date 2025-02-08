mod box_document;

use box_document::document_json;
use box_document::pdf_generator;
use std::fs::File;

fn main() {
    let document = document_json::parse(); // TODO: 実行時パラメータの JSON ファイル名を渡す

    pdf_generator::generate(
        document,
        File::create("output/printpdf_output.pdf").unwrap(),
    ); // TODO: 実行時パラメータの 出力ファイル名 を渡す
}
