#[cfg(feature = "xkb")] extern crate pkg_config;

fn main() {
    if cfg!(feature = "xkb") {
        pkg_config::find_library("xcb").unwrap();
    }
}
