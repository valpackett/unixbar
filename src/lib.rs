extern crate chrono;

pub mod format;
pub mod widget;

use std::sync::mpsc::channel;
pub use format::data::{Format, Formatter};
pub use widget::base::Widget;

pub struct UnixBar<F: Formatter> {
    formatter: F,
    widgets: Vec<Box<Widget>>,
}

impl<F: Formatter> UnixBar<F> {
    pub fn new(formatter: F) -> UnixBar<F> {
        UnixBar {
            formatter: formatter,
            widgets: Vec::new(),
        }
    }

    pub fn add(&mut self, widget: Box<Widget>) -> &mut UnixBar<F> {
        self.widgets.push(widget); self
    }

    pub fn run(&mut self) {
        let (tx, rx) = channel();
        for widget in &mut self.widgets {
            widget.spawn_notifier(tx.clone());
        }
        self.show();
        for _ in rx.iter() {
            self.show();
        }
    }

    fn show(&mut self) {
        let mut line = String::new();
        for widget in &mut self.widgets {
            line.push_str(self.formatter.format(&widget.current_value()).as_ref());
        }
        println!("{}", line.replace("\n", ""));
    }
}
