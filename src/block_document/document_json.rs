use crate::block_document::block::BlockType;
use crate::block_document::block_container::BlockContainer;
use crate::block_document::container::Container;
use crate::block_document::document::{px_to_mm, Document};
use crate::block_document::geometry::{Bounds, Point, Size};
use crate::block_document::image::Image;
use crate::block_document::line::Line;
use crate::block_document::rectangle::Rectangle;
use crate::block_document::style::{
    BorderStyle, RgbColor, Space, Style, TextOutlineStyle, TextStyle,
};
use crate::block_document::text::Text;
use crate::block_document::text_renderer::measure_text;
use image::GenericImageView;
use crate::block_document::direction::Direction;

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
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
    );

    // NOTE: 1ページ目
    let mut container1 = Container::new();

    // NOTE: Block Test1 - Rectangle
    let mut rectangle1 = Rectangle::new(Some(Bounds::new(10.0, 10.0, 1.0, 1.0)));
    rectangle1.add_style(Style::BackgroundColor(RgbColor {
        r: 200,
        g: 200,
        b: 200,
    }));
    container1.add_block(BlockType::Rectangle(rectangle1));

    let mut rectangle3 = Rectangle::new(Some(Bounds::new(10.0, 10.0, 1.0, 1.0)));
    rectangle3.add_style(Style::BackgroundColor(RgbColor {
        r: 230,
        g: 230,
        b: 230,
    }));
    rectangle3.add_style(Style::Space(Space {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    }));
    container1.add_block(BlockType::Rectangle(rectangle3));

    // NOTE: Block Test2 - Line
    let line1 = Line::new(Bounds::new(
        10.0,
        0.0,
        PAGE_A4_WIDTH - 11.0,
        PAGE_A4_HEIGHT - 1.0,
    ));
    container1.add_block(BlockType::Line(line1));
    let mut line2 = Line::new(Bounds::new(
        0.0,
        10.0,
        PAGE_A4_WIDTH - 1.0,
        PAGE_A4_HEIGHT - 11.0,
    ));
    line2.add_style(Style::BorderWidth(2.0));
    line2.add_style(Style::BorderColor(RgbColor {
        r: 200,
        g: 200,
        b: 200,
    }));
    line2.add_style(Style::BorderStyle(BorderStyle::Dash(2)));
    container1.add_block(BlockType::Line(line2));

    // NOTE: Block Test3 - Text
    let text_size1 = measure_text(
        &String::from("HELLO WORLD"),
        48.0,
        &String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
    );
    let mut text1 = Text::new(
        String::from("HELLO WORLD"),
        48.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        Some(Bounds::new(
            text_size1.width,
            text_size1.height,
            1.0,
            PAGE_A4_HEIGHT - text_size1.height - 1.0,
        )),
    );
    text1.add_style(Style::BorderStyle(BorderStyle::Dash(2)));
    text1.add_style(Style::BorderColor(RgbColor { r: 0, g: 0, b: 255 }));
    text1.add_style(Style::BorderWidth(1.0));
    text1.add_style(Style::TextFillColor(RgbColor { r: 0, g: 255, b: 0 }));
    text1.add_style(Style::TextOutlineColor(RgbColor {
        r: 255,
        g: 255,
        b: 0,
    }));
    text1.add_style(Style::TextStyle(TextStyle::FillStroke));
    text1.add_style(Style::TextOutlineStyle(TextOutlineStyle::Dash(2)));
    container1.add_block(BlockType::Text(text1));

    let text_size2 = measure_text(
        &String::from("------\nHELLO WORLD\nGOOD NIGHT :)\n------"),
        20.0,
        &String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
    );
    let mut text2 = Text::new(
        String::from("------\nHELLO WORLD\nGOOD NIGHT :)\n------"), // FIXME: 改行を反映するには!?
        20.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        Some(Bounds::new(
            text_size2.width,
            text_size2.height,
            30.0,
            30.0,
        )),
    );
    text2.add_style(Style::BorderColor(RgbColor {
        r: 200,
        g: 200,
        b: 200,
    }));
    text2.add_style(Style::BorderWidth(1.0));
    container1.add_block(BlockType::Text(text2));

    // NOTE: Block Test4 - Image
    let image = image::io::Reader::open("assets/images/channel.png")
        .unwrap()
        .decode()
        .unwrap();
    let (image_width, image_height) = image.dimensions();
    let mut image = Image::new(
        String::from("assets/images/channel.png"),
        Some(Bounds::new(
            px_to_mm(image_width as f32),
            px_to_mm(image_height as f32),
            PAGE_A4_WIDTH - px_to_mm(image_width as f32) - 1.0,
            1.0,
        )),
    );
    image.add_style(Style::BorderWidth(1.0));
    image.add_style(Style::BorderColor(RgbColor {
        r: 200,
        g: 0,
        b: 200,
    }));
    image.add_style(Style::BorderStyle(BorderStyle::Solid));
    container1.add_block(BlockType::Image(image));

    // NOTE: Block Test5 - BlockContainer
    let mut block_container = BlockContainer::new(Some(Bounds {
        point: Some(Point {
            x: PAGE_A4_WIDTH - 50.0 - 1.0,
            y: 50.0 + 1.0,
        }),
        size: Some(Size {
            width: 50.0,
            height: 50.0,
        }),
    }));
    let mut rectangle2 = Rectangle::new(Some(Bounds::new(
        50.0, 50.0, 0.0, // NOTE: BlockContainer からの座標
        0.0, // NOTE: BlockContainer からの座標
    )));
    rectangle2.add_style(Style::BackgroundColor(RgbColor {
        r: 200,
        g: 255,
        b: 255,
    }));
    rectangle2.add_style(Style::BorderColor(RgbColor {
        r: 0,
        g: 200,
        b: 255,
    }));
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
            text_size2.width,
            text_size2.height,
            1.0,
            1.0,
        )),
    );
    block_container.add_block(BlockType::Text(text2));
    container1.add_block(BlockType::Container(block_container));

    doc.add_container(container1);

    // 2ページ目
    let mut container2 = Container::new();

    let image2 = Image::new(String::from("assets/images/channel.png"), None);
    container2.add_block(BlockType::Image(image2));

    let image3 = Image::new(String::from("assets/images/channel.png"), None);
    container2.add_block(BlockType::Image(image3));

    let mut block_container2 = BlockContainer::new(None);
    let image4 = Image::new(String::from("assets/images/channel.png"), None);
    let image5 = Image::new(String::from("assets/images/channel.png"), None);
    block_container2.set_direction(Direction::Horizontal);
    block_container2.add_block(BlockType::Image(image4));
    block_container2.add_block(BlockType::Image(image5));
    container2.add_block(BlockType::Container(block_container2));

    let b1 = Bounds::new(10.0, 10.0, 50.0, 50.0);
    let mut r1 = Rectangle::new(Some(b1.clone()));
    r1.add_style(Style::BackgroundColor(RgbColor{ r: 255, g: 200, b: 200, }));

    let b2 = Bounds::new(10.0, 10.0, 55.0, 55.0);
    let mut r2 = Rectangle::new(Some(b2.clone()));
    r2.add_style(Style::BackgroundColor(RgbColor{ r: 200, g: 255, b: 200, }));

    let b3 = b2.union(&b1);
    let mut r3 = Rectangle::new(Some(b3.clone()));
    r3.add_style(Style::BackgroundColor(RgbColor{ r: 200, g: 200, b: 255, }));

    container2.add_block(BlockType::Rectangle(r3));
    container2.add_block(BlockType::Rectangle(r1));
    container2.add_block(BlockType::Rectangle(r2));

    let text3 = Text::new(
        String::from("GOOD AFTERNOON1"),
        20.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        None,
    );
    let text4 = Text::new(
        String::from("GOOD AFTERNOON2"),
        20.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        None,
    );
    container2.add_block(BlockType::Text(text3));
    container2.add_block(BlockType::Text(text4));

    let text5 = Text::new(
        String::from("GOOD AFTERNOON3"),
        20.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        None,
    );
    let text6 = Text::new(
        String::from("GOOD AFTERNOON4"),
        20.0,
        String::from("assets/fonts/NotoSansJP-VariableFont_wght.ttf"),
        None,
    );
    let mut block_container3 = BlockContainer::new(None);
    block_container3.add_block(BlockType::Text(text5));
    block_container3.add_block(BlockType::Text(text6));
    container2.add_block(BlockType::Container(block_container3));

    doc.add_container(container2);

    doc
}

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
