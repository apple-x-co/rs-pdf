#[derive(Debug)]
pub struct Bounds {
    pub width: Option<f32>,  // NOTE: mm
    pub height: Option<f32>, // NOTE: mm
    pub x: Option<f32>,      // NOTE: mm
    pub y: Option<f32>,      // NOTE: mm
}

pub struct Point {
    pub x: f32,  // NOTE: mm
    pub y: f32,  // NOTE: mm
}

pub struct Size {
    pub width: f32,  // NOTE: mm
    pub height: f32,  // NOTE: mm
}

impl Bounds {
    pub fn min_x(&self) -> f32 {
        self.x.unwrap()
    }

    pub fn max_x(&self) -> f32 {
        self.x.unwrap() + self.width.unwrap()
    }

    pub fn min_y(&self) -> f32 {
        self.y.unwrap()
    }

    pub fn max_y(&self) -> f32 {
        self.y.unwrap() + self.height.unwrap()
    }

    // NOTE: 左上座標から左下座標に変換をする
    pub fn transform(&self, parent: Bounds) -> Bounds {
        Bounds {
            width: self.width,
            height: self.height,
            x: self.x.map(|x| x + parent.x.unwrap_or(0.0)), // 親のX座標を加算
            y: self.y.map(|y| parent.height.unwrap_or(0.0) - self.y.unwrap() - self.height.unwrap_or(0.0) + parent.y.unwrap_or(0.0)), // Y座標の変換
        }
    }
}
