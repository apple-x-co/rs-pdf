use crate::block_document::block::BlockType;
use crate::block_document::block_container::BlockContainer;
use crate::block_document::page::Page;
use crate::block_document::direction::Direction;
use crate::block_document::document::Document;
use crate::block_document::flexible_container::FlexibleContainer;
use crate::block_document::geometry::{GeoRect, GeoPoint, GeoSize};
use crate::block_document::image::Image;
use crate::block_document::line::Line;
use crate::block_document::rectangle::Rectangle;
use crate::block_document::style::{Alignment, BorderStyle, HorizontalAlignment, RgbColor, Space, Style, TextOutlineStyle, TextStyle, TextWrap, TextWrapMode, TextOverflow, VerticalAlignment};
use crate::block_document::text::Text;
use serde_json::Value;
use std::fs::read_to_string;
use std::process::exit;
use crate::block_document::flexible_item::FlexibleItem;
use crate::block_document::wrapper::Wrapper;
// const PAGE_A4_WIDTH: f32 = 210.0;
// const PAGE_A4_HEIGHT: f32 = 297.0;

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

    let mut doc = Document::new(
        json["document"]["title"].as_str().unwrap().to_string(),
        GeoSize {
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
            let mut page = Page::new();

            let auto_pagination = page_json["auto_pagination"]
                .as_bool()
                .unwrap_or(false);
            page.set_auto_pagination(auto_pagination);

            page_json["objects"]
                .as_array()
                .unwrap()
                .iter()
                .for_each(|object_json| {
                    if let Some(object) = parse_object(object_json) {
                        page.add_block(object);
                    }
                });

            doc.add_page(page);
        });

    doc
}

