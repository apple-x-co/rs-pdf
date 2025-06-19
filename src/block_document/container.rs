use crate::block_document::block::BlockType;
use crate::block_document::direction::Direction;
use crate::block_document::document::px_to_mm;
use crate::block_document::geometry::{GeoRect, GeoPoint, GeoSize};
use crate::block_document::image::Image;
use crate::block_document::text::Text;
use crate::block_document::text_renderer::measure_text;
use image::{GenericImageView, ImageError};
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Container {
    pub blocks: Vec<BlockType>,
}

impl Container {
    pub fn new() -> Container {
        Container { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, block: BlockType) {
        self.blocks.push(block);
    }

    // NOTE: 座標を計算する
    pub fn apply_constraints(
        &mut self,
        parent_frame: &GeoRect,
        direction: &Direction,
        font_path: &String,
    ) {
        let mut drawn_frame = GeoRect::new(0.0, 0.0, parent_frame.min_x(), parent_frame.min_y());

        for block in self.blocks.iter_mut() {
            let (is_fixed, frame) = Self::apply_block_constraints(
                block,
                &parent_frame,
                &drawn_frame,
                direction,
                font_path,
            );
            if is_fixed {
                continue;
            }

            drawn_frame = drawn_frame.union(frame.as_ref().unwrap_or(&GeoRect::default()));
        }
    }

    fn apply_block_constraints(
        block: &mut BlockType,
        parent_frame: &GeoRect,
        drawn_frame: &GeoRect,
        direction: &Direction,
        font_path: &String,
    ) -> (bool, Option<GeoRect>) {
        match block {
            BlockType::Container(block_container) => {
                if block_container.frame.is_some()
                    && block_container.frame.as_ref().unwrap().point.is_some()
                    && block_container.frame.as_ref().unwrap().size.is_some()
                {
                    let mut inner_drawn_frame = GeoRect::zero();

                    for block in block_container.blocks.iter_mut() {
                        let (is_fixed, frame) = Self::apply_block_constraints(
                            block,
                            block_container.frame.as_ref().unwrap(), // NOTE: 合ってないかも...
                            &inner_drawn_frame,
                            &block_container.direction.clone(),
                            font_path,
                        );

                        if is_fixed {
                            continue;
                        }

                        if let Some(frame) = frame {
                            match block_container.direction {
                                Direction::Horizontal => {
                                    inner_drawn_frame = GeoRect::new(
                                        inner_drawn_frame.width() + frame.width(),
                                        inner_drawn_frame.height().max(frame.height()), // NOTE: 最大の高さを保持
                                        0.0,
                                        0.0,
                                    );
                                }
                                Direction::Vertical => {
                                    inner_drawn_frame = GeoRect::new(
                                        inner_drawn_frame.width().max(frame.width()), // NOTE: 最大の幅を保持
                                        inner_drawn_frame.height() + frame.height(),
                                        0.0,
                                        0.0,
                                    );
                                }
                            }
                        }
                    }

                    return (true, None);
                }

                let mut inner_drawn_frame = GeoRect::zero();

                for block in block_container.blocks.iter_mut() {
                    let (is_fixed, frame) = Self::apply_block_constraints(
                        block,
                        parent_frame, // NOTE: 合ってないかも...
                        &inner_drawn_frame,
                        &block_container.direction.clone(),
                        font_path,
                    );

                    if is_fixed {
                        continue;
                    }

                    if let Some(frame) = frame {
                        match block_container.direction {
                            Direction::Horizontal => {
                                inner_drawn_frame = GeoRect::new(
                                    inner_drawn_frame.width() + frame.width(),
                                    inner_drawn_frame.height().max(frame.height()), // NOTE: 最大の高さを保持
                                    0.0,
                                    0.0,
                                );
                            }
                            Direction::Vertical => {
                                inner_drawn_frame = GeoRect::new(
                                    inner_drawn_frame.width().max(frame.width()), // NOTE: 最大の幅を保持
                                    inner_drawn_frame.height() + frame.height(),
                                    0.0,
                                    0.0,
                                );
                            }
                        }
                    }
                }

                let container_drawn_frame = GeoRect::new(
                    inner_drawn_frame.width(),
                    inner_drawn_frame.height(),
                    match direction {
                        Direction::Vertical => drawn_frame.min_x(),
                        Direction::Horizontal => drawn_frame.max_x(),
                    },
                    match direction {
                        Direction::Vertical => drawn_frame.max_y(),
                        Direction::Horizontal => drawn_frame.min_y(),
                    },
                );
                block_container.set_frame(container_drawn_frame.clone());

                (false, Some(container_drawn_frame))
            }
            BlockType::Wrapper(block_wrapper) => {
                if block_wrapper.frame.is_some()
                    && block_wrapper.frame.as_ref().unwrap().point.is_some()
                    && block_wrapper.frame.as_ref().unwrap().size.is_some()
                {
                    let mut inner_drawn_frame = block_wrapper.frame.clone().unwrap();

                    for style in block_wrapper.styles.iter() {
                        match style {
                            Style::Space(space) => {
                                inner_drawn_frame = inner_drawn_frame.padding(space);
                            }
                            _ => {}
                        }
                    }

                    return (true, Some(inner_drawn_frame));
                }

                let (is_fixed, frame) = Self::apply_block_constraints(
                    &mut block_wrapper.block,
                    parent_frame, // NOTE: 合ってないかも...
                    &GeoRect::zero(),
                    &Direction::Horizontal,
                    font_path,
                );

                if is_fixed {
                    return (true, None);
                }

                if let Some(frame) = frame {
                    let mut inner_drawn_frame = GeoRect::new(
                        frame.width(),
                        frame.height(),
                        drawn_frame.max_x(),
                        drawn_frame.min_y(),
                    );

                    for style in block_wrapper.styles.iter() {
                        match style {
                            Style::Space(space) => {
                                inner_drawn_frame = inner_drawn_frame.padding(space);
                            }
                            _ => {}
                        }
                    }

                    block_wrapper.set_frame(inner_drawn_frame.clone());

                    return (false, Some(inner_drawn_frame))
                }

                (false, None)
            }
            BlockType::Flexible(flexible_container) => {
                let count = flexible_container.blocks.len();
                let mut width = parent_frame.width();
                let mut height = parent_frame.height() - drawn_frame.height();

                if flexible_container.frame.is_some()
                    && flexible_container.frame.as_ref().unwrap().point.is_some()
                    && flexible_container.frame.as_ref().unwrap().size.is_some()
                {
                    width = flexible_container.frame.as_ref().unwrap().width();
                    height = flexible_container.frame.as_ref().unwrap().height();
                }

                let item_width = width / count as f32;
                let item_height = height / count as f32;

                let mut inner_drawn_frame = GeoRect::zero();
                let mut i = 0;

                for block in flexible_container.blocks.iter_mut() {
                    let item_frame = match flexible_container.direction {
                        Direction::Horizontal => GeoRect {
                            point: Some(GeoPoint {
                                x: i as f32 * item_width,
                                y: 0.0,
                            }),
                            size: None,
                        },
                        Direction::Vertical => GeoRect {
                            point: Some(GeoPoint {
                                x: 0.0,
                                y: i as f32 * item_height,
                            }),
                            size: None,
                        },
                    };

                    // NOTE: FlexItem の場合は "アイテム幅 OR アイテム高さ" を設定
                    match block {
                        BlockType::FlexibleItem(flexible_item) => {
                            flexible_item.set_frame(match flexible_container.direction {
                                Direction::Horizontal => GeoRect {
                                    point: Some(GeoPoint {
                                        x: i as f32 * item_width,
                                        y: 0.0,
                                    }),
                                    size: Some(GeoSize {
                                        width: item_width,
                                        height: 0.0,
                                    }),
                                },
                                Direction::Vertical => GeoRect {
                                    point: Some(GeoPoint {
                                        x: 0.0,
                                        y: i as f32 * item_height,
                                    }),
                                    size: Some(GeoSize {
                                        width: 0.0,
                                        height: item_height,
                                    }),
                                },
                            });
                        }
                        _ => {}
                    }

                    let (is_fixed, frame) = Self::apply_block_constraints(
                        block,
                        &item_frame, // NOTE: 合ってないかも...
                        &inner_drawn_frame,
                        &flexible_container.direction.clone(),
                        font_path,
                    );

                    if is_fixed {
                        continue;
                    }

                    if let Some(frame) = frame {
                        match flexible_container.direction {
                            Direction::Horizontal => {
                                inner_drawn_frame = GeoRect::new(
                                    inner_drawn_frame.width() + item_width,
                                    inner_drawn_frame.height().max(frame.height()), // NOTE: 最大の高さを保持
                                    0.0,
                                    0.0,
                                );
                            }
                            Direction::Vertical => {
                                inner_drawn_frame = GeoRect::new(
                                    inner_drawn_frame.width().max(frame.width()), // NOTE: 最大の幅を保持
                                    inner_drawn_frame.height() + item_height,
                                    0.0,
                                    0.0,
                                );
                            }
                        }
                    }

                    i += 1;
                }

                let container_drawn_frame = GeoRect::new(
                    inner_drawn_frame.width(),
                    inner_drawn_frame.height(),
                    match direction {
                        Direction::Vertical => drawn_frame.min_x(),
                        Direction::Horizontal => drawn_frame.max_x(),
                    },
                    match direction {
                        Direction::Vertical => drawn_frame.max_y(),
                        Direction::Horizontal => drawn_frame.min_y(),
                    },
                );
                flexible_container.set_frame(container_drawn_frame.clone());

                (false, Some(container_drawn_frame))
            }
            BlockType::FlexibleItem(flexible_item) => {
                if let Some(frame) = flexible_item.frame.as_ref() {
                    let (_, frame) = Self::apply_block_constraints(
                        &mut flexible_item.block,
                        frame,
                        &GeoRect::zero(),
                        &Direction::Horizontal,
                        font_path,
                    );
                    
                    return (false, frame);
                }

                (false, None)
            }
            BlockType::Rectangle(block_rectangle) => {
                if block_rectangle.frame.is_some()
                    && block_rectangle.frame.as_ref().unwrap().point.is_some()
                    && block_rectangle.frame.as_ref().unwrap().size.is_some()
                {
                    return (true, None);
                }

                if block_rectangle.frame.is_some()
                    && block_rectangle.frame.as_ref().unwrap().point.is_none()
                    && block_rectangle.frame.as_ref().unwrap().size.is_some()
                {
                    let frame = GeoRect::new(
                        block_rectangle.frame.as_ref().unwrap().size.as_ref().unwrap().width,
                        block_rectangle.frame.as_ref().unwrap().size.as_ref().unwrap().height,
                        drawn_frame.max_x(),
                        drawn_frame.min_y(),
                    );

                    block_rectangle.set_frame(frame.clone());

                    return (false, Some(frame));
                }

                (false, Some(GeoRect::zero()))
            }
            BlockType::Text(block_text) => {
                let (
                    is_fixed,
                    frame_width,
                    frame_height,
                    frame_x,
                    frame_y,
                    text_width,
                    text_height,
                ) = Self::calculate_text_constraints(
                    block_text,
                    &drawn_frame,
                    direction,
                    font_path,
                );

                block_text.set_text_size(GeoSize::new(
                    text_width,
                    text_height
                ));

                if block_text.frame.is_some()
                    && block_text.frame.as_ref().unwrap().point.is_some()
                    && block_text.frame.as_ref().unwrap().size.is_some()
                {
                    return (true, None);
                }

                block_text.set_frame(GeoRect::new(frame_width, frame_height, frame_x, frame_y));

                if is_fixed {
                    return (true, None);
                }

                (
                    false,
                    Some(GeoRect::new(frame_width, frame_height, frame_x, frame_y)),
                )
            }
            BlockType::Image(block_image) => {
                if block_image.frame.is_some()
                    && block_image.frame.as_ref().unwrap().point.is_some()
                    && block_image.frame.as_ref().unwrap().size.is_some()
                {
                    return (true, None);
                }

                let (is_fixed, width, height, x, y) =
                    Self::calculate_image_constraints(block_image, &drawn_frame, direction);
                block_image.set_frame(GeoRect::new(width, height, x, y));

                if is_fixed {
                    return (true, None);
                }

                (false, Some(GeoRect::new(width, height, x, y)))
            }
            BlockType::Line(_) => (false, None),
        }
    }

    fn get_image_dimensions(path: &str) -> Result<(u32, u32), ImageError> {
        image::io::Reader::open(path)
            .map_err(|e| ImageError::from(e))?
            .decode()
            .map(|image| image.dimensions())
            .map_err(ImageError::from)
    }

    fn calculate_image_constraints(
        block_image: &Image,
        drawn_frame: &GeoRect,
        direction: &Direction,
    ) -> (bool, f32, f32, f32, f32) {
        // NOTE: 絶対配置
        let is_fixed =
            block_image.frame.is_some() && block_image.frame.as_ref().unwrap().point.is_some();

        let (mut width, mut height, mut x, mut y) = {
            let frame = block_image.frame.as_ref();
            (
                frame
                    .and_then(|b| b.size.as_ref().map(|s| s.width))
                    .unwrap_or(0.0),
                frame
                    .and_then(|b| b.size.as_ref().map(|s| s.height))
                    .unwrap_or(0.0),
                frame
                    .and_then(|b| b.point.as_ref().map(|p| p.x))
                    .unwrap_or(0.0),
                frame
                    .and_then(|b| b.point.as_ref().map(|p| p.y))
                    .unwrap_or(0.0),
            )
        };

        // NOTE: サイズが未指定の場合は画像を読み込んでサイズを取得
        if block_image
            .frame
            .as_ref()
            .map_or(true, |b| b.size.is_none())
        {
            match Self::get_image_dimensions(&block_image.path) {
                Ok((image_width, image_height)) => {
                    width = px_to_mm(image_width as f32);
                    height = px_to_mm(image_height as f32);
                }
                Err(e) => {
                    panic!(
                        "Error reading image dimensions for {}: {}",
                        block_image.path, e
                    );
                }
            }
        }

        // NOTE: 位置が未指定の場合は drawn_frame を基準に座標を決定
        if block_image
            .frame
            .as_ref()
            .map_or(true, |b| b.point.is_none())
        {
            x = match direction {
                Direction::Vertical => drawn_frame.min_x(),
                Direction::Horizontal => drawn_frame.max_x(),
            };
            y = match direction {
                Direction::Vertical => drawn_frame.max_y(),
                Direction::Horizontal => drawn_frame.min_y(),
            };
        }

        (is_fixed, width, height, x, y)
    }

    fn calculate_text_constraints(
        block_text: &Text,
        drawn_frame: &GeoRect,
        direction: &Direction,
        font_path: &String,
    ) -> (bool, f32, f32, f32, f32, f32, f32) {
        // NOTE: 絶対配置
        let is_fixed =
            block_text.frame.is_some() && block_text.frame.as_ref().unwrap().point.is_some();

        let (mut frame_width, mut frame_height, mut frame_x, mut frame_y) = {
            let frame = block_text.frame.as_ref();
            (
                frame
                    .and_then(|b| b.size.as_ref().map(|s| s.width))
                    .unwrap_or(0.0),
                frame
                    .and_then(|b| b.size.as_ref().map(|s| s.height))
                    .unwrap_or(0.0),
                frame
                    .and_then(|b| b.point.as_ref().map(|p| p.x))
                    .unwrap_or(0.0),
                frame
                    .and_then(|b| b.point.as_ref().map(|p| p.y))
                    .unwrap_or(0.0),
            )
        };

        // NOTE: グリフサイズを取得
        let text_size = measure_text(
            &block_text.text,
            block_text.font_size,
            block_text.font_path.as_ref().unwrap_or(&font_path),
        );

        // NOTE: サイズが未指定の場合はグリフサイズを設定
        if block_text
            .frame
            .as_ref()
            .map_or(true, |b| b.size.is_none())
        {
            frame_width = text_size.width;
            frame_height = text_size.height;
        }

        // NOTE: 位置が未指定の場合は drawn_frame を基準に座標を決定
        if block_text
            .frame
            .as_ref()
            .map_or(true, |b| b.point.is_none())
        {
            frame_x = match direction {
                Direction::Vertical => drawn_frame.min_x(),
                Direction::Horizontal => drawn_frame.max_x(),
            };
            frame_y = match direction {
                Direction::Vertical => drawn_frame.max_y(),
                Direction::Horizontal => drawn_frame.min_y(),
            };
        }

        (
            is_fixed,
            frame_width,
            frame_height,
            frame_x,
            frame_y,
            text_size.width,
            text_size.height,
        )
    }
}
