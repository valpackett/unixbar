use super::data::*;

pub struct LemonbarFormatter {
    escape: bool,
}

impl Formatter for LemonbarFormatter {
    fn format(&mut self, data: &Format) -> String {
        match *data {
            Format::UnescapedStr(ref s) =>
                s.clone(),
            Format::Str(ref s) =>
                if self.escape { s.replace("%", "%%") } else { s.to_owned() },
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
            Format::NoSeparator(ref f) => self.format(f),
            Format::Padding(_, ref f) => self.format(f),
            Format::Clickable(ref act, ref f) =>
                match act {
                    &ClickAction::ShellCommand(ref mb, ref a) =>
                        format!("%{{A{}:{}:}}{}%{{A}}", mouse_button(mb), a.replace(":", "\\:"), self.format(f)),
                    _ => self.format(f), // TODO
                }
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
        LemonbarFormatter { escape: true }
    }

    /// Turn off escaping for e.g. lemonbar-xft
    pub fn new_noescape() -> LemonbarFormatter {
        LemonbarFormatter { escape: false }
    }
}
