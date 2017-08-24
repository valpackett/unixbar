use alsa;
use alsa::ctl::Ctl;
use alsa::PollDescriptors;
use alsa::mixer::{Mixer, SelemId, SelemChannelId};

use super::base::Widget;
use format::data::Format;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::thread;
use std::ffi::CString;

use libc::{pollfd, poll}; // TODO use mio/epoll?


pub struct VolumeState {
    pub volume: f32,
    pub muted: bool,
}


pub struct Volume<F: Fn(VolumeState) -> Format> {
    updater: Arc<Box<F>>,
    last_value: Arc<RwLock<Format>>,
}

impl<F> Widget for Volume<F> where F: Fn(VolumeState) -> Format + Sync + Send + 'static  {
    fn current_value(&self) -> Format {
        (*self.last_value).read().unwrap().clone()
    }

    fn spawn_notifier(&mut self, tx: Sender<()>) {
        let ctl =Ctl::open(CString::new("default").unwrap().as_ref(), false).unwrap();
        ctl.subscribe_events(true);
            

        let mut fds = Vec::<pollfd>::with_capacity(ctl.count());
        fds.resize(ctl.count(), pollfd {fd: 0, events: 0, revents: 0});
        let filled = ctl.fill(&mut fds).unwrap();
        assert!(filled == ctl.count());


        let updater = self.updater.clone();
        let last_value = self.last_value.clone();
        thread::spawn(move || {
            loop {
                unsafe {
                    let err = poll(fds.as_mut_ptr(), fds.len() as u64, -1);
                    // TODO check error
                }
                
                let events = match ctl.read() {
                    Ok(Some(event)) => {
                        let state = Volume::<F>::get_alsa_state();

                        let mut writer = last_value.write().unwrap();
                        *writer = (*updater)(state);
                        let _ = tx.send(());
                    },
                    _ => continue
                };

            }
        });
    }

}

impl<F> Volume<F> where F: Fn(VolumeState) -> Format {
    pub fn new(updater: F) -> Box<Volume<F>> {
        Box::new(Volume {
            updater: Arc::new(Box::new(updater)),
            last_value: Arc::new(RwLock::new(Format::Str("".to_owned()))),
        })
    }
    fn get_alsa_state() -> VolumeState {
        let mixer = Mixer::new("default", false).unwrap();
        let selem_id = SelemId::new("Master", 0);
        let selem = mixer.find_selem(&selem_id).unwrap();
        let (min, max) = selem.get_playback_volume_range();
        let volume = selem.get_playback_volume(SelemChannelId::FrontLeft).unwrap();
        let switch = selem.get_playback_switch(SelemChannelId::FrontLeft).unwrap();
        VolumeState {
            volume: (volume as f64 / (max - min) as f64) as f32,
            muted: switch == 0,
        }
    }
}
