mod box_document;
use box_document::document::Document;

fn main() {
    let document = Document::new();
    document.save();
}
