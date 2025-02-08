pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub struct Coordinate {
    pub x: u64,
    pub y: u64,
}

pub struct Block {
    blocks: Vec<Block>,
    direction: Direction,
    size: Option<Size>,
    coordinate: Option<Coordinate>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            blocks: Vec::new(),
            direction: Direction::Horizontal,
            size: None,
            coordinate: None,
        }
    }
}
