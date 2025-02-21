use crate::block_document::block::BlockType;
use crate::block_document::document::px_to_mm;
use crate::block_document::geometry::{Bounds, Point};
use image::GenericImageView;

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
                    if !block_image.is_size_none() && !block_image.is_point_none() {
                        break;
                    }

                    let bounds = block_image
                        .bounds
                        .clone()
                        .unwrap_or_default();
                    let size = bounds.size
                        .clone()
                        .unwrap_or_default();
                    let point = bounds.point
                        .clone()
                        .unwrap_or_default();

                    let mut width = size.width;
                    let mut height = size.height;
                    let mut x = point.x;
                    let mut y = point.y;

                    if block_image.is_size_none() {
                        let image = image::io::Reader::open(block_image.path.clone())
                            .unwrap()
                            .decode()
                            .unwrap();
                        let (image_width, image_height) = image.dimensions();

                        width = px_to_mm(image_width as f32);
                        height = px_to_mm(image_height as f32);
                    }

                    if block_image.is_point_none() {
                        x = block_point.x;
                        y = block_point.y;

                        block_point.y += height;
                    }

                    block_image.set_bounds(Bounds::new(width, height, x, y));
                }
            }
        }
    }
}
