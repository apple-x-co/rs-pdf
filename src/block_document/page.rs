use crate::block_document::dynamic_page::DynamicPage;
use crate::block_document::static_page::StaticPage;

#[derive(Debug, Clone)]
pub enum Page {
    DynamicPage(DynamicPage),
    StaticPage(StaticPage)
}