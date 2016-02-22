#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! unixbar = "0"
//! systemstat = "0"
//! ```

#[macro_use] extern crate unixbar;
extern crate systemstat;
use unixbar::*;
use systemstat::{System, Platform};

fn main() {
    UnixBar::new(LemonbarFormatter::new())

        .add(Text::new(bfmt![click[MouseButton::Left => "notify-send hi"]
                             click[MouseButton::Right => "notify-send 'what?'"]
                             fg["#11bb55"] text[" HI "]]))

        .add(Wrap::new(
             |f| bfmt![fg["#bb1155"] f],
             DateTime::new(" %Y-%m-%d %H:%M:%S %z ")))

        .add(Periodic::new(
             Duration::from_secs(2),
             || match System::new().memory() {
                 Ok(mem) => bfmt![bg["#556677"] fmt[" {}/{} RAM ", mem.free, mem.free + mem.active + mem.inactive + mem.wired + mem.cache]],
                 Err(_) => bfmt![fg["#bb1155"] text["error"]],
             }))

        .add(Delayed::new(
             Duration::from_secs(1),
             || System::new().cpu_load_aggregate().unwrap(),
             |res| match res {
                 Ok(cpu) => bfmt![fg["#99aaff"] fmt[" {:.2}% CPU ", (1.0 - cpu.idle) * 100.0]],
                 Err(_) => bfmt![fg["#bb1155"] text["error"]],
             }))

        .run();
}
