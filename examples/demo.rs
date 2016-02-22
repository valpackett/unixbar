extern crate unixbar;
extern crate systemstat;
use unixbar::*;
use systemstat::{System, Platform};

fn main() {
    UnixBar::new(LemonbarFormatter::new())
        .add(Text::new(Format::FgColor("#11bb55".to_owned(), Box::new(Format::Str("HI ".to_owned())))))
        .add(Wrap::new(|f| Format::FgColor("#bb1155".to_owned(), Box::new(f)),
                       DateTime::new("%Y-%m-%d %H:%M:%S %z")))
        .add(Periodic::new(Duration::from_secs(2), || {
            let mem = System::new().memory().unwrap();
            Format::Str(format!(" {} free, {} active",
                                mem.free.to_string(true), mem.active.to_string(true)))
        }))
        .run();
}