fn parse_object(object_json: &Value) -> Option<BlockType> {
    match object_json["type"].as_str().unwrap() {
        "text" => {
            let frame = object_json["frame"]
                .as_object()
                .map(|_| parse_frame(&object_json["frame"]));

            let font_path: Option<String> = object_json["font_path"]
                .as_str()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());

            let mut text = Text::new(
                object_json["text"].as_str().unwrap().to_string(),
                object_json["font_size"].as_f64().unwrap() as f32,
                font_path,
                frame,
            );

            let style = &object_json["style"];

            if style.is_null() {
                return Some(BlockType::Text(text));
            }

            if !style["alignment"].is_null() {
                if let Some(alignment) = parse_alignment(&style["alignment"]) {
                    text.add_style(alignment);
                }
            }

            if !style["border_color"].is_null() {
                if let Some(border_color) = parse_border_color(&style["border_color"]) {
                    text.add_style(border_color);
                }
            }

            if !style["border_style"].is_null() {
                if let Some(border_style) = parse_border_style(&style["border_style"]) {
                    text.add_style(border_style);
                }
            }

            if !style["border_width"].is_null() {
                if let Some(border_width) = parse_border_width(&style["border_width"]) {
                    text.add_style(border_width);
                }
            }

            if !style["text_fill_color"].is_null() {
                if let Some(text_fill_color) = parse_text_fill_color(&style["text_fill_color"]) {
                    text.add_style(text_fill_color);
                }
            }

            if !style["text_outline_color"].is_null() {
                if let Some(text_outline_color) =
                    parse_text_outline_color(&style["text_outline_color"])
                {
                    text.add_style(text_outline_color);
                }
            }

            if !style["text_outline_style"].is_null() {
                if let Some(text_outline_style) =
                    parse_text_outline_style(&style["text_outline_style"])
                {
                    text.add_style(text_outline_style);
                }
            }

            if !style["text_style"].is_null() {
                if let Some(text_style) = parse_text_style(&style["text_style"]) {
                    text.add_style(text_style);
                }
            }

            if !style["text_wrap"].is_null() {
                if let Some(text_wrap) = parse_text_wrap(&style["text_wrap"]) {
                    text.add_style(text_wrap);
                }
            }

            Some(BlockType::Text(text))
        }
        "image" => {
            let image_path = object_json["path"].as_str().unwrap().to_string();

            let frame = object_json["frame"]
                .as_object()
                .map(|_| parse_frame(&object_json["frame"]));

            let mut image = Image::new(image_path, frame);

            let style = &object_json["style"];

            if style.is_null() {
                return Some(BlockType::Image(image));
            }

            if !style["border_color"].is_null() {
                if let Some(border_color) = parse_border_color(&style["border_color"]) {
                    image.add_style(border_color);
                }
            }

            if !style["border_style"].is_null() {
                if let Some(border_style) = parse_border_style(&style["border_style"]) {
                    image.add_style(border_style);
                }
            }

            if !style["border_width"].is_null() {
                if let Some(border_width) = parse_border_width(&style["border_width"]) {
                    image.add_style(border_width);
                }
            }

            Some(BlockType::Image(image))
        }
        "line" => {
            let frame = object_json["frame"]
                .as_object()
                .map(|_| parse_frame(&object_json["frame"]));

            let style = &object_json["style"];

            let mut line = Line::new(frame.unwrap());

            if style.is_null() {
                return Some(BlockType::Line(line));
            }

            if !style["space"].is_null() {
                if let Some(space) = parse_space(&style["space"]) {
                    (line).add_style(space);
                }
            }

            if !style["border_color"].is_null() {
                if let Some(border_color) = parse_border_color(&style["border_color"]) {
                    line.add_style(border_color);
                }
            }

            if !style["border_style"].is_null() {
                if let Some(border_style) = parse_border_style(&style["border_style"]) {
                    line.add_style(border_style);
                }
            }

            if !style["border_width"].is_null() {
                if let Some(border_width) = parse_border_width(&style["border_width"]) {
                    line.add_style(border_width);
                }
            }

            Some(BlockType::Line(line))
        }
        "rectangle" => {
            let frame = object_json["frame"]
                .as_object()
                .map(|_| parse_frame(&object_json["frame"]));

            let mut rectangle = Rectangle::new(frame);

            let style = &object_json["style"];

            if style.is_null() {
                return Some(BlockType::Rectangle(rectangle));
            }

            if !style["background_color"].is_null() {
                if let Some(background_color) = parse_background_color(&style["background_color"]) {
                    rectangle.add_style(background_color);
                }
            }

            if !style["border_color"].is_null() {
                if let Some(border_color) = parse_border_color(&style["border_color"]) {
                    rectangle.add_style(border_color);
                }
            }

            if !style["border_style"].is_null() {
                if let Some(border_style) = parse_border_style(&style["border_style"]) {
                    rectangle.add_style(border_style);
                }
            }

            if !style["border_width"].is_null() {
                if let Some(border_width) = parse_border_width(&style["border_width"]) {
                    rectangle.add_style(border_width);
                }
            }

            Some(BlockType::Rectangle(rectangle))
        }
        "object" => {
            if let Some(object) = parse_object(&object_json["object"]) {
                let mut wrapper = Wrapper::new(object);

                let style = &object_json["style"];
                if !style["space"].is_null() {
                    if let Some(space) = parse_space(&style["space"]) {
                        wrapper.add_style(space);
                    }
                }

                return Some(BlockType::Wrapper(Box::from(wrapper)));
            }

            None
        },
        "objects" => {
            let frame = object_json["frame"]
                .as_object()
                .map(|_| parse_frame(&object_json["frame"]));

            let mut container = BlockContainer::new(frame);

            if !object_json["direction"].is_null() {
                container.set_direction(parse_direction(&object_json["direction"]));
            }

            object_json["objects"]
                .as_array()
                .unwrap()
                .iter()
                .for_each(|object_json| {
                    if let Some(object) = parse_object(object_json) {
                        container.add_block(object);
                    }
                });

            Some(BlockType::Container(container))
        }
        "flexible" => {
            let frame = object_json["frame"]
                .as_object()
                .map(|_| parse_frame(&object_json["frame"]));

            let mut container = FlexibleContainer::new(frame);

            if !object_json["direction"].is_null() {
                container.set_direction(parse_direction(&object_json["direction"]));
            }

            object_json["objects"]
                .as_array()
                .unwrap()
                .iter()
                .for_each(|object_json| {
                    if let Some(object) = parse_object(object_json) {
                        container.add_block(object);
                    }
                });

            Some(BlockType::Flexible(container))
        }
        "flexible_item" => {
            if let Some(object) = parse_object(&object_json["object"]) {
                let mut basis: Option<f32> = None;

                if !object_json["basis"].is_null() {
                    basis = Some(object_json["basis"].as_f64().unwrap() as f32);
                }

                return Some(
                    BlockType::FlexibleItem(Box::from(
                        FlexibleItem::new(
                            object,
                            basis,
                        )
                    ))
                );
            }

            None
        }
        _ => {
            eprintln!("unknown block type");

            None
        }
    }
}

fn parse_frame(frame_json: &Value) -> GeoRect {
    match frame_json.as_object() {
        Some(frame) => {
            let point_x = frame["point"]["x"].as_f64();
            let point_y = frame["point"]["y"].as_f64();
            let size_w = frame["size"]["width"].as_f64();
            let size_h = frame["size"]["height"].as_f64();

            match (point_x, point_y, size_w, size_h) {
                (Some(x), Some(y), Some(w), Some(h)) => {
                    GeoRect::new(w as f32, h as f32, x as f32, y as f32)
                }
                (Some(x), Some(y), None, None) => GeoRect {
                    point: Some(GeoPoint {
                        x: x as f32,
                        y: y as f32,
                    }),
                    size: None,
                },
                (None, None, Some(w), Some(h)) => GeoRect {
                    point: None,
                    size: Some(GeoSize {
                        width: w as f32,
                        height: h as f32,
                    }),
                },
                _ => GeoRect::none(),
            }
        }
        _ => GeoRect::none(),
    }
}

