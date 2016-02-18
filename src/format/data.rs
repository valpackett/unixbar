#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
pub enum Format {
    UnescapedStr(String),
    Str(String),
    FgColor(String, Box<Format>),
    BgColor(String, Box<Format>),
    Clickable(MouseButton, String, Box<Format>),
}

pub trait Formatter {
    fn format(&self, data: &Format) -> String;
}
