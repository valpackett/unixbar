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

        .add(Volume::new(ALSA::new(),
            |volume|
                match volume.muted {
                    false => bfmt![fmt["Volume {}", (volume.volume * 100.0) as i32]],
                    true => bfmt![fmt["(muted) {}", (volume.volume * 100.0) as i32]]
                }
        ))
        .add(Mpd::new(
            |mpd| {
                if let Some(true) = mpd.playback.map(|playback| playback.playing) {
                    if mpd.title == "" {
                        bfmt![fmt["{}", mpd.filename]]
                    } else if mpd.artist == "" {
                        bfmt![fmt["{}", mpd.title]]
                    } else {
                        bfmt![fmt["{} - {}", mpd.title, mpd.artist ]]
                    }
                } else {
                    bfmt![text["(paused)"]]
                }
            }
        ))
        .add(Text::new(bfmt![click[MouseButton::Left => "notify-send hi"]
                             click[MouseButton::Right => "notify-send 'what?'"]
                             fg["#11bb55"] text[" Hello World! "]]))

        .add(Bspwm::new(|bsp| Format::Concat(bsp.desktops.iter().map(|d| Box::new({
                let bg = if d.focused { "#99aa11" } else { "#111111" };
                bfmt![click[MouseButton::Left => format!("bspc desktop -f {}", d.name)]
                      bg[bg] fmt[" {} ", d.name]]
            })).collect())))

        .add(Text::new(bfmt![right]))

        .add(Periodic::new(
             Duration::from_secs(2),
             || match System::new().memory() {
                 Ok(mem) => bfmt![bg["#556677"] fmt[" {}/{} RAM ", mem.free.to_string(false).replace(" GB", ""), mem.total]],
                 Err(_) => bfmt![fg["#bb1155"] text["error"]],
             }))

        .add(Delayed::new(
             Duration::from_secs(1),
             || System::new().cpu_load_aggregate().unwrap(),
             |res| match res {
                 Ok(cpu) => bfmt![fg["#99aaff"] fmt[" {:04.1}% CPU ", (1.0 - cpu.idle) * 100.0]],
                 Err(_) => bfmt![fg["#bb1155"] text["error"]],
             }))

        .add(Wrap::new(
             |f| bfmt![fg["#bb1155"] f],
             DateTime::new(" %Y-%m-%d %H:%M:%S %z ")))

        .run();
}
