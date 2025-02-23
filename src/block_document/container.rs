use crate::block_document::block::BlockType;
use crate::block_document::direction::Direction;
use crate::block_document::document::px_to_mm;
use crate::block_document::geometry::Bounds;
use crate::block_document::image::Image;
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
    pub fn apply_constraints(&mut self, parent_bounds: &Bounds) {
        let mut drawn_bounds =
            Bounds::new(0.0, 0.0, parent_bounds.min_x(), parent_bounds.min_y());

        for block in self.blocks.iter_mut() {
            let (is_fixed, bounds) =
                Self::apply_block_constraints(block, &drawn_bounds, Direction::Vertical);
            if is_fixed {
                continue;
            }

            drawn_bounds = drawn_bounds.union(bounds.as_ref().unwrap_or(&Bounds::default()));
        }
    }

    // FIXME: 実装する
    // FIXME: styles に Space が場合は insets した矩形が描画開始位置サイズになる。矩形自体は変わらない。
    fn apply_block_constraints(
        block: &mut BlockType,
        drawn_bounds: &Bounds,
        direction: Direction,
    ) -> (bool, Option<Bounds>) {
        match block {
            BlockType::Container(block_container) => {
                if block_container.bounds.is_some()
                    && block_container.bounds.as_ref().unwrap().point.is_some()
                    && block_container.bounds.as_ref().unwrap().size.is_some()
                {
                    return (true, None);
                }

                let mut inner_drawn_bounds = Bounds::new(
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                );

                for block in block_container.blocks.iter_mut() {
                    let (is_fixed, bounds) = Self::apply_block_constraints(
                        block,
                        &inner_drawn_bounds,
                        block_container.direction.clone(),
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
                                    0.0
                                );
                            }
                            Direction::Vertical => {
                                inner_drawn_bounds = Bounds::new(
                                    inner_drawn_bounds.width().max(bounds.width()), // NOTE: 最大の幅を保持
                                    inner_drawn_bounds.height() + bounds.height(),
                                    0.0,
                                    0.0
                                );
                            }
                        }
                    }
                }

                let container_drawn_bounds = Bounds::new(
                    inner_drawn_bounds.width(),
                    inner_drawn_bounds.height(),
                    if direction.is_vertical() { drawn_bounds.min_x() } else { drawn_bounds.max_x() },
                    if direction.is_vertical() { drawn_bounds.max_y() } else { drawn_bounds.min_y() },
                );
                block_container.set_bounds(container_drawn_bounds.clone());

                (false, Some(container_drawn_bounds))
            }
            // BlockType::Line(_) => {}
            // BlockType::Rectangle(_) => {}
            // BlockType::Text(_) => {}
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
            _ => (false, None),
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
        direction: Direction,
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

        // サイズが未指定の場合は画像を読み込んでサイズを取得
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

        // 位置が未指定の場合は drawn_bounds を基準に座標を決定
        if block_image
            .bounds
            .as_ref()
            .map_or(true, |b| b.point.is_none())
        {
            x = if direction.is_vertical() { drawn_bounds.min_x() } else { drawn_bounds.max_x() };
            y = if direction.is_vertical() { drawn_bounds.max_y() } else { drawn_bounds.min_y() };
        }

        (is_fixed, width, height, x, y)
    }
}
