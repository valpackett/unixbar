use std::sync::mpsc::Sender;
use std::time::Duration;
use std::process::{Command, Stdio};
use std::thread;
use std::sync::{Arc, RwLock};
use std::io::{BufReader, BufRead};
use super::base::Widget;
use format::data::Format;
use nom::IResult;


#[derive(Debug,Clone)]
pub struct PlaybackInfo {
    pub playing: bool,

    pub progress: Duration,
    pub total: Duration,

    pub playlist_index: u32,
    pub playlist_total: u32,
}
#[derive(Debug,Clone)]
pub struct SongInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub filename: String,
    pub playback: PlaybackInfo
}

named!(parse_playback_info<&[u8], PlaybackInfo>,
    do_parse!(
        tag!("[") >>
        playing: take_until!("]") >>
        take_until_and_consume!("#") >>
        playlist_index: take_until!("/") >>
        take!(1) >>
        playlist_total: take_until!(" ") >>
        take!(1) >>
        progress_m: ws!(take_until!(":")) >>
        take!(1) >>
        progress_s: take_until!("/") >>
        take!(1) >>
        total_m: take_until!(":") >>
        take!(1) >>
        total_s: take_until!(" ") >>


        ( PlaybackInfo {
            playing: (String::from_utf8_lossy(playing) == "playing"),

            progress: Duration::from_secs(
                String::from_utf8_lossy(progress_m).parse::<u64>().unwrap() * 60
                + String::from_utf8_lossy(progress_s).parse::<u64>().unwrap()),
            total: Duration::from_secs(
                String::from_utf8_lossy(total_m).parse::<u64>().unwrap() * 60
                + String::from_utf8_lossy(total_s).parse::<u64>().unwrap()),

            playlist_index:
                String::from_utf8_lossy(playlist_index).parse().unwrap(),
            playlist_total:
                String::from_utf8_lossy(playlist_total).parse().unwrap()
        })
    )
);


pub struct Mpd<F: Fn(SongInfo) -> Format> {
    updater: Arc<Box<F>>,
    last_value: Arc<RwLock<Format>>,
}

impl<F> Widget for Mpd<F> where F: Fn(SongInfo) -> Format + Sync + Send + 'static  {
    fn current_value(&self) -> Format {
        (*self.last_value).read().unwrap().clone()
    }

    fn spawn_notifier(&mut self, tx: Sender<()>) {
        let updater = self.updater.clone();
        let last_value = self.last_value.clone();
        thread::spawn(move || {
            loop {
                // Wait for event
                let _ = Command::new("mpc").arg("idle").arg("player").status();

                let title = mpc_get_format("%title%").unwrap_or("".to_owned());
                let artist = mpc_get_format("%artist%").unwrap_or("".to_owned());
                let album = mpc_get_format("%album%").unwrap_or("".to_owned());
                let filename = mpc_get_format("%file%").unwrap_or("".to_owned());

                let playback_info = get_playback_info().unwrap();

                let state = SongInfo {
                    title: title,
                    artist: artist,
                    album: album,
                    filename: filename,
                    playback: playback_info,
                };

                let mut writer = last_value.write().unwrap();
                *writer = (*updater)(state);
                let _ = tx.send(());
            }
        });
    }
}

impl<F> Mpd<F> where F: Fn(SongInfo) -> Format {
    pub fn new(updater: F) -> Box<Mpd<F>> {
        Box::new(Mpd {
            updater: Arc::new(Box::new(updater)),
            last_value: Arc::new(RwLock::new(Format::Str("".to_owned()))),
        })
    }
}


fn mpc_get_format(format: &'static str) -> Option<String> {
    let mpc = Command::new("mpc").arg("-f").arg(format)
        .stdout(Stdio::piped()).spawn()
        .expect("Couldn't run `mpc -f`");

    BufReader::new(mpc.stdout.unwrap()).lines().next()
        .and_then(|result| result.ok())
}


fn get_playback_info() -> Option<PlaybackInfo> {
    let mpc = Command::new("mpc")
        .stdout(Stdio::piped()).spawn()
        .expect("Couldn't run `mpc` to get song info");
    for line in BufReader::new(mpc.stdout.unwrap()).lines() {
        let line = line.unwrap_or("".to_owned());
        match parse_playback_info(&line.into_bytes()) {
            IResult::Done(_, playback_info) => return Some(playback_info),
            _ => continue,
        }
    }
    None
}
