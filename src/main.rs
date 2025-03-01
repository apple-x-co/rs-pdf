mod block_document;
use block_document::document_json;
use block_document::pdf_writer;
use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_path: String,

    #[arg(short, long)]
    output_path: String,

    #[arg(short, long)]
    font_path: String,

    #[arg(short, long, default_value_t=false)]
    debug: bool,

    #[arg(short, long, default_value_t=false)]
    allow_override: bool,
}

fn main() {
    let args = Args::parse();

    if !args.allow_override && Path::new(args.output_path.as_str()).exists() {
        eprintln!("The output path already exists!");
        exit(1);
    }

    if !Path::new(args.input_path.as_str()).exists() {
        eprintln!("The input path does not exist!");
        exit(1);
    }

    let output_file = File::create(args.output_path.as_str()).map_err(|e|{
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

    let document = document_json::parse(
        args.input_path.as_str(),
        args.font_path.as_str(),
    );
    pdf_writer::save(document, file, args.debug);
}
