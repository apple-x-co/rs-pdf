use printpdf::{PdfDocument, Mm};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let (doc, _, _) = PdfDocument::new("Title", Mm(210.0), Mm(297.0), "Layer");
    doc.save(&mut BufWriter::new(File::create("output/printpdf_output.pdf").unwrap()))
        .unwrap();
}
