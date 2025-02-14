use crate::block_document::block::BlockType;
use crate::block_document::bounds::Bounds as BlockBounds;
use crate::block_document::document::{Document as BlockDocument, DPI as BlockDPI};
use crate::block_document::image::Image as BlockImage;
use crate::block_document::line::Line as BlockLine;
use crate::block_document::rectangle::Rectangle as BlockRectangle;
use crate::block_document::text::Text as BlockText;
use image::GenericImageView;
use printpdf::{
    Color, Image, ImageTransform, Line, LineDashPattern, Mm, PdfDocument,
    PdfDocumentReference, PdfPageIndex, Point, Rect, Rgb,
};
use std::any::{Any, TypeId};
use std::fs::File;
use std::io::BufWriter;

pub fn save(block_document: BlockDocument, file: File) {
    let (doc, mut page_index, mut _layer_index) = PdfDocument::new(
        block_document.title.clone(),
        Mm(block_document.width),
        Mm(block_document.height),
        "Layer 1",
    );
    // let _layer = doc.get_page(page_index).get_layer(layer_index);

    // TODO: Bounds を調整
    // ...
    // ...
    // ...

    // TODO: 描画（Bounds が確定している）
    let mut count = 0;
    for container in block_document.containers.iter() {
        if count > 0 {
            (page_index, _layer_index) = doc.add_page(
                Mm(block_document.width),
                Mm(block_document.height),
                "Layer 1",
            );
        }

        count += 1;

        for block in container.blocks.iter() {
            match block {
                BlockType::Line(line) => {
                    // println!("- This is a BlockLine!");
                    // line 変数を使って BlockLine の情報にアクセスできます
                    // println!("  - bounds: {:?}", line.bounds); // 例えば、bounds にアクセス
                }
                BlockType::Rectangle(rectangle) => {
                    // println!("- This is a Rectangle!");
                    // rectangle 変数を使って Rectangle の情報にアクセスできます
                    // println!("  - bounds: {:?}", rectangle.bounds); // 例えば、bounds にアクセス

                    write_rectangle(
                        &doc,
                        page_index,
                        &rectangle,
                        BlockBounds {
                            width: Some(block_document.width),
                            height: Some(block_document.height),
                            x: Some(0.0),
                            y: Some(0.0),
                        },
                    )
                }
                BlockType::Text(text) => {
                    // println!("- This is a Text!");
                    // text 変数を使って Text の情報にアクセスできます
                    // println!("  - bounds: {:?}", text.bounds); // 例えば、bounds にアクセス

                    write_text(
                        &doc,
                        page_index,
                        &text,
                        BlockBounds {
                            width: Some(block_document.width),
                            height: Some(block_document.height),
                            x: Some(0.0),
                            y: Some(0.0),
                        },
                    );
                }
                BlockType::Image(image) => {
                    // println!("- This is an Image!");
                    // image 変数を使って Image の情報にアクセスできます
                    // println!("  - bounds: {:?}", image.bounds); // 例えば、bounds にアクセス

                    write_image(
                        &doc,
                        page_index,
                        &image,
                        BlockBounds {
                            width: Some(block_document.width),
                            height: Some(block_document.height),
                            x: Some(0.0),
                            y: Some(0.0),
                        },
                    );
                }
            }
        }
    }

    doc.save(&mut BufWriter::new(file)).unwrap();
}

fn write_rectangle(
    doc: &PdfDocumentReference,
    page_index: PdfPageIndex,
    block_rectangle: &BlockRectangle,
    block_bounds: BlockBounds,
) {
    if let Some(bounds) = &block_rectangle.bounds {
        if let (Some(_x), Some(_y)) = (bounds.x, bounds.y) {
            let lb_bounds = bounds.transform(block_bounds);
            // println!("  - lb_bounds: {:?}", lb_bounds);
            // println!("  - lb_bounds.min_x: {:?}", lb_bounds.min_x());
            // println!("  - lb_bounds.max_y: {:?}", lb_bounds.max_y());
            // println!("  - lb_bounds.max_x {:?}", lb_bounds.max_x());
            // println!("  - lb_bounds.min_y: {:?}", lb_bounds.min_y());

            let layer = doc.get_page(page_index).add_layer("Layer");
            layer.set_fill_color(Color::Rgb(Rgb {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                icc_profile: None,
            }));
            layer.add_rect(Rect::new(
                Mm(lb_bounds.min_x()), // 左上X
                Mm(lb_bounds.max_y()), // 左上Y
                Mm(lb_bounds.max_x()), // 右下X
                Mm(lb_bounds.min_y()), // 右下Y
            ));
        }
    }
}

