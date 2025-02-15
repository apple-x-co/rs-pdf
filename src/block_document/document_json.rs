use image::GenericImageView;
use crate::block_document::block::BlockType;
use crate::block_document::geometry::Bounds;
use crate::block_document::container::Container;
use crate::block_document::document::{px_to_mm, Document};
use crate::block_document::font::measure_text;
use crate::block_document::image::Image;
use crate::block_document::line::Line;
use crate::block_document::rectangle::Rectangle;
use crate::block_document::text::Text;

const PAGE_A4_WIDTH: f32 = 210.0;
const PAGE_A4_HEIGHT: f32 = 297.0;

// TODO: JSON ファイルをパースして Document 構造体を返す
// NOTE: BlockDocument の座標基準は左上（printpdf は左下）
pub fn parse() -> Document {
    let mut doc = Document::new(String::from("HELLO"), PAGE_A4_WIDTH, PAGE_A4_HEIGHT);

    let mut container = Container::new();

    // Block Test1 - Rectangle
    let rectangle = Rectangle::new(Some(Bounds {
        width: Some(10.0),
        height: Some(10.0),
        x: Some(1.0),
        y: Some(1.0),
    }));
    container.add_block(BlockType::Rectangle(rectangle));

    // Block Test2 - Line
    let line1 = Line::new(Bounds {
        width: Some(10.0),
        height: Some(0.0),
        x: Some(PAGE_A4_WIDTH - 11.0),
        y: Some(PAGE_A4_HEIGHT - 1.0),
    });
    container.add_block(BlockType::Line(line1));
    let line2 = Line::new(Bounds {
        width: Some(0.0),
        height: Some(10.0),
        x: Some(PAGE_A4_WIDTH - 1.0),
        y: Some(PAGE_A4_HEIGHT - 11.0),
    });
    container.add_block(BlockType::Line(line2));

    // Block Test3 - Text
    let text_bounds = measure_text(
        &String::from("HELLO WORLD"),
        48.0,
        &String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
    );
    let text = Text::new(
        String::from("HELLO WORLD"),
        48.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        Some(Bounds {
            width: text_bounds.width, // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            height: text_bounds.height, // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            x: Some(1.0),
            y: Some(PAGE_A4_HEIGHT - text_bounds.height.unwrap() - 1.0),
        }),
    );
    container.add_block(BlockType::Text(text));

    // Block Test4 - Image
    let image = image::io::Reader::open("assets/images/channel.png")
        .unwrap()
        .decode()
        .unwrap();
    let (image_width, image_height) = image.dimensions();
    let image = Image::new(
        String::from("assets/images/channel.png"),
        Some(Bounds {
            width: Some(px_to_mm(image_width as f32)), // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            height: Some(px_to_mm(image_height as f32)), // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            x: Some(166.666668),
            y: Some(1.0),
        }),
    );
    container.add_block(BlockType::Image(image));

    doc.add_container(container);

    doc
}

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
