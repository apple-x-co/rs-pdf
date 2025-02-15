#[derive(Debug, Clone)]
pub struct Bounds {
    // pub width: Option<f32>,  // NOTE: mm
    // pub height: Option<f32>, // NOTE: mm
    // pub x: Option<f32>,      // NOTE: mm
    // pub y: Option<f32>,      // NOTE: mm
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

    pub fn min_x(&self) -> f32 {
        // self.x.unwrap()
        // self.point.as_ref().unwrap_or_default().x
        self.point.as_ref().unwrap_or(&Default::default()).x
    }

    pub fn max_x(&self) -> f32 {
        // self.x.unwrap() + self.width.unwrap()
        // self.point.as_ref().unwrap_or_default().x + self.size.as_ref().unwrap_or_default().width
        self.point.as_ref().unwrap_or(&Default::default()).x
            + self.size.as_ref().unwrap_or(&Default::default()).width
    }

    pub fn min_y(&self) -> f32 {
        // self.y.unwrap()
        // self.point.as_ref().unwrap_or_default().y
        self.point.as_ref().unwrap_or(&Default::default()).y
    }

    pub fn max_y(&self) -> f32 {
        // self.y.unwrap() + self.height.unwrap()
        // self.point.as_ref().unwrap_or_default().y + self.size.as_ref().unwrap_or_default().height
        self.point.as_ref().unwrap_or(&Default::default()).y
            + self.size.as_ref().unwrap_or(&Default::default()).height
    }

    // NOTE: 左上座標から左下座標に変換をする
    pub fn transform(&self, parent: Bounds) -> Bounds {
        // Bounds {
        //     width: self.width,
        //     height: self.height,
        //     x: self.x.map(|x| x + parent.x.unwrap_or(0.0)), // 親のX座標を加算
        //     y: self.y.map(|y| parent.height.unwrap_or(0.0) - self.y.unwrap() - self.height.unwrap_or(0.0) + parent.y.unwrap_or(0.0)), // Y座標の変換
        // }

        // Bounds {
        //     point: Some(Point {
        //         x: self.point.as_ref().unwrap_or_default().x + parent.point.as_ref().unwrap_or_default().x,
        //         y: parent.size.as_ref().unwrap_or_default().height
        //             - self.point.as_ref().unwrap_or_default().y
        //             - self.size.as_ref().unwrap_or_default().height
        //             + parent.point.as_ref().unwrap_or_default().y,
        //     }),
        //     size: self.size.clone(),
        // }

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
