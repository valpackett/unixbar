extern crate unixbar;
extern crate systemstat;
use unixbar::*;
use systemstat::{System, Platform};

fn main() {
    UnixBar::new(LemonbarFormatter::new())
        .add(Text::new(Format::FgColor("#11bb55".to_owned(), Box::new(Format::Str("HI ".to_owned())))))
        .add(Wrap::new(|f| Format::FgColor("#bb1155".to_owned(), Box::new(f)),
                       DateTime::new("%Y-%m-%d %H:%M:%S %z")))
        .add(Periodic::new(Duration::from_secs(2), || match System::new().memory() {
            Ok(mem) => Format::Str(format!(" {} free, {} active",
                                           mem.free.to_string(true), mem.active.to_string(true))),
            Err(_) => Format::FgColor("#bb1155".to_owned(), Box::new(Format::Str("error".to_owned()))),
        }))
        .add(Delayed::new(Duration::from_secs(2),
                          || System::new().cpu_load_aggregate().unwrap(),
                          |res| match res {
                              Ok(cpu) => Format::FgColor("#99aaff".to_owned(), Box::new(Format::Str(format!(" {}% CPU", (1.0 - cpu.idle) * 100.0)))),
                              Err(_) => Format::FgColor("#bb1155".to_owned(), Box::new(Format::Str("error".to_owned()))),
                          }))
        .run();
}