fn write_text(
    doc: &PdfDocumentReference,
    page_index: PdfPageIndex,
    block_text: &BlockText,
    block_bounds: BlockBounds,
) {
    if let Some(bounds) = &block_text.bounds {
        if let (Some(_x), Some(_y)) = (bounds.x, bounds.y) {
            let lb_bounds = bounds.transform(block_bounds);
            // println!("  - lb_bounds: {:?}", lb_bounds);

            let layer = doc.get_page(page_index).add_layer("Layer");
            // let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
            let font = doc
                .add_external_font(File::open(&block_text.font_path).unwrap())
                .unwrap();
            layer.use_text(
                block_text.text.clone(),
                block_text.font_size,
                Mm(lb_bounds.min_x()),
                Mm(lb_bounds.min_y()),
                &font,
            );

            // DEBUG
            layer.set_outline_color(Color::Rgb(Rgb {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                icc_profile: None,
            }));
            layer.set_line_dash_pattern(LineDashPattern {
                dash_1: Some(2),
                ..Default::default()
            });
            layer.set_outline_thickness(1.0);
            layer.add_line(Line {
                points: vec![
                    (Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.min_y())), false),
                    (Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.min_y())), false),
                    (Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.max_y())), false),
                    (Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.max_y())), false),
                    (Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.min_y())), false),
                ],
                is_closed: false,
            });
            // DEBUG
        }
    }
}

fn write_image(
    doc: &PdfDocumentReference,
    page_index: PdfPageIndex,
    block_image: &BlockImage,
    block_bounds: BlockBounds,
) {
    if let Some(bounds) = &block_image.bounds {
        if let (Some(_x), Some(_y)) = (bounds.x, bounds.y) {
            let lb_bounds = bounds.transform(block_bounds);
            // println!("  - lb_bounds: {:?}", lb_bounds);

            let layer = doc.get_page(page_index).add_layer("Layer");

            let image = image::io::Reader::open(&block_image.location)
                .unwrap()
                .decode()
                .unwrap();
            // let (image_width, image_height) = image.dimensions();

            let pdf_image = Image::from_dynamic_image(&image);

            let transform = ImageTransform {
                translate_x: Option::from(Mm(lb_bounds.min_x())), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
                translate_y: Option::from(Mm(lb_bounds.min_y())), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
                scale_x: Some(1.0),                               // NOTE: 水平方向の拡縮小
                scale_y: Some(1.0),                               // NOTE: 垂直方向の拡縮小
                rotate: None,                                     // 回転なし
                dpi: Option::from(BlockDPI),
            };

            pdf_image.add_to_layer(layer, transform);
        }
    }
}

// pub fn dummy(block_document: BlockDocument, file: File) {
//     let (doc, page_index, layer_index) = PdfDocument::new(
//         block_document.title,
//         Mm(block_document.width),
//         Mm(block_document.height),
//         "Layer 1",
//     );
//     let layer = doc.get_page(page_index).get_layer(layer_index);
//
//     // --------------------
//     // フォントを指定してテキストを描画
//     // let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
//     let font = doc.add_external_font(File::open("assets/fonts/NotoSansJP-VariableFont_wght.ttf").unwrap()).unwrap();
//     layer.use_text("HELLO WORLD", 48.0, Mm(10.0), Mm(20.0), &font);
//
//     // --------------------
//     let layer2 = doc.get_page(page_index).add_layer("Layer 2");
//     // 画像を読み込む
//     let image = image::io::Reader::open("assets/images/channel.png")
//         .unwrap()
//         .decode()
//         .unwrap();
//     let (image_width, image_height) = image.dimensions();
//
//     // `printpdf::Image` に変換
//     let pdf_image = Image::from_dynamic_image(&image);
//
//     // 変換行列（スケールと位置の指定）
//     let dpi = 300.0;
//     let transform = ImageTransform {
//         translate_x: Option::from(Mm(10.0)), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
//         translate_y: Option::from(Mm(40.0)), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
//         scale_x: Option::from(image_width as f32 / dpi), // NOTE: ミリメートル単位 で指定する。
//         scale_y: Option::from(image_height as f32 / dpi), // NOTE: ミリメートル単位 で指定する。
//         rotate: None,                        // 回転なし
//         dpi: Option::from(dpi),
//     };
//
//     // 画像を PDF に描画
//     pdf_image.add_to_layer(layer2, transform);
//
//     // --------------------
//     let layer3 = doc.get_page(page_index).add_layer("Layer 3");
//     layer3.set_fill_color(Color::Rgb(Rgb {
//         r: 255.0,
//         g: 0.0,
//         b: 0.0,
//         icc_profile: None,
//     }));
//     layer3.add_rect(Rect::new(
//         Mm(10.0),  // 左上X
//         Mm(287.0), // 左上Y
//         Mm(20.0),  // 右下X
//         Mm(277.0), // 右下Y
//     ));
//
//     // --------------------
//     let layer4 = doc.get_page(page_index).add_layer("Layer 4");
//     layer4.set_outline_color(Color::Rgb(Rgb {
//         r: 0.0,
//         g: 255.0,
//         b: 0.0,
//         icc_profile: None,
//     }));
//     layer4.set_outline_thickness(1.0);
//     layer4.add_line(Line {
//         points: vec![
//             (Point::new(Mm(10.0), Mm(260.0)), false),
//             (Point::new(Mm(20.0), Mm(260.0)), false),
//         ],
//         is_closed: false,
//     });
//     layer4.set_line_dash_pattern(LineDashPattern {
//         dash_1: Some(2),
//         ..Default::default()
//     });
//     layer4.set_outline_thickness(5.0);
//     layer4.add_line(Line {
//         points: vec![
//             (Point::new(Mm(10.0), Mm(255.0)), false),
//             (Point::new(Mm(20.0), Mm(255.0)), false),
//         ],
//         is_closed: false,
//     });
//
//     // --------------------
//     // 保存
//     doc.save(&mut BufWriter::new(file)).unwrap();
// }
