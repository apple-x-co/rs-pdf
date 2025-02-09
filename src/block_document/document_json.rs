use crate::block_document::bounds::Bounds;
use crate::block_document::rectangle::Rectangle;
use crate::block_document::container::Container;
use crate::block_document::document::Document;
use crate::block_document::image::Image;
use crate::block_document::line::Line;
use crate::block_document::text::Text;

// TODO: JSON ファイルをパースして Document 構造体を返す
// NOTE: BlockDocument の座標基準は左上（printpdf は左下）
pub fn parse() -> Document {
    let mut doc = Document::new(String::from("HELLO"), 210.0, 297.0);

    let mut container = Container::new();

    // Block Test1
    let rectangle = Rectangle::new(Some(Bounds {
        width: Some(10.0),
        height: Some(10.0),
        x: Some(10.0),
        y: Some(10.0), // 297.0 - 287.0
    }));
    container.add_block(Box::new(rectangle));

    // Block Test2
    let line = Line::new(Bounds {
        width: Some(10.0),
        height: Some(1.0),
        x: Some(10.0),
        y: Some(37.0), // 297.0 - 255.0
    });
    container.add_block(Box::new(line));

    // Block Test3
    let text = Text::new(String::from("HELLO WORLD"), Some(Bounds {
        width: None,
        height: None,
        x: Some(10.0),
        y: Some(287.0), // 297.0 - 10.0
    }));
    container.add_block(Box::new(text));

    // Block Test4
    let image = Image::new(String::from("assets/channel.png"), Some(Bounds {
        width: None,
        height: None,
        x: Some(10.0),
        y: Some(257.0), // 297.0 - 40.0
    }));
    container.add_block(Box::new(image));

    doc.add_container(container);

    doc
}

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
