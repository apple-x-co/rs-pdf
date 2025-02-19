use crate::block_document::geometry::Space;

#[derive(Debug, Clone)]
pub enum Style {
    TextFillColor(RgbColor),
    TextOutlineColor(RgbColor),
    TextStyle(TextStyle),
    TextOutlineStyle(TextOutlineStyle),
    BackgroundColor(RgbColor),
    BorderColor(RgbColor),
    BorderWidth(f32),
    BorderStyle(BorderStyle),
    Space(Space),
}

#[derive(Debug, Clone)]
pub struct RgbColor {
    pub r: u8, // NOTE: 0-255
    pub g: u8, // NOTE: 0-255
    pub b: u8, // NOTE: 0-255
}

#[derive(Debug, Clone)]
pub enum TextStyle {
    Fill,
    Stroke,
    FillStroke,
}

#[derive(Debug, Clone)]
pub enum TextOutlineStyle {
    Solid,
    Dash(i64),
}

#[derive(Debug, Clone)]
pub enum BorderStyle {
    Solid,
    Dash(i64),
}
