mod block_document;
use block_document::document_json;
use block_document::pdf_writer;
use std::fs::File;
use std::path::Path;
use std::process::exit;

fn main() {
    let output_path = "output/printpdf_output.pdf"; // TODO: 実行時パラメータの 出力ファイル名 を渡す
    let is_debug = true; // TODO: 実行時パラメーターの デバッグ を渡す
    let is_override = true; // TODO: 実行時パラメーターの 上書き を渡す

    if !is_override && Path::new(output_path).exists() {
        eprintln!("The output path already exists!");
        exit(1);
    }

    let output_file = File::create(&output_path).map_err(|e|{
        eprintln!("Could not create output file! {}", e);
        e
    });
    let file = match output_file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            exit(1);
        }
    };

    let document = document_json::parse(); // TODO: 実行時パラメータの JSON ファイル名を渡す
    pdf_writer::save(document, file, is_debug);
}
