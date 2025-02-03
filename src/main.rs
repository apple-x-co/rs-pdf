use printpdf::{PdfDocument, Mm, BuiltinFont};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let (doc, page_index, layer_index) = PdfDocument::new("Title", Mm(210.0), Mm(297.0), "Layer");

    let layer = doc.get_page(page_index).get_layer(layer_index);

    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    layer.use_text("HELLO WORLD", 48.0, Mm(10.0), Mm(20.0), &font);

    doc.save(&mut BufWriter::new(File::create("output/printpdf_output.pdf").unwrap()))
        .unwrap();
}
