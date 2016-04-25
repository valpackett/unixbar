use std::sync::mpsc::Sender;
use std::time::Duration;
use std::process::{Command, Stdio};
use std::thread;
use std::sync::{Arc, RwLock};
use std::io::{BufReader, BufRead};
use super::base::Widget;
use format::data::Format;
use nom::IResult;

#[derive(Debug, Clone)]
pub struct BspwmDesktop {
    pub name: String,
    pub occupied: bool,
    pub focused: bool,
    pub urgent: bool,
}

named!(bspstr<&[u8], Vec<BspwmDesktop> >,
   chain!(
       tag!("WM") ~
       take_until_and_consume!(":") ~
       d: many0!(
           chain!(
               mode: take!(1) ~
               name: take_until_and_consume!(":") ,
               || BspwmDesktop {
                   name: String::from_utf8_lossy(name).into_owned(),
                   occupied: mode == b"o" || mode == b"O",
                   focused: mode == b"F" || mode == b"O" || mode == b"U",
                   urgent: mode == b"u" || mode == b"U",
               }
           )
       ) ~
       tag!("LT") ,
       || d
   )
);

pub struct Bspwm<F: Fn(Vec<BspwmDesktop>) -> Format> {
    updater: Arc<Box<F>>,
    last_value: Arc<RwLock<Format>>,
}

impl<F> Widget for Bspwm<F> where F: Fn(Vec<BspwmDesktop>) -> Format + Sync + Send + 'static  {
    fn current_value(&self) -> Format {
        (*self.last_value).read().unwrap().clone()
    }

    fn spawn_notifier(&mut self, tx: Sender<()>) {
        let updater = self.updater.clone();
        let last_value = self.last_value.clone();
        thread::spawn(move || {
            loop {
                // Should be possible to use the socket directly...
                let bspc = Command::new("bspc").arg("subscribe")
                    .stdout(Stdio::piped()).spawn().expect("Couldn't run bspc");
                for line in BufReader::new(bspc.stdout.unwrap()).lines() {
                    let mut writer = last_value.write().unwrap();
                    let line = line.unwrap_or("".to_owned());
                    if let IResult::Done(_, result) = bspstr(&line.into_bytes()) {
                        *writer = (*updater)(result);
                        let _ = tx.send(());
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
    }
}

impl<F> Bspwm<F> where F: Fn(Vec<BspwmDesktop>) -> Format {
    pub fn new(updater: F) -> Box<Bspwm<F>> {
        Box::new(Bspwm {
            updater: Arc::new(Box::new(updater)),
            last_value: Arc::new(RwLock::new(Format::Str("".to_owned()))),
        })
    }
}
