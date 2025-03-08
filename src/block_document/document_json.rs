use crate::block_document::block::BlockType;
use crate::block_document::block_container::BlockContainer;
use crate::block_document::container::Container;
use crate::block_document::direction::Direction;
use crate::block_document::document::{Document, px_to_mm};
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
use serde_json::{Map, Value};
use std::fs::read_to_string;
use std::process::exit;

const PAGE_A4_WIDTH: f32 = 210.0;
const PAGE_A4_HEIGHT: f32 = 297.0;

const JSON_SCHEMA_BYTES: &'static [u8] = include_bytes!("../../schema/schema.json");

// NOTE: BlockDocument の座標基準は左上（printpdf は左下）
pub fn parse(json_path: &str) -> Document {
    let json_string = read_to_string(json_path).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let schema = serde_json::from_slice(JSON_SCHEMA_BYTES).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();

    if !validator.validate(&json).is_ok() {
        eprintln!("Invalid schema");
        for error in validator.iter_errors(&json) {
            eprintln!("Error: {error}");
            eprintln!("Location: {}", error.instance_path);
        }
        exit(1);
    }

    // TODO: JSON を Document に展開する
    let mut doc = Document::new(
        json["document"]["title"].as_str().unwrap().to_string(),
        Size {
            width: json["document"]["width"].as_f64().unwrap() as f32,
            height: json["document"]["height"].as_f64().unwrap() as f32,
        },
        json["document"]["font_path"].as_str().unwrap().to_string(),
    );

    json["document"]["pages"]
        .as_array()
        .unwrap()
        .iter()
        .for_each(|page_json| {
            // println!("{:#?}", json_page);

            let mut container = Container::new();

            page_json["objects"]
                .as_array()
                .unwrap()
                .iter()
                .for_each(|object_json| {
                    // println!("{:#?}", object_json);

                    match object_json["type"].as_str().unwrap() {
                        "text" => {
                            let point_x = object_json["bounds"]["point"]["x"].as_f64();
                            let point_y = object_json["bounds"]["point"]["y"].as_f64();
                            let size_w = object_json["bounds"]["size"]["w"].as_f64();
                            let size_h = object_json["bounds"]["size"]["h"].as_f64();

                            let bounds = match (point_x, point_y, size_w, size_h) {
                                (Some(x), Some(y), Some(w), Some(h)) => {
                                    Some(Bounds::new(w as f32, h as f32, x as f32, y as f32))
                                }
                                (Some(x), Some(y), None, None) => Some(Bounds {
                                    point: Some(Point {
                                        x: x as f32,
                                        y: y as f32,
                                    }),
                                    size: None,
                                }),
                                (None, None, Some(w), Some(h)) => Some(Bounds {
                                    point: None,
                                    size: Some(Size {
                                        width: w as f32,
                                        height: h as f32,
                                    }),
                                }),
                                _ => None,
                            };

                            let font_path: Option<String> = object_json["font_path"]
                                .as_str()
                                .filter(|s| !s.is_empty())
                                .map(|s| s.to_string());

                            let mut text = Text::new(
                                object_json["text"].as_str().unwrap().to_string(),
                                object_json["font_size"].as_f64().unwrap() as f32,
                                font_path,
                                bounds,
                            );

                            // TODO: スタイル
                            let style_json = object_json["style"].as_object();
                            match style_json {
                                Some(style_json) => {
                                    println!("style_json: {:?}", style_json);

                                    // NOTE: border_color
                                    if style_json.contains_key("border_color") {
                                        // if let Some(border_color) = parse_border_color_style(style_json);
                                        text.add_style(Style::BorderColor(RgbColor {
                                            r: style_json["border_color"]["red"].as_u64().unwrap()
                                                as u8,
                                            g: style_json["border_color"]["green"].as_u64().unwrap()
                                                as u8,
                                            b: style_json["border_color"]["blue"].as_u64().unwrap()
                                                as u8,
                                        }));
                                    }

                                    // NOTE: border_style
                                    if style_json.contains_key("border_style") {
                                        let border_style_json =
                                            style_json["border_style"].as_object().unwrap();
                                        let line_style =
                                            border_style_json["line_style"].as_str().unwrap();

                                        if let Some(border_style) = match line_style {
                                            "solid" => Some(BorderStyle::Solid),
                                            "dash" => {
                                                let dash_1 =
                                                    border_style_json["dash_1"].as_i64().unwrap();
                                                Some(BorderStyle::Dash(dash_1))
                                            }
                                            _ => None,
                                        } {
                                            text.add_style(Style::BorderStyle(border_style));
                                        }
                                    }

                                    // NOTE: border_width
                                    if style_json.contains_key("border_width") {
                                        text.add_style(Style::BorderWidth(
                                            style_json["border_width"]["width"].as_f64().unwrap()
                                                as f32,
                                        ));
                                    }

                                    // NOTE: text_fill_color
                                    if style_json.contains_key("text_fill_color") {
                                        text.add_style(Style::TextFillColor(RgbColor {
                                            r: style_json["text_fill_color"]["red"]
                                                .as_u64()
                                                .unwrap()
                                                as u8,
                                            g: style_json["text_fill_color"]["green"]
                                                .as_u64()
                                                .unwrap()
                                                as u8,
                                            b: style_json["text_fill_color"]["blue"]
                                                .as_u64()
                                                .unwrap()
                                                as u8,
                                        }));
                                    }

                                    // NOTE: text_outline_color
                                    if style_json.contains_key("text_outline_color") {
                                        text.add_style(Style::TextOutlineColor(RgbColor {
                                            r: style_json["text_outline_color"]["red"]
                                                .as_u64()
                                                .unwrap()
                                                as u8,
                                            g: style_json["text_outline_color"]["green"]
                                                .as_u64()
                                                .unwrap()
                                                as u8,
                                            b: style_json["text_outline_color"]["blue"]
                                                .as_u64()
                                                .unwrap()
                                                as u8,
                                        }));
                                    }

                                    // NOTE: text_outline_style
                                    if style_json.contains_key("text_outline_style") {
                                        let text_outline_style_json =
                                            style_json["text_outline_style"].as_object().unwrap();
                                        let line_style =
                                            text_outline_style_json["line_style"].as_str().unwrap();

                                        if let Some(text_outline_style) = match line_style {
                                            "solid" => Some(TextOutlineStyle::Solid),
                                            "dash" => {
                                                let dash_1 = text_outline_style_json["dash_1"]
                                                    .as_i64()
                                                    .unwrap();
                                                Some(TextOutlineStyle::Dash(dash_1))
                                            }
                                            _ => None,
                                        } {
                                            text.add_style(Style::TextOutlineStyle(
                                                text_outline_style,
                                            ));
                                        }
                                    }

                                    // NOTE: text_style
                                    if style_json.contains_key("text_style") {
                                        let line_style = style_json["text_style"]["line_style"]
                                            .as_str()
                                            .unwrap();
                                        match line_style {
                                            "fill" => {
                                                text.add_style(Style::TextStyle(TextStyle::Fill));
                                            }
                                            "stroke" => {
                                                text.add_style(Style::TextStyle(TextStyle::Stroke));
                                            }
                                            "fill_stroke" => {
                                                text.add_style(Style::TextStyle(
                                                    TextStyle::FillStroke,
                                                ));
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }

                            container.add_block(BlockType::Text(text));
                        }
                        _ => {}
                    }
                });

            doc.add_container(container);
        });

    doc

    // <editor-fold desc="">
    // let mut doc = Document::new(
    //     String::from("HELLO"),
    //     Size {
    //         width: PAGE_A4_WIDTH,
    //         height: PAGE_A4_HEIGHT,
    //     },
    //     String::from(font_path),
    // );
    //
    // // NOTE: 1ページ目
    // let mut container1 = Container::new();
    //
    // // NOTE: Block Test1 - Rectangle
    // let mut rectangle1 = Rectangle::new(Some(Bounds::new(10.0, 10.0, 1.0, 1.0)));
    // rectangle1.add_style(Style::BackgroundColor(RgbColor {
    //     r: 200,
    //     g: 200,
    //     b: 200,
    // }));
    // container1.add_block(BlockType::Rectangle(rectangle1));
    //
    // let mut rectangle3 = Rectangle::new(Some(Bounds::new(10.0, 10.0, 1.0, 1.0)));
    // rectangle3.add_style(Style::BackgroundColor(RgbColor {
    //     r: 230,
    //     g: 230,
    //     b: 230,
    // }));
    // rectangle3.add_style(Style::Space(Space {
    //     top: 2.0,
    //     right: 2.0,
    //     bottom: 2.0,
    //     left: 2.0,
    // }));
    // container1.add_block(BlockType::Rectangle(rectangle3));
    //
    // // NOTE: Block Test2 - Line
    // let line1 = Line::new(Bounds::new(
    //     10.0,
    //     0.0,
    //     PAGE_A4_WIDTH - 11.0,
    //     PAGE_A4_HEIGHT - 1.0,
    // ));
    // container1.add_block(BlockType::Line(line1));
    // let mut line2 = Line::new(Bounds::new(
    //     0.0,
    //     10.0,
    //     PAGE_A4_WIDTH - 1.0,
    //     PAGE_A4_HEIGHT - 11.0,
    // ));
    // line2.add_style(Style::BorderWidth(2.0));
    // line2.add_style(Style::BorderColor(RgbColor {
    //     r: 200,
    //     g: 200,
    //     b: 200,
    // }));
    // line2.add_style(Style::BorderStyle(BorderStyle::Dash(2)));
    // container1.add_block(BlockType::Line(line2));
    //
    // // NOTE: Block Test3 - Text
    // let text_size1 = measure_text(
    //     &String::from("HELLO WORLD"),
    //     48.0,
    //     &doc.font_path,
    // );
    // let mut text1 = Text::new(
    //     String::from("HELLO WORLD"),
    //     48.0,
    //     None,
    //     Some(Bounds::new(
    //         text_size1.width,
    //         text_size1.height,
    //         1.0,
    //         PAGE_A4_HEIGHT - text_size1.height - 1.0,
    //     )),
    // );
    // text1.add_style(Style::BorderStyle(BorderStyle::Dash(2)));
    // text1.add_style(Style::BorderColor(RgbColor { r: 0, g: 0, b: 255 }));
    // text1.add_style(Style::BorderWidth(1.0));
    // text1.add_style(Style::TextFillColor(RgbColor { r: 0, g: 255, b: 0 }));
    // text1.add_style(Style::TextOutlineColor(RgbColor {
    //     r: 255,
    //     g: 255,
    //     b: 0,
    // }));
    // text1.add_style(Style::TextStyle(TextStyle::FillStroke));
    // text1.add_style(Style::TextOutlineStyle(TextOutlineStyle::Dash(2)));
    // container1.add_block(BlockType::Text(text1));
    //
    // let text_size2 = measure_text(
    //     &String::from("------\nHELLO WORLD\nGOOD NIGHT :)\n------"),
    //     20.0,
    //     &doc.font_path,
    // );
    // let mut text2 = Text::new(
    //     String::from("------\nHELLO WORLD\nGOOD NIGHT :)\n------"),
    //     20.0,
    //     None,
    //     Some(Bounds::new(
    //         text_size2.width,
    //         text_size2.height,
    //         30.0,
    //         30.0,
    //     )),
    // );
    // text2.add_style(Style::BorderColor(RgbColor {
    //     r: 200,
    //     g: 200,
    //     b: 200,
    // }));
    // text2.add_style(Style::BorderWidth(1.0));
    // container1.add_block(BlockType::Text(text2));
    //
    // // NOTE: Block Test4 - Image
    // let image = image::io::Reader::open("assets/images/channel.png")
    //     .unwrap()
    //     .decode()
    //     .unwrap();
    // let (image_width, image_height) = image.dimensions();
    // let mut image = Image::new(
    //     String::from("assets/images/channel.png"),
    //     Some(Bounds::new(
    //         px_to_mm(image_width as f32),
    //         px_to_mm(image_height as f32),
    //         PAGE_A4_WIDTH - px_to_mm(image_width as f32) - 1.0,
    //         1.0,
    //     )),
    // );
    // image.add_style(Style::BorderWidth(1.0));
    // image.add_style(Style::BorderColor(RgbColor {
    //     r: 200,
    //     g: 0,
    //     b: 200,
    // }));
    // image.add_style(Style::BorderStyle(BorderStyle::Solid));
    // container1.add_block(BlockType::Image(image));
    //
    // // NOTE: Block Test5 - BlockContainer
    // let mut block_container = BlockContainer::new(Some(Bounds {
    //     point: Some(Point {
    //         x: PAGE_A4_WIDTH - 50.0 - 1.0,
    //         y: 50.0 + 1.0,
    //     }),
    //     size: Some(Size {
    //         width: 50.0,
    //         height: 50.0,
    //     }),
    // }));
    // let mut rectangle2 = Rectangle::new(Some(Bounds::new(
    //     50.0, 50.0, 0.0, // NOTE: BlockContainer からの座標
    //     0.0, // NOTE: BlockContainer からの座標
    // )));
    // rectangle2.add_style(Style::BackgroundColor(RgbColor {
    //     r: 200,
    //     g: 255,
    //     b: 255,
    // }));
    // rectangle2.add_style(Style::BorderColor(RgbColor {
    //     r: 0,
    //     g: 200,
    //     b: 255,
    // }));
    // rectangle2.add_style(Style::BorderWidth(1.0));
    // rectangle2.add_style(Style::BorderStyle(BorderStyle::Dash(2)));
    // block_container.add_block(BlockType::Rectangle(rectangle2));
    // let text_size2 = measure_text(
    //     &String::from("Hi!!"),
    //     20.0,
    //     &doc.font_path,
    // );
    // let text2 = Text::new(
    //     String::from("Hi!!"),
    //     20.0,
    //     None,
    //     Some(Bounds::new(
    //         text_size2.width,
    //         text_size2.height,
    //         1.0,
    //         1.0,
    //     )),
    // );
    // block_container.add_block(BlockType::Text(text2));
    // container1.add_block(BlockType::Container(block_container));
    //
    // doc.add_container(container1);
    //
    // // 2ページ目
    // let mut container2 = Container::new();
    //
    // let image2 = Image::new(String::from("assets/images/channel.png"), None);
    // container2.add_block(BlockType::Image(image2));
    //
    // let image3 = Image::new(String::from("assets/images/channel.png"), None);
    // container2.add_block(BlockType::Image(image3));
    //
    // let mut block_container2 = BlockContainer::new(None);
    // let image4 = Image::new(String::from("assets/images/channel.png"), None);
    // let image5 = Image::new(String::from("assets/images/channel.png"), None);
    // block_container2.set_direction(Direction::Horizontal);
    // block_container2.add_block(BlockType::Image(image4));
    // block_container2.add_block(BlockType::Image(image5));
    // container2.add_block(BlockType::Container(block_container2));
    //
    // let b1 = Bounds::new(10.0, 10.0, 50.0, 50.0);
    // let mut r1 = Rectangle::new(Some(b1.clone()));
    // r1.add_style(Style::BackgroundColor(RgbColor{ r: 255, g: 200, b: 200, }));
    //
    // let b2 = Bounds::new(10.0, 10.0, 55.0, 55.0);
    // let mut r2 = Rectangle::new(Some(b2.clone()));
    // r2.add_style(Style::BackgroundColor(RgbColor{ r: 200, g: 255, b: 200, }));
    //
    // let b3 = b2.union(&b1);
    // let mut r3 = Rectangle::new(Some(b3.clone()));
    // r3.add_style(Style::BackgroundColor(RgbColor{ r: 200, g: 200, b: 255, }));
    //
    // container2.add_block(BlockType::Rectangle(r3));
    // container2.add_block(BlockType::Rectangle(r1));
    // container2.add_block(BlockType::Rectangle(r2));
    //
    // let text3 = Text::new(
    //     String::from("GOOD AFTERNOON1"),
    //     20.0,
    //     None,
    //     None,
    // );
    // let text4 = Text::new(
    //     String::from("GOOD AFTERNOON2"),
    //     20.0,
    //     None,
    //     None,
    // );
    // container2.add_block(BlockType::Text(text3));
    // container2.add_block(BlockType::Text(text4));
    //
    // let text5 = Text::new(
    //     String::from("GOOD AFTERNOON3"),
    //     20.0,
    //     None,
    //     None,
    // );
    // let text6 = Text::new(
    //     String::from("GOOD AFTERNOON4"),
    //     20.0,
    //     None,
    //     None,
    // );
    // let mut block_container3 = BlockContainer::new(None);
    // block_container3.add_block(BlockType::Text(text5));
    // block_container3.add_block(BlockType::Text(text6));
    // container2.add_block(BlockType::Container(block_container3));
    //
    // doc.add_container(container2);
    //
    // doc
    // </editor-fold>
}

// fn parse_border_color_style(style_json: &Value) -> Option<Style> {
//     Some(Style::BorderColor(RgbColor {
//         r: style_json["border_color"]["red"].as_u64().unwrap() as u8,
//         g: style_json["border_color"]["green"].as_u64().unwrap() as u8,
//         b: style_json["border_color"]["blue"].as_u64().unwrap() as u8,
//     }));
// }

// TODO: Document 構造体を JSON ファイルに出力
// pub fn toJson(document: Document) {
// }
