use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Duration;
use super::base::Widget;
use format::data::Format;

pub mod mpd;
pub use self::mpd::MPDMusic;

#[derive(Debug, Clone)]
pub struct PlaybackInfo {
    /// playing or paused
    pub playing: bool,

    pub progress: Duration,
    pub total: Duration,

    pub playlist_index: u32,
    pub playlist_total: u32,
}

#[derive(Debug, Clone)]
pub struct SongInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub filename: String,
    pub playback: Option<PlaybackInfo>
}

pub struct Music<F: Fn(SongInfo) -> Format, B: MusicBackend<F>> {
    updater: Arc<Box<F>>,
    backend: B,
}

impl<F, B> Widget for Music<F, B> where F: Fn(SongInfo) -> Format + Sync + Send + 'static, B: MusicBackend<F> {
    fn current_value(&self) -> Format {
        self.backend.current_value()
    }

    fn spawn_notifier(&mut self, tx: Sender<()>) {
        self.backend.spawn_notifier(tx, self.updater.clone());
    }
}

impl<F, B> Music<F, B> where F: Fn(SongInfo) -> Format, B: MusicBackend<F> {
    pub fn new(backend: B, updater: F) -> Box<Music<F, B>> {
        Box::new(Music {
            updater: Arc::new(Box::new(updater)),
            backend: backend,
        })
    }
}

pub trait MusicBackend<F: Fn(SongInfo) -> Format> {
    fn current_value(&self) -> Format;
    fn spawn_notifier(&mut self, tx: Sender<()>, updater: Arc<Box<F>>);
}
