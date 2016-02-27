use super::data::*;

pub struct Dzen2Formatter;

impl Formatter for Dzen2Formatter {
    fn format(&self, data: &Format) -> String {
        match *data {
            Format::UnescapedStr(ref s) =>
                s.clone(),
            Format::Str(ref s) =>
                s.replace("^", "^^"),
            Format::Concat(ref fs) =>
                fs.iter().map(|f| self.format(f)).fold("".to_owned(), |a, b| a + &b),
            Format::Align(_, ref f) =>
                self.format(f), // :-(
            Format::FgColor(ref c, ref f) =>
                format!("^fg({}){}^fg()", c, self.format(f)),
            Format::BgColor(ref c, ref f) =>
                format!("^bg({}){}^bg()", c, self.format(f)),
            Format::Clickable(ref mb, ref a, ref f) =>
                format!("^ca({}, {}){}^ca()", mouse_button(mb), a, self.format(f)),
        }
    }
}

fn mouse_button(mb: &MouseButton) -> usize {
    match *mb {
        MouseButton::Left => 1,
        MouseButton::Right => 2,
        MouseButton::Middle => 3,
    }
}

impl Dzen2Formatter {
    pub fn new() -> Dzen2Formatter {
        Dzen2Formatter
    }
}
