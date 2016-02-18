use std::sync::mpsc::Sender;
use format::data::Format;

pub trait Widget {
    fn current_value(&self) -> Format;
    fn spawn_notifier(&mut self, tx: Sender<()>);
}
