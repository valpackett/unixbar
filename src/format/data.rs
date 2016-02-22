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

#[macro_export]
macro_rules! bfmt {
    (raw[$str:expr]) => { Format::UnescapedStr($str.to_owned()) };
    (text[$str:expr]) => { Format::Str($str.to_owned()) };
    (fmt[$($rest:tt)*]) => { Format::Str(format!($($rest)*)) };
    (fg[$color:expr] $($rest:tt)*) => { Format::FgColor($color.to_owned(), Box::new(bfmt!($($rest)*))) };
    (bg[$color:expr] $($rest:tt)*) => { Format::BgColor($color.to_owned(), Box::new(bfmt!($($rest)*))) };
    (click[$btn:expr => $act:expr] $($rest:tt)*) => {
        Format::Clickable($btn, $act.to_owned(), Box::new(bfmt!($($rest)*)))
    };
    ($e:expr) => { $e };
}
