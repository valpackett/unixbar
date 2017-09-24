use format::data::Format;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::{thread, fs, mem};
use std::time::Duration;
use std::os::unix::io::AsRawFd;

use libc::{ioctl, c_int, c_ulong};

use super::{VolumeBackend, VolumeState};

const IOR: u32 = 0x40000000;

const IOCPARM_SHIFT: u32 = 13;
const IOCPARM_MASK: u32 = ((1 << IOCPARM_SHIFT) - 1);

const TYPESHIFT: u32 = 8;
const SIZESHIFT: u32 = 16;

macro_rules! ioctl {
    ($dir:expr, $name:ident, $ioty:expr, $nr:expr, $size:expr; $ty:ty) => (
        pub unsafe fn $name(fd: c_int, val: *mut $ty) -> c_int {
            let ioc = ($dir as u32) |
                      (($size as u32 & IOCPARM_MASK) << SIZESHIFT) |
                      (($ioty as u32) << TYPESHIFT) |
                      ($nr as u32);
            ioctl(fd, ioc as c_ulong, val)
        }
    );
}

ioctl!(IOR, sound_mixer_read_volume, b'M', 0, mem::size_of::<c_int>(); c_int);
ioctl!(IOR, sound_mixer_read_mute, b'M', 28, mem::size_of::<c_int>(); c_int);


pub struct FreeBSDSound {
    last_value: Arc<RwLock<Format>>,
    mixer: Arc<fs::File>,
}

impl FreeBSDSound {
    pub fn new() -> FreeBSDSound {
        FreeBSDSound {
            last_value: Arc::new(RwLock::new(Format::Str("".to_owned()))),
            mixer: Arc::new(fs::File::open("/dev/mixer").unwrap()),
        }
    }

    fn get_volume_state(fd: c_int) -> VolumeState {
        let mut volume : c_int = 0;
        unsafe { sound_mixer_read_volume(fd, &mut volume); }
        // right channel: (volume >> 8) & 0x7f
        let mut muted : c_int = 0;
        unsafe { sound_mixer_read_mute(fd, &mut muted); }
        VolumeState {
            volume: ((volume & 0x7f) as f32) / 100.0,
            muted: muted == 1,
        }
    }
}

impl<F> VolumeBackend<F> for FreeBSDSound
where F: Fn(VolumeState) -> Format + Sync + Send + 'static {

    fn current_value(&self) -> Format {
        (*self.last_value).read().unwrap().clone()
    }

    fn spawn_notifier(&mut self, tx: Sender<()>, updater: Arc<Box<F>>) {
        let last_value = self.last_value.clone();
        let mixer = self.mixer.clone();
        thread::spawn(move || {
            loop {
                {
                    let mut writer = last_value.write().unwrap();
                    *writer = (*updater)(FreeBSDSound::get_volume_state((*mixer).as_raw_fd()));
                }
                let _ = tx.send(());
                thread::sleep(Duration::from_secs(4));
            }
        });
    }
}

pub fn default_volume() -> FreeBSDSound {
    FreeBSDSound::new()
}
