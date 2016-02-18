use super::data::*;

pub struct LemonbarFormatter;

impl Formatter for LemonbarFormatter {
    fn format(&self, data: &Format) -> String {
        match *data {
            Format::UnescapedStr(ref s) =>
                s.clone(),
            Format::Str(ref s) =>
                s.replace("%", "%%"),
            Format::FgColor(ref c, ref f) =>
                format!("%{{F{}}}{}%{{F-}}", c, self.format(f)),
            Format::BgColor(ref c, ref f) =>
                format!("%{{B{}}}{}%{{B-}}", c, self.format(f)),
            Format::Clickable(ref mb, ref a, ref f) =>
                format!("%{{A{}:{}:}}{}%{{A}}", mouse_button(mb), a.replace(":", "\\:"), self.format(f)),
        }
    }
}

fn mouse_button(mb: &MouseButton) -> usize {
    match *mb {
        MouseButton::Left => 1,
        MouseButton::Middle => 2,
        MouseButton::Right => 3,
    }
}

impl LemonbarFormatter {
    pub fn new() -> LemonbarFormatter {
        LemonbarFormatter
    }
}
