#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn is_horizontal(&self) -> bool {
        *self == Direction::Horizontal
    }

    pub fn is_vertical(&self) -> bool {
        *self == Direction::Vertical
    }
}