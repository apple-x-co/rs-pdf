use printpdf::{PdfDocument, BuiltinFont, Image, Mm, ImageTransform};
use std::fs::File;
use std::io::BufWriter;
use image::GenericImageView;

fn main() {
    let (doc, page_index, layer_index) = PdfDocument::new("Title", Mm(210.0), Mm(297.0), "Layer");

    let layer = doc.get_page(page_index).get_layer(layer_index);

    // フォントを指定してテキストを描画
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    layer.use_text("HELLO WORLD", 48.0, Mm(10.0), Mm(20.0), &font);

    // 画像を読み込む
    let image = image::io::Reader::open("assets/channel.png").unwrap().decode().unwrap();
    let (image_width, image_height) = image.dimensions();

    // `printpdf::Image` に変換
    let pdf_image = Image::from_dynamic_image(&image);

    // 画像を PDF に描画
    // let x = Mm(10.0); // 左から 10mm
    // let y = Mm(297.0 - 10.0 - (image_height as f32 * 0.2)); // 上から 10mm
    let scale = 0.2; // 画像のスケール（倍率）

    // 変換行列（スケールと位置の指定）
    let transform = ImageTransform {
        translate_x: Option::from(Mm(10.0)), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
        translate_y: Option::from(Mm(50.0)), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
        scale_x: Option::from(image_width as f32 * scale), // NOTE: ミリメートル単位 で指定する。
        scale_y: Option::from(image_height as f32 * scale), // NOTE: ミリメートル単位 で指定する。
        rotate: None, // 回転なし
        dpi: None,
    };

    // 画像を PDF に描画
    pdf_image.add_to_layer(layer, transform);

    // 保存
    doc.save(&mut BufWriter::new(File::create("output/printpdf_output.pdf").unwrap()))
        .unwrap();
}
