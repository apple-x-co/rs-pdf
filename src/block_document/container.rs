use crate::block_document::block::BlockType;
use crate::block_document::direction::Direction;
use crate::block_document::document::px_to_mm;
use crate::block_document::geometry::{Bounds, Point, Size};
use crate::block_document::image::Image;
use crate::block_document::text::Text;
use crate::block_document::text_renderer::measure_text;
use image::{GenericImageView, ImageError};

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
        parent_bounds: &Bounds,
        direction: &Direction,
        font_path: &String,
    ) {
        let mut drawn_bounds = Bounds::new(0.0, 0.0, parent_bounds.min_x(), parent_bounds.min_y());

        for block in self.blocks.iter_mut() {
            let (is_fixed, bounds) = Self::apply_block_constraints(
                block,
                &parent_bounds,
                &drawn_bounds,
                direction,
                font_path,
            );
            if is_fixed {
                continue;
            }

            drawn_bounds = drawn_bounds.union(bounds.as_ref().unwrap_or(&Bounds::default()));
        }
    }

    // FIXME: styles に Space が場合は insets した矩形が描画開始位置サイズになる。矩形自体は変わらない。
    fn apply_block_constraints(
        block: &mut BlockType,
        parent_bounds: &Bounds,
        drawn_bounds: &Bounds,
        direction: &Direction,
        font_path: &String,
    ) -> (bool, Option<Bounds>) {
        match block {
            BlockType::Container(block_container) => {
                if block_container.bounds.is_some()
                    && block_container.bounds.as_ref().unwrap().point.is_some()
                    && block_container.bounds.as_ref().unwrap().size.is_some()
                {
                    let mut inner_drawn_bounds = Bounds::zero();

                    for block in block_container.blocks.iter_mut() {
                        let (is_fixed, bounds) = Self::apply_block_constraints(
                            block,
                            block_container.bounds.as_ref().unwrap(), // NOTE: 合ってないかも...
                            &inner_drawn_bounds,
                            &block_container.direction.clone(),
                            font_path,
                        );

                        if is_fixed {
                            continue;
                        }

                        if let Some(bounds) = bounds {
                            match block_container.direction {
                                Direction::Horizontal => {
                                    inner_drawn_bounds = Bounds::new(
                                        inner_drawn_bounds.width() + bounds.width(),
                                        inner_drawn_bounds.height().max(bounds.height()), // NOTE: 最大の高さを保持
                                        0.0,
                                        0.0,
                                    );
                                }
                                Direction::Vertical => {
                                    inner_drawn_bounds = Bounds::new(
                                        inner_drawn_bounds.width().max(bounds.width()), // NOTE: 最大の幅を保持
                                        inner_drawn_bounds.height() + bounds.height(),
                                        0.0,
                                        0.0,
                                    );
                                }
                            }
                        }
                    }

                    return (true, None);
                }

                let mut inner_drawn_bounds = Bounds::zero();

                for block in block_container.blocks.iter_mut() {
                    let (is_fixed, bounds) = Self::apply_block_constraints(
                        block,
                        parent_bounds, // NOTE: 合ってないかも...
                        &inner_drawn_bounds,
                        &block_container.direction.clone(),
                        font_path,
                    );

                    if is_fixed {
                        continue;
                    }

                    if let Some(bounds) = bounds {
                        match block_container.direction {
                            Direction::Horizontal => {
                                inner_drawn_bounds = Bounds::new(
                                    inner_drawn_bounds.width() + bounds.width(),
                                    inner_drawn_bounds.height().max(bounds.height()), // NOTE: 最大の高さを保持
                                    0.0,
                                    0.0,
                                );
                            }
                            Direction::Vertical => {
                                inner_drawn_bounds = Bounds::new(
                                    inner_drawn_bounds.width().max(bounds.width()), // NOTE: 最大の幅を保持
                                    inner_drawn_bounds.height() + bounds.height(),
                                    0.0,
                                    0.0,
                                );
                            }
                        }
                    }
                }

                let container_drawn_bounds = Bounds::new(
                    inner_drawn_bounds.width(),
                    inner_drawn_bounds.height(),
                    match direction {
                        Direction::Vertical => drawn_bounds.min_x(),
                        Direction::Horizontal => drawn_bounds.max_x(),
                    },
                    match direction {
                        Direction::Vertical => drawn_bounds.max_y(),
                        Direction::Horizontal => drawn_bounds.min_y(),
                    },
                );
                block_container.set_bounds(container_drawn_bounds.clone());

                (false, Some(container_drawn_bounds))
            }
            BlockType::Flexible(flexible_container) => {
                let count = flexible_container.blocks.len();
                let mut width = parent_bounds.width();
                let mut height = parent_bounds.height() - drawn_bounds.height();

                if flexible_container.bounds.is_some()
                    && flexible_container.bounds.as_ref().unwrap().point.is_some()
                    && flexible_container.bounds.as_ref().unwrap().size.is_some()
                {
                    width = flexible_container.bounds.as_ref().unwrap().width();
                    height = flexible_container.bounds.as_ref().unwrap().height();
                }

                let item_width = width / count as f32;
                let item_height = height / count as f32;

                let mut inner_drawn_bounds = Bounds::zero();
                let mut i = 0;

                for block in flexible_container.blocks.iter_mut() {
                    let item_bounds = match flexible_container.direction {
                        Direction::Horizontal => Bounds {
                            point: Some(Point {
                                x: i as f32 * item_width,
                                y: 0.0,
                            }),
                            size: None,
                        },
                        Direction::Vertical => Bounds {
                            point: Some(Point {
                                x: 0.0,
                                y: i as f32 * item_height,
                            }),
                            size: None,
                        },
                    };

                    // NOTE: FlexItem の場合は "アイテム幅 OR アイテム高さ" を設定
                    match block {
                        BlockType::FlexibleItem(flexible_item) => {
                            flexible_item.set_bounds(match flexible_container.direction {
                                Direction::Horizontal => Bounds {
                                    point: Some(Point {
                                        x: i as f32 * item_width,
                                        y: 0.0,
                                    }),
                                    size: Some(Size {
                                        width: item_width,
                                        height: 0.0,
                                    }),
                                },
                                Direction::Vertical => Bounds {
                                    point: Some(Point {
                                        x: 0.0,
                                        y: i as f32 * item_height,
                                    }),
                                    size: Some(Size {
                                        width: 0.0,
                                        height: item_height,
                                    }),
                                },
                            });
                        }
                        _ => {}
                    }

                    let (is_fixed, bounds) = Self::apply_block_constraints(
                        block,
                        &item_bounds, // NOTE: 合ってないかも...
                        &inner_drawn_bounds,
                        &flexible_container.direction.clone(),
                        font_path,
                    );

                    if is_fixed {
                        continue;
                    }

                    if let Some(bounds) = bounds {
                        match flexible_container.direction {
                            Direction::Horizontal => {
                                inner_drawn_bounds = Bounds::new(
                                    inner_drawn_bounds.width() + item_width,
                                    inner_drawn_bounds.height().max(bounds.height()), // NOTE: 最大の高さを保持
                                    0.0,
                                    0.0,
                                );
                            }
                            Direction::Vertical => {
                                inner_drawn_bounds = Bounds::new(
                                    inner_drawn_bounds.width().max(bounds.width()), // NOTE: 最大の幅を保持
                                    inner_drawn_bounds.height() + item_height,
                                    0.0,
                                    0.0,
                                );
                            }
                        }
                    }

                    i += 1;
                }

                let container_drawn_bounds = Bounds::new(
                    inner_drawn_bounds.width(),
                    inner_drawn_bounds.height(),
                    match direction {
                        Direction::Vertical => drawn_bounds.min_x(),
                        Direction::Horizontal => drawn_bounds.max_x(),
                    },
                    match direction {
                        Direction::Vertical => drawn_bounds.max_y(),
                        Direction::Horizontal => drawn_bounds.min_y(),
                    },
                );
                flexible_container.set_bounds(container_drawn_bounds.clone());

                (false, Some(container_drawn_bounds))
            }
            BlockType::FlexibleItem(flexible_item) => {
                if let Some(bounds) = flexible_item.bounds.as_ref() {
                    let (_, bounds) = Self::apply_block_constraints(
                        &mut flexible_item.block,
                        bounds,
                        &Bounds::zero(),
                        &Direction::Horizontal,
                        font_path,
                    );
                    
                    return (false, bounds);
                }

                (false, None)
            }
            BlockType::Rectangle(block_rectangle) => {
                if block_rectangle.bounds.is_some()
                    && block_rectangle.bounds.as_ref().unwrap().point.is_some()
                    && block_rectangle.bounds.as_ref().unwrap().size.is_some()
                {
                    return (true, None);
                }

                (false, Some(Bounds::zero())) // FIXME: これあってる...?
            }
            BlockType::Text(block_text) => {
                let (
                    is_fixed,
                    bounds_width,
                    bounds_height,
                    bounds_x,
                    bounds_y,
                    text_width,
                    text_height,
                ) = Self::calculate_text_constraints(
                    block_text,
                    &drawn_bounds,
                    direction,
                    font_path,
                );

                block_text.set_text_size(Size::new(
                    text_width,
                    text_height
                ));

                if block_text.bounds.is_some()
                    && block_text.bounds.as_ref().unwrap().point.is_some()
                    && block_text.bounds.as_ref().unwrap().size.is_some()
                {
                    return (true, None);
                }

                block_text.set_bounds(Bounds::new(bounds_width, bounds_height, bounds_x, bounds_y));

                if is_fixed {
                    return (true, None);
                }

                (
                    false,
                    Some(Bounds::new(bounds_width, bounds_height, bounds_x, bounds_y)),
                )
            }
            BlockType::Image(block_image) => {
                if block_image.bounds.is_some()
                    && block_image.bounds.as_ref().unwrap().point.is_some()
                    && block_image.bounds.as_ref().unwrap().size.is_some()
                {
                    return (true, None);
                }

                let (is_fixed, width, height, x, y) =
                    Self::calculate_image_constraints(block_image, &drawn_bounds, direction);
                block_image.set_bounds(Bounds::new(width, height, x, y));

                if is_fixed {
                    return (true, None);
                }

                (false, Some(Bounds::new(width, height, x, y)))
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
        drawn_bounds: &Bounds,
        direction: &Direction,
    ) -> (bool, f32, f32, f32, f32) {
        // NOTE: 絶対配置
        let is_fixed =
            block_image.bounds.is_some() && block_image.bounds.as_ref().unwrap().point.is_some();

        let (mut width, mut height, mut x, mut y) = {
            let bounds = block_image.bounds.as_ref();
            (
                bounds
                    .and_then(|b| b.size.as_ref().map(|s| s.width))
                    .unwrap_or(0.0),
                bounds
                    .and_then(|b| b.size.as_ref().map(|s| s.height))
                    .unwrap_or(0.0),
                bounds
                    .and_then(|b| b.point.as_ref().map(|p| p.x))
                    .unwrap_or(0.0),
                bounds
                    .and_then(|b| b.point.as_ref().map(|p| p.y))
                    .unwrap_or(0.0),
            )
        };

        // NOTE: サイズが未指定の場合は画像を読み込んでサイズを取得
        if block_image
            .bounds
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

        // NOTE: 位置が未指定の場合は drawn_bounds を基準に座標を決定
        if block_image
            .bounds
            .as_ref()
            .map_or(true, |b| b.point.is_none())
        {
            x = match direction {
                Direction::Vertical => drawn_bounds.min_x(),
                Direction::Horizontal => drawn_bounds.max_x(),
            };
            y = match direction {
                Direction::Vertical => drawn_bounds.max_y(),
                Direction::Horizontal => drawn_bounds.min_y(),
            };
        }

        (is_fixed, width, height, x, y)
    }

    fn calculate_text_constraints(
        block_text: &Text,
        drawn_bounds: &Bounds,
        direction: &Direction,
        font_path: &String,
    ) -> (bool, f32, f32, f32, f32, f32, f32) {
        // NOTE: 絶対配置
        let is_fixed =
            block_text.bounds.is_some() && block_text.bounds.as_ref().unwrap().point.is_some();

        let (mut bounds_width, mut bounds_height, mut bounds_x, mut bounds_y) = {
            let bounds = block_text.bounds.as_ref();
            (
                bounds
                    .and_then(|b| b.size.as_ref().map(|s| s.width))
                    .unwrap_or(0.0),
                bounds
                    .and_then(|b| b.size.as_ref().map(|s| s.height))
                    .unwrap_or(0.0),
                bounds
                    .and_then(|b| b.point.as_ref().map(|p| p.x))
                    .unwrap_or(0.0),
                bounds
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
            .bounds
            .as_ref()
            .map_or(true, |b| b.size.is_none())
        {
            bounds_width = text_size.width;
            bounds_height = text_size.height;
        }

        // NOTE: 位置が未指定の場合は drawn_bounds を基準に座標を決定
        if block_text
            .bounds
            .as_ref()
            .map_or(true, |b| b.point.is_none())
        {
            bounds_x = match direction {
                Direction::Vertical => drawn_bounds.min_x(),
                Direction::Horizontal => drawn_bounds.max_x(),
            };
            bounds_y = match direction {
                Direction::Vertical => drawn_bounds.max_y(),
                Direction::Horizontal => drawn_bounds.min_y(),
            };
        }

        (
            is_fixed,
            bounds_width,
            bounds_height,
            bounds_x,
            bounds_y,
            text_size.width,
            text_size.height,
        )
    }
}
