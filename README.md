[![crates.io](https://img.shields.io/crates/v/unixbar.svg)](https://crates.io/crates/unixbar)
[![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

# unixbar

A better way to set up your [lemonbar]/[dzen2]/[i3bar]: with [Rust] and [cargo-script] instead of shell scripting or dynamic languages.

![Screenshot](https://unrelentingtech.s3.dualstack.eu-west-1.amazonaws.com/unixbar.png)

Included widgets:

- date/time
- [systemstat] CPU/RAM/etc. measurements
- XKB current keyboard layout
- music via [D-Bus MPRIS] \(Rhythmbox, Clementine, Spotify, etc.) or [MPD]
- volume via Linux ALSA or FreeBSD mixer
- [bspwm] desktops

Features:

- colors
- alignment
- separator control (i3bar)
- click handlers (both shell and Rust code in i3bar, only shell in lemonbar and dzen2)
- possible to disable systemstat, XKB (libxcb), D-Bus from the build via Cargo features

[lemonbar]: https://github.com/LemonBoy/bar
[dzen2]: https://github.com/robm/dzen
[i3bar]: https://i3wm.org
[Rust]: https://www.rust-lang.org
[systemstat]: https://github.com/myfreeweb/systemstat
[D-Bus MPRIS]: https://specifications.freedesktop.org/mpris-spec/latest/
[MPD]: https://musicpd.org
[bspwm]: https://github.com/baskerville/bspwm
[cargo-script]: https://github.com/DanielKeep/cargo-script

## Usage

See [examples/demo.rs](https://github.com/myfreeweb/unixbar/blob/master/examples/demo.rs).

## Contributing

Please feel free to submit pull requests!

By participating in this project you agree to follow the [Contributor Code of Conduct](http://contributor-covenant.org/version/1/4/).

[The list of contributors is available on GitHub](https://github.com/myfreeweb/unixbar/graphs/contributors).

## License

This is free and unencumbered software released into the public domain.  
For more information, please refer to the `UNLICENSE` file or [unlicense.org](http://unlicense.org).
