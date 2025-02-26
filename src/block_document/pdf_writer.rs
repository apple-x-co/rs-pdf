use crate::block_document::block::BlockType;
use crate::block_document::direction::Direction;
use crate::block_document::document::{Document as BlockDocument, DPI as BlockDPI};
use crate::block_document::geometry::{Bounds as GeoBounds, Bounds};
use crate::block_document::image::Image as BlockImage;
use crate::block_document::line::Line as BlockLine;
use crate::block_document::rectangle::Rectangle as BlockRectangle;
use crate::block_document::style::{BorderStyle, Style, TextOutlineStyle, TextStyle};
use crate::block_document::text::Text as BlockText;
use printpdf::{
    Color, Image, ImageTransform, Line, LineDashPattern, Mm, PdfDocument, PdfDocumentReference,
    PdfPageIndex, Point, Rect, Rgb, TextRenderingMode,
};
use std::fs;
use std::fs::File;
use std::io::BufWriter;

pub fn save(block_document: BlockDocument, file: File, is_debug: bool) {
    let mut working_block_document = block_document.clone();

    let (doc, mut page_index, _) = PdfDocument::new(
        working_block_document.title.clone(),
        Mm(working_block_document.page_size.width),
        Mm(working_block_document.page_size.height),
        "Layer 1",
    );

    // NOTE: 基準点は左下
    let page_bounds = GeoBounds::new(
        working_block_document.page_size.width,
        working_block_document.page_size.height,
        0.0,
        0.0,
    );

    // NOTE: レイアウト（Bounds が確定する）
    for container in working_block_document.containers.iter_mut() {
        container.apply_constraints(&page_bounds, &Direction::Vertical);
    }

    // NOTE: 描画（Bounds が確定している）
    let mut i = 0;
    for container in working_block_document.containers.iter() {
        if i > 0 {
            (page_index, _) = doc.add_page(
                Mm(working_block_document.page_size.width),
                Mm(working_block_document.page_size.height),
                "Layer 1",
            );
        }

        i += 1;

        if is_debug {
            draw_grid(&doc, &page_index, &page_bounds)
        }

        for block in container.blocks.iter() {
            draw(&doc, &page_index, &page_bounds, block);
        }
    }

    doc.save(&mut BufWriter::new(file)).unwrap();
}

// NOTE: parent_bounds の基準点は左下
fn draw(
    doc: &PdfDocumentReference,
    page_index: &PdfPageIndex,
    parent_bounds: &Bounds,
    block: &BlockType,
) {
    match block {
        BlockType::Container(block_container) => {
            let lb_bounds = block_container
                .bounds
                .as_ref()
                .unwrap_or(&Bounds::none())
                .transform(parent_bounds);
            for block in block_container.blocks.iter() {
                draw(doc, page_index, &lb_bounds, block);
            }
        }
        BlockType::Line(line) => draw_line(doc, page_index, line, parent_bounds),
        BlockType::Rectangle(rectangle) => {
            draw_rectangle(doc, page_index, rectangle, parent_bounds)
        }
        BlockType::Text(text) => {
            draw_text(doc, page_index, text, parent_bounds);
        }
        BlockType::Image(image) => {
            draw_image(doc, page_index, image, parent_bounds);
        }
    }
}

