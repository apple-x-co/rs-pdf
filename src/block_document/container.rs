use crate::block_document::block::BlockType;
use crate::block_document::document::px_to_mm;
use crate::block_document::geometry::{Bounds, Point};
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
    pub fn apply_constraints(&mut self, bounds: &Bounds) {
        // FIXME: 実装する
        // FIXME: styles に Space が場合は insets した矩形が描画開始位置サイズになる。矩形自体は変わらない。

        let mut block_point = Point {
            x: bounds.min_x(),
            y: bounds.min_y(),
        };

        // NOTE: Y軸方向（上から下）に描画していく。固定の座標がある場合は無視する。横レイアウトは block_container を使う
        for block in self.blocks.iter_mut() {
            match block {
                BlockType::Container(_) => {}
                BlockType::Line(_) => {}
                BlockType::Rectangle(_) => {}
                BlockType::Text(_) => {}
                BlockType::Image(block_image) => {
                    if block_image.bounds.is_some()
                        && block_image.bounds.as_ref().unwrap().point.is_some()
                        && block_image.bounds.as_ref().unwrap().size.is_some()
                    {
                        continue;
                    }

                    let (mut width, mut height, mut x, mut y) = {
                        let bounds = block_image.bounds.as_ref();
                        (
                            bounds.and_then(|b| b.size.as_ref().map(|s| s.width)).unwrap_or(0.0),
                            bounds.and_then(|b| b.size.as_ref().map(|s| s.height)).unwrap_or(0.0),
                            bounds.and_then(|b| b.point.as_ref().map(|p| p.x)).unwrap_or(0.0),
                            bounds.and_then(|b| b.point.as_ref().map(|p| p.y)).unwrap_or(0.0),
                        )
                    };

                    // サイズが未指定の場合は画像を読み込んでサイズを取得
                    if block_image.bounds.as_ref().map_or(true, |b| b.size.is_none()) {
                        match Self::get_image_dimensions(&block_image.path) {
                            Ok((image_width, image_height)) => {
                                width = px_to_mm(image_width as f32);
                                height = px_to_mm(image_height as f32);
                            }
                            Err(e) => {
                                panic!("Error reading image dimensions for {}: {}", block_image.path, e);
                            }
                        }
                    }

                    // 位置が未指定の場合は block_point を使用
                    if block_image.bounds.as_ref().map_or(true, |b| b.point.is_none()) {
                        x = block_point.x;
                        y = block_point.y;

                        block_point.y += height;
                    }

                    block_image.set_bounds(Bounds::new(width, height, x, y));
                }
            }
        }
    }

    fn get_image_dimensions(path: &str) -> Result<(u32, u32), ImageError> {
        image::io::Reader::open(path)
            .map_err(|e| ImageError::from(e))?
            .decode()
            .map(|image| image.dimensions())
            .map_err(ImageError::from)
    }
}
