#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub enum Format {
    UnescapedStr(String),
    Str(String),
    Concat(Vec<Box<Format>>),
    Align(Alignment, Box<Format>),
    FgColor(String, Box<Format>),
    BgColor(String, Box<Format>),
    Clickable(MouseButton, String, Box<Format>),
}

pub trait Formatter {
    fn format(&self, data: &Format) -> String;

    fn format_all(&self, data: &[Format]) -> String {
        let mut line = String::new();
        for f in data {
            line.push_str(self.format(f).as_ref());
        }
        line
    }
}

#[macro_export]
macro_rules! bfmt {
    (raw[$str:expr]) => { Format::UnescapedStr($str.to_owned()) };
    (text[$str:expr]) => { Format::Str($str.to_owned()) };
    (fmt[$($rest:tt)*]) => { Format::Str(format!($($rest)*)) };
    (left $($rest:tt)*) => { Format::Align(Alignment::Left, Box::new(bfmt!($($rest)*))) };
    (center $($rest:tt)*) => { Format::Align(Alignment::Center, Box::new(bfmt!($($rest)*))) };
    (right $($rest:tt)*) => { Format::Align(Alignment::Right, Box::new(bfmt!($($rest)*))) };
    (fg[$color:expr] $($rest:tt)*) => { Format::FgColor($color.to_owned(), Box::new(bfmt!($($rest)*))) };
    (bg[$color:expr] $($rest:tt)*) => { Format::BgColor($color.to_owned(), Box::new(bfmt!($($rest)*))) };
    (click[$btn:expr => $act:expr] $($rest:tt)*) => {
        Format::Clickable($btn, $act.to_owned(), Box::new(bfmt!($($rest)*)))
    };
    ($e:expr) => { $e };
    () => { Format::UnescapedStr("".to_owned()) };
}
