use super::data::*;

pub struct LemonbarFormatter;

impl Formatter for LemonbarFormatter {
    fn format(&self, data: &Format) -> String {
        match *data {
            Format::UnescapedStr(ref s) =>
                s.clone(),
            Format::Str(ref s) =>
                s.replace("%", "%%"),
            Format::Concat(ref fs) =>
                fs.iter().map(|f| self.format(f)).fold("".to_owned(), |a, b| a + &b),
            Format::Align(ref a, ref f) =>
                format!("{}{}", match *a {
                    Alignment::Left => "%{l}",
                    Alignment::Center => "%{c}",
                    Alignment::Right => "%{r}",
                }, self.format(f)),
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
