use crate::block_document::block::BlockType;
use crate::block_document::block_container::BlockContainer;
use crate::block_document::container::Container;
use crate::block_document::document::{px_to_mm, Document};
use crate::block_document::geometry::{Bounds, Point, Size};
use crate::block_document::image::Image;
use crate::block_document::line::Line;
use crate::block_document::rectangle::Rectangle;
use crate::block_document::style::{BorderStyle, RgbColor, Style, TextOutlineStyle, TextStyle};
use crate::block_document::text::Text;
use crate::block_document::text_renderer::measure_text;
use image::GenericImageView;

const PAGE_A4_WIDTH: f32 = 210.0;
const PAGE_A4_HEIGHT: f32 = 297.0;

// TODO: JSON ファイルをパースして Document 構造体を返す
// NOTE: BlockDocument の座標基準は左上（printpdf は左下）
pub fn parse() -> Document {
    let mut doc = Document::new(
        String::from("HELLO"),
        Size {
            width: PAGE_A4_WIDTH,
            height: PAGE_A4_HEIGHT,
        },
    );

    let mut container = Container::new();

    // Block Test1 - Rectangle
    let mut rectangle1 = Rectangle::new(Some(Bounds::new(10.0, 10.0, 1.0, 1.0)));
    rectangle1.add_style(Style::BackgroundColor(RgbColor { r: 200, g: 200, b: 200 }));
    container.add_block(BlockType::Rectangle(rectangle1));

    // Block Test2 - Line
    let line1 = Line::new(Bounds::new(
        10.0,
        0.0,
        PAGE_A4_WIDTH - 11.0,
        PAGE_A4_HEIGHT - 1.0,
    ));
    container.add_block(BlockType::Line(line1));
    let line2 = Line::new(Bounds::new(
        0.0,
        10.0,
        PAGE_A4_WIDTH - 1.0,
        PAGE_A4_HEIGHT - 11.0,
    ));
    container.add_block(BlockType::Line(line2));

    // Block Test3 - Text
    let text_size = measure_text(
        &String::from("HELLO WORLD"),
        48.0,
        &String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
    );
    let mut text = Text::new(
        String::from("HELLO WORLD"),
        48.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        Some(Bounds::new(
            text_size.width,  // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            text_size.height, // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            1.0,
            PAGE_A4_HEIGHT - text_size.height - 1.0,
        )),
    );
    text.add_style(Style::BorderStyle(BorderStyle::Dash(2)));
    text.add_style(Style::BorderColor(RgbColor { r: 0, g: 0, b: 255 }));
    text.add_style(Style::BorderWidth(1.0)); // NOTE: 太さを設定すると描画される...
    text.add_style(Style::TextFillColor(RgbColor { r: 0, g: 255, b: 0 }));
    text.add_style(Style::TextOutlineColor(RgbColor { r: 255, g: 255, b: 0 }));
    text.add_style(Style::TextStyle(TextStyle::FillStroke));
    text.add_style(Style::TextOutlineStyle(TextOutlineStyle::Dash(2)));
    container.add_block(BlockType::Text(text));

    // Block Test4 - Image
    let image = image::io::Reader::open("assets/images/channel.png")
        .unwrap()
        .decode()
        .unwrap();
    let (image_width, image_height) = image.dimensions();
    let image = Image::new(
        String::from("assets/images/channel.png"),
        Some(Bounds::new(
            px_to_mm(image_width as f32), // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            px_to_mm(image_height as f32), // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            PAGE_A4_WIDTH - px_to_mm(image_width as f32) - 1.0,
            1.0,
        )),
    );
    container.add_block(BlockType::Image(image));

    // Block Test5 - BlockContainer
    let mut block_container = BlockContainer::new(Some(Bounds {
        point: Some(Point {
            x: (PAGE_A4_WIDTH / 2.0) - (50.0 / 2.0),
            y: (PAGE_A4_HEIGHT / 2.0) - (50.0 / 2.0),
        }), // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
        size: Some(Size {
            width: 50.0,
            height: 50.0,
        }), // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
    }));
    let mut rectangle2 = Rectangle::new(Some(Bounds::new(
        50.0, 50.0, 0.0, // NOTE: BlockContainer からの座標
        0.0, // NOTE: BlockContainer からの座標
    )));
    rectangle2.add_style(Style::BackgroundColor(RgbColor { r: 200, g: 255, b: 255 }));
    rectangle2.add_style(Style::BorderColor(RgbColor { r: 0, g: 200, b: 255 }));
    rectangle2.add_style(Style::BorderWidth(1.0));
    rectangle2.add_style(Style::BorderStyle(BorderStyle::Dash(2)));
    block_container.add_block(BlockType::Rectangle(rectangle2));
    let text_size2 = measure_text(
        &String::from("Hi!!"),
        20.0,
        &String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
    );
    let text2 = Text::new(
        String::from("Hi!!"),
        20.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        Some(Bounds::new(
            text_size2.width,  // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            text_size2.height, // NOTE: 指定なしの場合は自動計算する予定だが、今は指定必須
            1.0,               // NOTE: BlockContainer からの座標
            1.0,               // NOTE: BlockContainer からの座標
        )),
    );
    block_container.add_block(BlockType::Text(text2));
    container.add_block(BlockType::Container(block_container));

    doc.add_container(container);

    doc
}

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
