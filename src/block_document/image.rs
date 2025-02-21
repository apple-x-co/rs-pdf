use crate::block_document::geometry::Bounds;
use crate::block_document::style::Style;

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub bounds: Option<Bounds>,
    pub styles: Vec<Style>,
}

impl Image {
    pub fn new(path: String, bounds: Option<Bounds>) -> Image {
        Image {
            path,
            bounds,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn is_point_none(&self) -> bool {
        self.bounds.is_none() || self.bounds.as_ref().unwrap().point.is_none()
    }

    pub fn is_size_none(&self) -> bool {
        self.bounds.is_none() || self.bounds.as_ref().unwrap().size.is_none()
    }

    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = Some(bounds);
    }
}
