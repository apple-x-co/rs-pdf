use crate::block_document::geometry::GeoSize;
use crate::block_document::page::Page;
use crate::block_document::page_number::PageNumber;

pub const DPI: f32 = 300.0;

pub fn px_to_mm(pixel: f32) -> f32 {
    pixel / (DPI / 25.4)
}

#[derive(Clone)]
pub struct Document {
    pub title: String,
    pub page_size: GeoSize,
    pub font_path: String,
    pub page_number: Option<PageNumber>,
    pub pages: Vec<Page>,
}

impl Document {
    pub fn new(title: String, page_size: GeoSize, font_path: String) -> Document {
        Document {
            title,
            page_size,
            font_path,
            page_number: None,
            pages: Vec::new(),
        }
    }

    pub fn set_page_number(&mut self, page_number: PageNumber) {
        self.page_number = Some(page_number);
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }
}
