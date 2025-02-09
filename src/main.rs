mod block_document;
use block_document::document_json;
use block_document::pdf_generator;
use std::fs::File;

fn main() {
    let document1 = document_json::parse(); // TODO: 実行時パラメータの JSON ファイル名を渡す
    pdf_generator::dummy(
        document1,
        File::create("output/printpdf_output_.pdf").unwrap(),
    );

    let document2 = document_json::parse(); // TODO: 実行時パラメータの JSON ファイル名を渡す
    pdf_generator::generate(
        document2,
        File::create("output/printpdf_output.pdf").unwrap(),
    ); // TODO: 実行時パラメータの 出力ファイル名 を渡す
}
