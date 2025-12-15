use crate::block_document::geometry::GeoSize;
use crate::block_document::page::Page;

pub const DPI: f32 = 300.0;

pub fn px_to_mm(pixel: f32) -> f32 {
    pixel / (DPI / 25.4)
}

#[derive(Clone)]
pub struct Document {
    pub title: String,
    pub page_size: GeoSize,
    pub font_path: String,
    pub pages: Vec<Page>,
}

impl Document {
    pub fn new(title: String, page_size: GeoSize, font_path: String) -> Document {
        Document {
            title,
            page_size,
            font_path,
            pages: Vec::new(),
        }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }
}