fn draw_rectangle(
    doc: &PdfDocumentReference,
    page_index: &PdfPageIndex,
    block_rectangle: &BlockRectangle,
    geo_bounds: &GeoBounds,
) {
    if let Some(bounds) = &block_rectangle.bounds {
        if bounds.point.is_some() {
            let lb_bounds = bounds.transform(geo_bounds);

            let layer = doc.get_page(*page_index).add_layer("Layer");

            let mut border_required = false;

            for style in &block_rectangle.styles {
                match style {
                    Style::BackgroundColor(rgb_color) => {
                        layer.set_fill_color(Color::Rgb(Rgb {
                            r: rgb_color.r as f32 / 255.0,
                            g: rgb_color.g as f32 / 255.0,
                            b: rgb_color.b as f32 / 255.0,
                            icc_profile: None,
                        }));
                    }
                    Style::BorderColor(rgb_color) => {
                        border_required = true;
                        layer.set_outline_color(Color::Rgb(Rgb {
                            r: rgb_color.r as f32 / 255.0,
                            g: rgb_color.g as f32 / 255.0,
                            b: rgb_color.b as f32 / 255.0,
                            icc_profile: None,
                        }));
                    }
                    Style::BorderWidth(width) => {
                        border_required = true;
                        layer.set_outline_thickness(*width);
                    }
                    Style::BorderStyle(border_style) => match border_style {
                        BorderStyle::Dash(i) => {
                            border_required = true;
                            layer.set_line_dash_pattern(LineDashPattern {
                                dash_1: Some(*i),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            layer.add_rect(Rect::new(
                Mm(lb_bounds.min_x()), // 左上X
                Mm(lb_bounds.max_y()), // 左上Y
                Mm(lb_bounds.max_x()), // 右下X
                Mm(lb_bounds.min_y()), // 右下Y
            ));

            if border_required {
                layer.add_line(Line {
                    points: vec![
                        (
                            Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.min_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.min_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.max_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.max_y())),
                            false,
                        ),
                    ],
                    is_closed: true,
                });
            }
        }
    }
}

fn draw_line(
    doc: &PdfDocumentReference,
    page_index: &PdfPageIndex,
    block_line: &BlockLine,
    geo_bounds: &GeoBounds,
) {
    let lb_bounds = block_line.bounds.transform(geo_bounds);

    let layer = doc.get_page(*page_index).add_layer("Layer");

    for style in &block_line.styles {
        match style {
            Style::BorderColor(rgb_color) => {
                layer.set_outline_color(Color::Rgb(Rgb {
                    r: rgb_color.r as f32 / 255.0,
                    g: rgb_color.g as f32 / 255.0,
                    b: rgb_color.b as f32 / 255.0,
                    icc_profile: None,
                }));
            }
            Style::BorderWidth(width) => {
                layer.set_outline_thickness(*width);
            }
            Style::BorderStyle(border_style) => match border_style {
                BorderStyle::Dash(i) => {
                    layer.set_line_dash_pattern(LineDashPattern {
                        dash_1: Some(*i),
                        ..Default::default()
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }

    if lb_bounds.min_x() == lb_bounds.max_x() {
        layer.add_line(Line {
            points: vec![
                (
                    Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.min_y())),
                    false,
                ),
                (
                    Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.max_y())),
                    false,
                ),
            ],
            is_closed: false,
        });
    } else {
        layer.add_line(Line {
            points: vec![
                (
                    Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.max_y())),
                    false,
                ),
                (
                    Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.max_y())),
                    false,
                ),
            ],
            is_closed: false,
        });
    }
}

fn draw_text(
    doc: &PdfDocumentReference,
    page_index: &PdfPageIndex,
    block_text: &BlockText,
    geo_bounds: &GeoBounds,
) {
    if let Some(bounds) = &block_text.bounds {
        if bounds.point.is_some() {
            let lb_bounds = bounds.transform(geo_bounds);

            let layer1 = doc.get_page(*page_index).add_layer("Layer 1");

            let mut border_required = false;

            for style in &block_text.styles {
                match style {
                    Style::BorderColor(rgb_color) => {
                        border_required = true;
                        layer1.set_outline_color(Color::Rgb(Rgb {
                            r: rgb_color.r as f32 / 255.0,
                            g: rgb_color.g as f32 / 255.0,
                            b: rgb_color.b as f32 / 255.0,
                            icc_profile: None,
                        }));
                    }
                    Style::BorderWidth(width) => {
                        border_required = true;
                        layer1.set_outline_thickness(*width);
                    }
                    Style::BorderStyle(border_style) => match border_style {
                        BorderStyle::Dash(i) => {
                            border_required = true;
                            layer1.set_line_dash_pattern(LineDashPattern {
                                dash_1: Some(*i),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            if border_required {
                layer1.add_line(Line {
                    points: vec![
                        (
                            Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.min_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.min_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.max_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.max_y())),
                            false,
                        ),
                    ],
                    is_closed: true,
                });
            }

            let layer2 = doc.get_page(*page_index).add_layer("Layer 2");

            // let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
            let font = doc
                .add_external_font(File::open(&block_text.font_path).unwrap())
                .unwrap();
            for style in &block_text.styles {
                match style {
                    Style::TextFillColor(rgb_color) => {
                        layer2.set_fill_color(Color::Rgb(Rgb {
                            r: rgb_color.r as f32 / 255.0,
                            g: rgb_color.g as f32 / 255.0,
                            b: rgb_color.b as f32 / 255.0,
                            icc_profile: None,
                        }));
                    }
                    Style::TextOutlineColor(rgb_color) => {
                        layer2.set_outline_color(Color::Rgb(Rgb {
                            r: rgb_color.r as f32 / 255.0,
                            g: rgb_color.g as f32 / 255.0,
                            b: rgb_color.b as f32 / 255.0,
                            icc_profile: None,
                        }));
                    }
                    Style::TextStyle(text_style) => match text_style {
                        TextStyle::Fill => {
                            layer2.set_text_rendering_mode(TextRenderingMode::Fill);
                        }
                        TextStyle::Stroke => {
                            layer2.set_text_rendering_mode(TextRenderingMode::Stroke);
                        }
                        TextStyle::FillStroke => {
                            layer2.set_text_rendering_mode(TextRenderingMode::FillStroke);
                        }
                    },
                    Style::TextOutlineStyle(text_outline_style) => match text_outline_style {
                        TextOutlineStyle::Dash(i) => {
                            layer2.set_line_dash_pattern(LineDashPattern {
                                dash_1: Some(*i),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            // NOTE: 改行を考慮無し
            if !block_text.text.contains("\n") {
                layer2.use_text(
                    block_text.text.clone(),
                    block_text.font_size,
                    Mm(lb_bounds.min_x()),
                    Mm(lb_bounds.min_y()),
                    &font,
                );

                return;
            }

            // NOTE: 改行を考慮して描画
            let texts: Vec<&str> = block_text.text.split("\n").collect();
            let line_height = lb_bounds.height() / texts.iter().count() as f32;
            let mut current_y = lb_bounds.max_y() - line_height;
            for line in texts {
                layer2.use_text(
                    line,
                    block_text.font_size,
                    Mm(lb_bounds.min_x()),
                    Mm(current_y),
                    &font,
                );
                current_y -= line_height;
            }
        }
    }
}

fn draw_image(
    doc: &PdfDocumentReference,
    page_index: &PdfPageIndex,
    block_image: &BlockImage,
    geo_bounds: &GeoBounds,
) {
    if !fs::exists(&block_image.path).map_err(|e|{
        eprintln!("Failed to check if file {} exists due to {}", block_image.path, e);
    }).unwrap_or(false) {
        eprintln!("No such file or directory -> {:?}", &block_image.path);
        return;
    }

    if let Some(bounds) = &block_image.bounds {
        if bounds.point.is_some() {
            let lb_bounds = bounds.transform(geo_bounds);

            let image = image::io::Reader::open(&block_image.path)
                .unwrap()
                .decode()
                .unwrap();

            let pdf_image = Image::from_dynamic_image(&image);

            let transform = ImageTransform {
                translate_x: Some(Mm(lb_bounds.min_x())), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
                translate_y: Some(Mm(lb_bounds.min_y())), // NOTE: 画像の左下基準 なので、(0, 0) に配置すると PDF の左下に画像が配置される。
                scale_x: None,                            // NOTE: 水平方向の拡縮小
                scale_y: None,                            // NOTE: 垂直方向の拡縮小
                rotate: None,                             // 回転なし
                dpi: Some(BlockDPI),
            };

            let layer1 = doc.get_page(*page_index).add_layer("Layer");
            pdf_image.add_to_layer(layer1, transform);

            let layer2 = doc.get_page(*page_index).add_layer("Layer");
            let mut border_required = false;

            for style in &block_image.styles {
                match style {
                    Style::BorderColor(rgb_color) => {
                        border_required = true;
                        layer2.set_outline_color(Color::Rgb(Rgb {
                            r: rgb_color.r as f32 / 255.0,
                            g: rgb_color.g as f32 / 255.0,
                            b: rgb_color.b as f32 / 255.0,
                            icc_profile: None,
                        }));
                    }
                    Style::BorderWidth(width) => {
                        border_required = true;
                        layer2.set_outline_thickness(*width);
                    }
                    Style::BorderStyle(border_style) => match border_style {
                        BorderStyle::Dash(i) => {
                            border_required = true;
                            layer2.set_line_dash_pattern(LineDashPattern {
                                dash_1: Some(*i),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            if border_required {
                layer2.add_line(Line {
                    points: vec![
                        (
                            Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.min_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.min_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.max_x()), Mm(lb_bounds.max_y())),
                            false,
                        ),
                        (
                            Point::new(Mm(lb_bounds.min_x()), Mm(lb_bounds.max_y())),
                            false,
                        ),
                    ],
                    is_closed: true,
                });
            }
        }
    }
}

fn draw_grid(
    doc: &PdfDocumentReference,
    page_index: &PdfPageIndex,
    parent_bounds: &Bounds,
) {
    let layer1 = doc.get_page(*page_index).add_layer("Layer");
    layer1.set_outline_thickness(0.1);
    layer1.set_outline_color(Color::Rgb(Rgb {
        r: 200.0 / 255.0,
        g: 200.0 / 255.0,
        b: 200.0 / 255.0,
        icc_profile: None,
    }));

    let layer2 = doc.get_page(*page_index).add_layer("Layer");
    layer2.set_outline_thickness(0.1);
    layer2.set_outline_color(Color::Rgb(Rgb {
        r: 220.0 / 255.0,
        g: 220.0 / 255.0,
        b: 220.0 / 255.0,
        icc_profile: None,
    }));
    layer2.set_line_dash_pattern(LineDashPattern {
        dash_1: Some(1),
        ..Default::default()
    });

    let mut i = 0.0;
    while i < parent_bounds.max_y() {
        i += 1.0;

        if i % 5.0 == 0.0 {
            layer1.add_line(Line {
                points: vec![
                    (Point::new(Mm(parent_bounds.min_x()), Mm(parent_bounds.max_y() - i)), false),
                    (Point::new(Mm(parent_bounds.max_x()), Mm(parent_bounds.max_y() - i)), false),
                ],
                is_closed: false,
            });

            continue;
        }

        layer2.add_line(Line {
            points: vec![
                (Point::new(Mm(parent_bounds.min_x()), Mm(parent_bounds.max_y() - i)), false),
                (Point::new(Mm(parent_bounds.max_x()), Mm(parent_bounds.max_y() - i)), false),
            ],
            is_closed: false,
        });
    }

    i = 0.0;
    while i < parent_bounds.max_x() {
        i += 1.0;

        if i % 5.0 == 0.0 {
            layer1.add_line(Line {
                points: vec![
                    (Point::new(Mm(parent_bounds.max_x() - i), Mm(parent_bounds.min_y())), false),
                    (Point::new(Mm(parent_bounds.max_x() - i), Mm(parent_bounds.max_y())), false),
                ],
                is_closed: false,
            });

            continue;
        }

        layer2.add_line(Line {
            points: vec![
                (Point::new(Mm(parent_bounds.max_x() - i), Mm(parent_bounds.min_y())), false),
                (Point::new(Mm(parent_bounds.max_x() - i), Mm(parent_bounds.max_y())), false),
            ],
            is_closed: false,
        });
    }
}