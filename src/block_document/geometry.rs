#[derive(Debug, Clone)]
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

// #[derive(Debug, Default, Clone)]
// pub struct Margin {
//     pub top: f32,    // NOTE: mm
//     pub right: f32,  // NOTE: mm
//     pub bottom: f32, // NOTE: mm
//     pub left: f32,   // NOTE: mm
// }

#[derive(Debug, Default, Clone)]
pub struct Insets {
    pub top: f32,    // NOTE: mm
    pub right: f32,  // NOTE: mm
    pub bottom: f32, // NOTE: mm
    pub left: f32,   // NOTE: mm
}

impl Bounds {
    pub fn new(width: f32, height: f32, x: f32, y: f32) -> Bounds {
        Bounds {
            point: Some(Point { x, y }),
            size: Some(Size { width, height }),
        }
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

    pub fn inset(&self, insets: &Insets) -> Bounds {
        Bounds {
            point: Some(Point {
                x: self.point.as_ref().unwrap_or(&Default::default()).x + insets.left,
                y: self.point.as_ref().unwrap_or(&Default::default()).y + insets.top,
            }),
            size: Some(Size {
                width: self.size.as_ref().unwrap_or(&Default::default()).width
                    - insets.left
                    - insets.right,
                height: self.size.as_ref().unwrap_or(&Default::default()).height
                    - insets.top
                    - insets.bottom,
            }),
        }
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
