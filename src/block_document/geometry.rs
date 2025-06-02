use crate::block_document::style::Space;

#[derive(Debug, Default, Clone)]
pub struct Bounds {
    pub point: Option<Point>,
    pub size: Option<Size>,
}

#[derive(Debug, Default, Clone)]
pub struct Point {
    pub x: f32, // NOTE: mm
    pub y: f32, // NOTE: mm
}

#[derive(Debug, Default, Clone)]
pub struct Size {
    pub width: f32,  // NOTE: mm
    pub height: f32, // NOTE: mm
}

impl Bounds {
    pub fn new(width: f32, height: f32, x: f32, y: f32) -> Bounds {
        Bounds {
            point: Some(Point { x, y }),
            size: Some(Size { width, height }),
        }
    }

    pub fn zero() -> Bounds {
        Bounds {
            point: Some(Point { x: 0.0, y: 0.0 }),
            size: Some(Size {
                width: 0.0,
                height: 0.0,
            }),
        }
    }

    pub fn none() -> Bounds {
        Bounds {
            point: None,
            size: None,
        }
    }

    pub fn width(&self) -> f32 {
        self.size.as_ref().map(|s| s.width).unwrap_or(0.0)
    }

    pub fn height(&self) -> f32 {
        self.size.as_ref().map(|s| s.height).unwrap_or(0.0)
    }

    pub fn min_x(&self) -> f32 {
        self.point.as_ref().unwrap_or(&Default::default()).x
    }

    pub fn max_x(&self) -> f32 {
        self.point.as_ref().unwrap_or(&Default::default()).x
            + self.size.as_ref().unwrap_or(&Default::default()).width
    }

    pub fn min_y(&self) -> f32 {
        self.point.as_ref().unwrap_or(&Default::default()).y
    }

    pub fn max_y(&self) -> f32 {
        self.point.as_ref().unwrap_or(&Default::default()).y
            + self.size.as_ref().unwrap_or(&Default::default()).height
    }

    // NOTE: 内側に余白を作る
    pub fn inset(&self, space: &Space) -> Bounds {
        Bounds {
            point: Some(Point {
                x: self.point.as_ref().unwrap_or(&Default::default()).x + space.left,
                y: self.point.as_ref().unwrap_or(&Default::default()).y + space.top,
            }),
            size: Some(Size {
                width: self.size.as_ref().unwrap_or(&Default::default()).width
                    - space.left
                    - space.right,
                height: self.size.as_ref().unwrap_or(&Default::default()).height
                    - space.top
                    - space.bottom,
            }),
        }
    }

    // NOTE: 2の矩形を満たす新しい矩形を作る
    pub fn union(&self, bounds: &Bounds) -> Bounds {
        Bounds::new(
            if self.max_x() > bounds.max_x() {
                self.max_x() - bounds.min_x()
            } else {
                bounds.max_x() - self.min_x()
            },
            if self.max_y() > bounds.max_y() {
                self.max_y() - bounds.min_y()
            } else {
                bounds.max_y() - self.min_y()
            },
            if self.min_x() < bounds.min_x() {
                self.min_x()
            } else {
                bounds.min_x()
            },
            if self.min_y() < bounds.min_y() {
                self.min_y()
            } else {
                bounds.min_y()
            },
        )
    }

    // NOTE: 左上座標から左下座標に変換をする
    pub fn transform(&self, parent: &Bounds) -> Bounds {
        Bounds {
            point: Some(Point {
                x: self.point.as_ref().unwrap_or(&Default::default()).x
                    + parent.point.as_ref().unwrap_or(&Default::default()).x,
                y: parent.size.as_ref().unwrap_or(&Default::default()).height
                    - self.point.as_ref().unwrap_or(&Default::default()).y
                    - self.size.as_ref().unwrap_or(&Default::default()).height
                    + parent.point.as_ref().unwrap_or(&Default::default()).y,
            }),
            size: self.size.clone(),
        }
    }
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size { width, height }
    }
}
