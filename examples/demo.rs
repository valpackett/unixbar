extern crate unixbar;
use unixbar::*;
use unixbar::format::dzen2::Dzen2Formatter;
use unixbar::format::lemonbar::LemonbarFormatter;
use unixbar::widget::text::Text;
use unixbar::widget::wrap::Wrap;
use unixbar::widget::datetime::DateTime;

fn main() {
    UnixBar::new(LemonbarFormatter::new())
        .add(Text::new(Format::FgColor("#11bb55".to_owned(), Box::new(Format::Str("HI".to_owned())))))
        .add(Wrap::new(|f| Format::FgColor("#bb1155".to_owned(), Box::new(f)), DateTime::new("%Y-%m-%d %H:%M:%S %z")))
        .run();
}
