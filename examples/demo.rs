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

        .add(Volume::new(default_volume(),
            |volume|
                match volume.muted {
                    false => bfmt![fmt["Volume {}", (volume.volume * 100.0) as i32]],
                    true => bfmt![fmt["(muted) {}", (volume.volume * 100.0) as i32]]
                }
        ))

        .register_fn("play_pause", move || { MPRISMusic::new().play_pause(); })
        .add(Music::new(MPRISMusic::new(),
            |song| {
                let paused = song.playback.map(|playback| playback.playing) == Some(true);
                bfmt![click[MouseButton::Left => fn "play_pause"]
                      fmt["{} {} - {}", if paused { "PAUSED" } else { "PLAYING" }, song.title, song.artist]]
            }
        ))

        .add(Text::new(bfmt![click[MouseButton::Left => sh "notify-send hi"]
                             click[MouseButton::Right => sh "notify-send 'what?'"]
                             fg["#11bb55"] text[" Hello World! "]]))

        .add(Bspwm::new(|bsp| Format::Concat(bsp.desktops.iter().map(|d| Box::new({
                let bg = if d.focused { "#99aa11" } else { "#111111" };
                bfmt![click[MouseButton::Left => sh format!("bspc desktop -f {}", d.name)]
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
