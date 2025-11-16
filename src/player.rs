extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

use crate::dir;

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;

#[derive(Clone)]
pub struct CopiedData {
    pub bytes: Vec<u8>,
    pub position: usize,
    pub freq: i32,
    pub channels: u8,
    pub format: AudioFormat,
    pub length: f64,
}

#[derive(Clone, Copy)]
pub enum AudioFormat {
    U8,
    I16,
    I32,
}

// For u8 audio
pub struct U8AudioCallback {
    pub bytes: Vec<u8>,
    pub position: usize,
}

impl AudioCallback for U8AudioCallback {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let remaining = self.bytes.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        if to_copy > 0 {
            data[..to_copy].copy_from_slice(&self.bytes[self.position..self.position + to_copy]);
            self.position += to_copy;
        }

        if to_copy < data.len() {
            data[to_copy..].fill(128);
        }
    }
}

// For i16 audio
pub struct I16AudioCallback {
    pub samples: Vec<i16>,
    pub position: usize,
}

impl AudioCallback for I16AudioCallback {
    type Channel = i16;

    fn callback(&mut self, data: &mut [i16]) {
        let remaining = self.samples.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        if to_copy > 0 {
            data[..to_copy].copy_from_slice(&self.samples[self.position..self.position + to_copy]);
            self.position += to_copy;
        }

        if to_copy < data.len() {
            data[to_copy..].fill(0);
        }
    }
}

// For i32 audio
pub struct I32AudioCallback {
    pub samples: Vec<i32>,
    pub position: usize,
}

impl AudioCallback for I32AudioCallback {
    type Channel = i32;

    fn callback(&mut self, data: &mut [i32]) {
        let remaining = self.samples.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        if to_copy > 0 {
            data[..to_copy].copy_from_slice(&self.samples[self.position..self.position + to_copy]);
            self.position += to_copy;
        }

        if to_copy < data.len() {
            data[to_copy..].fill(0);
        }
    }
}

pub fn init(loaded_audios: &mut Vec<CopiedData>) {
    dir::load_audio(loaded_audios);
}

static CURRENTLY_PLAYING: Mutex<bool> = Mutex::new(false);
static STARTED_AT: Mutex<f64> = Mutex::new(0.0);

pub enum AudioDeviceType {
    U8(AudioDevice<U8AudioCallback>),
    I16(AudioDevice<I16AudioCallback>),
    I32(AudioDevice<I32AudioCallback>),
}

pub fn update(
    audio_system: &sdl2::AudioSubsystem,
    loaded_audios: &Vec<CopiedData>,
    audio_device: &mut Option<AudioDeviceType>,
) {
    let mut currently_playing = CURRENTLY_PLAYING.lock().unwrap();

    if !*currently_playing {
        play_audio(audio_system, loaded_audios, audio_device, &mut currently_playing);
    } else if let Some(first_audio) = loaded_audios.get(0) {
        if (*STARTED_AT.lock().unwrap() + first_audio.length) - SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_secs_f64() <= 0.0 {
            *currently_playing = false;
        }
    }
}

fn play_audio(
    audio_system: &sdl2::AudioSubsystem,
    loaded_audios: &Vec<CopiedData>,
    audio_device: &mut Option<AudioDeviceType>,
    currently_playing: &mut bool,
) {
    if let Some(first_audio) = loaded_audios.get(0) {
        let audio_spec = AudioSpecDesired {
            freq: Some(first_audio.freq),
            channels: Some(first_audio.channels),
            samples: None,
        };

        match first_audio.format {
            AudioFormat::U8 => {
                let callback = U8AudioCallback {
                    bytes: first_audio.bytes.clone(),
                    position: 0,
                };

                let device = audio_system
                    .open_playback(None, &audio_spec, move |_spec| callback)
                    .unwrap();

                device.resume();
                *audio_device = Some(AudioDeviceType::U8(device));
            }
            AudioFormat::I16 => {
                let samples: Vec<i16> = first_audio
                    .bytes
                    .chunks_exact(2)
                    .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();

                let callback = I16AudioCallback {
                    samples,
                    position: 0,
                };

                let device = audio_system
                    .open_playback(None, &audio_spec, move |_spec| callback)
                    .unwrap();

                device.resume();
                *audio_device = Some(AudioDeviceType::I16(device));
            }
            AudioFormat::I32 => {
                let samples: Vec<i32> = first_audio
                    .bytes
                    .chunks_exact(4)
                    .map(|chunk| i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                    .collect();

                let callback = I32AudioCallback {
                    samples,
                    position: 0,
                };

                let device = audio_system
                    .open_playback(None, &audio_spec, move |_spec| callback)
                    .unwrap();

                device.resume();
                *audio_device = Some(AudioDeviceType::I32(device));
            }
        }

        let mut started_at = STARTED_AT.lock().unwrap();
        *started_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_secs_f64();

        *currently_playing = true;
    }
}