fn parse_direction(direction_json: &Value) -> Direction {
    match direction_json.as_str().unwrap() {
        "horizontal" => Direction::Horizontal,
        "vertical" => Direction::Vertical,
        _ => {
            panic!("unknown direction");
        }
    }
}

fn parse_background_color(background_color_json: &Value) -> Option<Style> {
    Some(Style::BackgroundColor(RgbColor {
        r: background_color_json["red"].as_u64().unwrap() as u8,
        g: background_color_json["green"].as_u64().unwrap() as u8,
        b: background_color_json["blue"].as_u64().unwrap() as u8,
    }))
}

fn parse_border_color(border_color_json: &Value) -> Option<Style> {
    Some(Style::BorderColor(RgbColor {
        r: border_color_json["red"].as_u64().unwrap() as u8,
        g: border_color_json["green"].as_u64().unwrap() as u8,
        b: border_color_json["blue"].as_u64().unwrap() as u8,
    }))
}

fn parse_border_style(border_json: &Value) -> Option<Style> {
    match border_json["line_style"].as_str().unwrap() {
        "solid" => Some(Style::BorderStyle(BorderStyle::Solid)),
        "dash" => {
            let dash_1 = border_json["dash_1"].as_i64().unwrap();
            Some(Style::BorderStyle(BorderStyle::Dash(dash_1)))
        }
        _ => None,
    }
}

fn parse_border_width(border_width_json: &Value) -> Option<Style> {
    Some(Style::BorderWidth(
        border_width_json["width"].as_f64().unwrap() as f32,
    ))
}

fn parse_alignment(alignment_json: &Value) -> Option<Style> {
    Some(Style::Alignment(Alignment {
        horizontal: match alignment_json["horizontal"].as_str().unwrap_or("") {
            "left" => Some(HorizontalAlignment::Left),
            "center" => Some(HorizontalAlignment::Center),
            "right" => Some(HorizontalAlignment::Right),
            _ => None,
        },
        vertical: match alignment_json["vertical"].as_str().unwrap_or("") {
            "top" => Some(VerticalAlignment::Top),
            "center" => Some(VerticalAlignment::Center),
            "bottom" => Some(VerticalAlignment::Bottom),
            _ => None,
        },
    }))
}

fn parse_space(space_json: &Value) -> Option<Style> {
    Some(
        Style::Space(
            Space {
                top: space_json["top"].as_f64().unwrap() as f32,
                right: space_json["right"].as_f64().unwrap() as f32,
                bottom: space_json["bottom"].as_f64().unwrap() as f32,
                left: space_json["left"].as_f64().unwrap() as f32,
            }
        )
    )
}

fn parse_text_fill_color(text_fill_color_json: &Value) -> Option<Style> {
    Some(Style::TextFillColor(RgbColor {
        r: text_fill_color_json["red"].as_u64().unwrap() as u8,
        g: text_fill_color_json["green"].as_u64().unwrap() as u8,
        b: text_fill_color_json["blue"].as_u64().unwrap() as u8,
    }))
}

fn parse_text_outline_color(text_outline_color: &Value) -> Option<Style> {
    Some(Style::TextOutlineColor(RgbColor {
        r: text_outline_color["red"].as_u64().unwrap() as u8,
        g: text_outline_color["green"].as_u64().unwrap() as u8,
        b: text_outline_color["blue"].as_u64().unwrap() as u8,
    }))
}

fn parse_text_outline_style(text_outline_style_json: &Value) -> Option<Style> {
    match text_outline_style_json["line_style"].as_str().unwrap() {
        "solid" => Some(Style::TextOutlineStyle(TextOutlineStyle::Solid)),
        "dash" => {
            let dash_1 = text_outline_style_json["dash_1"].as_i64().unwrap();
            Some(Style::TextOutlineStyle(TextOutlineStyle::Dash(dash_1)))
        }
        _ => None,
    }
}

fn parse_text_style(text_style_json: &Value) -> Option<Style> {
    let line_style = text_style_json["line_style"].as_str().unwrap();
    match line_style {
        "fill" => Some(Style::TextStyle(TextStyle::Fill)),
        "stroke" => Some(Style::TextStyle(TextStyle::Stroke)),
        "fill_stroke" => Some(Style::TextStyle(TextStyle::FillStroke)),
        _ => None,
    }
}

fn parse_text_wrap(text_wrap_json: &Value) -> Option<Style> {
    let mode = match text_wrap_json["mode"].as_str().unwrap() {
        "none" => TextWrapMode::None,
        "word" => TextWrapMode::Word,
        "character" => TextWrapMode::Character,
        _ => return None,
    };

    let break_anywhere = text_wrap_json["break_anywhere"].as_bool().unwrap_or(false);

    let overflow = match text_wrap_json["overflow"].as_str().unwrap_or("clip") {
        "clip" => TextOverflow::Clip,
        "ellipsis" => TextOverflow::Ellipsis,
        _ => TextOverflow::Clip,
    };

    Some(Style::TextWrap(TextWrap {
        mode,
        break_anywhere,
        overflow,
    }))
}