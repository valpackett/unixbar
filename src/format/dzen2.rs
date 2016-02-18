use super::data::*;

pub struct Dzen2Formatter;

impl Formatter for Dzen2Formatter {
    fn format(&self, data: &Format) -> String {
        match *data {
            Format::UnescapedStr(ref s) =>
                s.clone(),
            Format::Str(ref s) =>
                s.replace("^", "^^"),
            Format::FgColor(ref c, ref f) =>
                format!("^fg({}){}^fg()", c, self.format(f)),
            Format::BgColor(ref c, ref f) =>
                format!("^bg({}){}^bg()", c, self.format(f)),
            Format::Clickable(ref mb, ref a, ref f) =>
                format!("^ca({}, {}){}^ca()", *mb as usize, a, self.format(f)),
        }
    }
}

impl Dzen2Formatter {
    pub fn new() -> Dzen2Formatter {
        Dzen2Formatter
    }
}
