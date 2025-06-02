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
    Alignment(Alignment)
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

#[derive(Debug, Default, Clone)]
pub struct Space {
    pub top: f32,    // NOTE: mm
    pub right: f32,  // NOTE: mm
    pub bottom: f32, // NOTE: mm
    pub left: f32,   // NOTE: mm
}

#[derive(Debug, Clone)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct Alignment {
    pub horizontal: Option<HorizontalAlignment>,
    pub vertical: Option<VerticalAlignment>,
}
