use std::sync::mpsc::Sender;
use std::time::Duration;
use std::thread;
use chrono::Local;
use super::base::Widget;
use format::data::Format;

pub struct DateTime {
    format: String
}

impl Widget for DateTime {
    fn current_value(&self) -> Format {
        Format::Str(Local::now().format(&self.format).to_string())
    }

    fn spawn_notifier(&mut self, tx: Sender<()>) {
        let seconds = if self.format.contains("%S") { 1 } else { 60 };
        thread::spawn(move|| {
            loop {
                thread::sleep(Duration::from_secs(seconds));
                let _ = tx.send(());
            }
        });
    }
}

impl DateTime {
    pub fn new(format: &str) -> Box<DateTime> {
        Box::new(DateTime { format: format.to_owned() })
    }
}
