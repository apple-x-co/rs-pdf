use crate::block_document::style::Space;

#[derive(Debug, Default, Clone)]
pub struct GeoRect {
    pub point: Option<GeoPoint>,
    pub size: Option<GeoSize>,
}

#[derive(Debug, Default, Clone)]
pub struct GeoPoint {
    pub x: f32, // NOTE: mm
    pub y: f32, // NOTE: mm
}

#[derive(Debug, Default, Clone)]
pub struct GeoSize {
    pub width: f32,  // NOTE: mm
    pub height: f32, // NOTE: mm
}

impl GeoRect {
    pub fn new(width: f32, height: f32, x: f32, y: f32) -> GeoRect {
        GeoRect {
            point: Some(GeoPoint { x, y }),
            size: Some(GeoSize { width, height }),
        }
    }

    pub fn zero() -> GeoRect {
        GeoRect {
            point: Some(GeoPoint { x: 0.0, y: 0.0 }),
            size: Some(GeoSize {
                width: 0.0,
                height: 0.0,
            }),
        }
    }

    pub fn none() -> GeoRect {
        GeoRect {
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
    pub fn inset(&self, space: &Space) -> GeoRect {
        GeoRect {
            point: Some(GeoPoint {
                x: self.point.as_ref().unwrap_or(&Default::default()).x + space.left,
                y: self.point.as_ref().unwrap_or(&Default::default()).y + space.top,
            }),
            size: Some(GeoSize {
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
    pub fn union(&self, geo_rect: &GeoRect) -> GeoRect {
        GeoRect::new(
            if self.max_x() > geo_rect.max_x() {
                self.max_x() - geo_rect.min_x()
            } else {
                geo_rect.max_x() - self.min_x()
            },
            if self.max_y() > geo_rect.max_y() {
                self.max_y() - geo_rect.min_y()
            } else {
                geo_rect.max_y() - self.min_y()
            },
            if self.min_x() < geo_rect.min_x() {
                self.min_x()
            } else {
                geo_rect.min_x()
            },
            if self.min_y() < geo_rect.min_y() {
                self.min_y()
            } else {
                geo_rect.min_y()
            },
        )
    }

    // NOTE: 左上座標から左下座標に変換をする
    pub fn transform(&self, parent: &GeoRect) -> GeoRect {
        GeoRect {
            point: Some(GeoPoint {
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

impl GeoSize {
    pub fn new(width: f32, height: f32) -> GeoSize {
        GeoSize { width, height }
    }
}
