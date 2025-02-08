mod box_document;
use box_document::document::Document;
use box_document::root::Root;

fn main() {
    let root = Root::new();

    let document = Document::new(root);
    document.save();
}
