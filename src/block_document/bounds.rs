#[derive(Debug)]
pub struct Bounds {
    pub width: Option<f32>,  // NOTE: mm
    pub height: Option<f32>, // NOTE: mm
    pub x: Option<f32>,      // NOTE: mm
    pub y: Option<f32>,      // NOTE: mm
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
    pub fn translate_lb(&self, parent: Bounds) -> Bounds {
        Bounds {
            width: self.width,
            height: self.height,
            x: self.x,
            y: Some(parent.max_y() - self.height.unwrap() - self.y.unwrap()), // NOTE: あってる?
        }
    }
}
