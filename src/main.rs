use image::GenericImageView;
use printpdf::{BuiltinFont, Color, Image, ImageTransform, Mm, PdfDocument, Rect, Rgb};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let (doc, page_index, layer_index) = PdfDocument::new("Title", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page_index).get_layer(layer_index);

    // --------------------
    // フォントを指定してテキストを描画
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    layer.use_text("HELLO WORLD", 48.0, Mm(10.0), Mm(20.0), &font);

    // --------------------
    let layer2 = doc.get_page(page_index).add_layer("Layer 2");
    // 画像を読み込む
    let image = image::io::Reader::open("assets/channel.png")
        .unwrap()
        .decode()
        .unwrap();
    let (image_width, image_height) = image.dimensions();

    // `printpdf::Image` に変換
    let pdf_image = Image::from_dynamic_image(&image);

    // 変換行列（スケールと位置の指定）
    let dpi = 300.0;
    let transform = ImageTransform {
        translate_x: Option::from(Mm(10.0)), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
        translate_y: Option::from(Mm(40.0)), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
        scale_x: Option::from(image_width as f32 / dpi), // NOTE: ミリメートル単位 で指定する。
        scale_y: Option::from(image_height as f32 / dpi), // NOTE: ミリメートル単位 で指定する。
        rotate: None,                        // 回転なし
        dpi: Option::from(dpi),
    };

    // 画像を PDF に描画
    pdf_image.add_to_layer(layer2, transform);

    // --------------------
    let layer3 = doc.get_page(page_index).add_layer("Layer 3");
    layer3.set_fill_color(Color::Rgb(Rgb {
        r: 255.0,
        g: 0.0,
        b: 0.0,
        icc_profile: None,
    }));
    layer3.add_rect(Rect::new(
        Mm(10.0),  // 左上X
        Mm(287.0), // 左上Y
        Mm(20.0),  // 右下X
        Mm(277.0), // 右下Y
    ));

    // --------------------
    // 保存
    doc.save(&mut BufWriter::new(
        File::create("output/printpdf_output.pdf").unwrap(),
    ))
    .unwrap();
}
