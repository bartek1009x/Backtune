extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioSpecWAV};

use crate::dir;

use rand::prelude::*;
use std::ops::DerefMut;
use std::path;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
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

static CURRENTLY_PLAYING: Mutex<bool> = Mutex::new(false);
static CURRENTLY_PLAYING_INDEX: Mutex<usize> = Mutex::new(0);
static STARTED_AT: Mutex<f64> = Mutex::new(0.0);
static CHOSEN_WAIT: Mutex<f64> = Mutex::new(0.0);

pub fn init(loaded_audio_paths: &mut Vec<path::PathBuf>) {
    dir::load_audio_paths(loaded_audio_paths);
}

pub enum AudioDeviceType {
    U8(AudioDevice<U8AudioCallback>),
    I16(AudioDevice<I16AudioCallback>),
    I32(AudioDevice<I32AudioCallback>),
}

pub fn update(
    audio_system: &sdl2::AudioSubsystem,
    loaded_audio_paths: &Vec<path::PathBuf>,
    audio_device: &mut Option<AudioDeviceType>,
    settings: &crate::settings::Settings,
    loaded_audio: &mut Option<CopiedData>,
) {
    let mut currently_playing = CURRENTLY_PLAYING.lock().unwrap();
    let mut chosen_wait = CHOSEN_WAIT.lock().unwrap();
    let mut started_at = STARTED_AT.lock().unwrap();
    let mut current_index = CURRENTLY_PLAYING_INDEX.lock().unwrap();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs_f64();

    if !*currently_playing {
        if *chosen_wait == 0.0 {
            let min = settings.min_wait_time;
            let max = settings.max_wait_time;
            let rand_time = rand::rng().random_range(min..max);

            *chosen_wait = now + rand_time;
            return;
        }

        if now >= *chosen_wait {
            play_random_audio(
                audio_system,
                loaded_audio_paths,
                audio_device,
                &mut currently_playing,
                &mut current_index,
                &mut started_at,
                loaded_audio,
            );

            *started_at = now;
            *chosen_wait = 0.0;
        }
    } else {
        match loaded_audio {
            Some(existing_audio) => {
                let finish_time = *started_at + existing_audio.length;

                if now >= finish_time {
                    stop_audio(audio_device, settings, &now);
                }
            }
            None => {
                println!("Error playing the audio: not loaded properly");
            }
        }
    }
}

pub fn stop_audio(
    audio_device: &mut Option<AudioDeviceType>,
    settings: &crate::settings::Settings,
    now: &f64,
) {
    if let Some(dev) = audio_device {
        match dev {
            AudioDeviceType::U8(d) => d.pause(),
            AudioDeviceType::I16(d) => d.pause(),
            AudioDeviceType::I32(d) => d.pause(),
        }
    }

    *audio_device = None;

    *CURRENTLY_PLAYING.lock().unwrap() = false;

    let min = settings.min_wait_time;
    let max = settings.max_wait_time;
    let rand_time = rand::rng().random_range(min..max);

    *CHOSEN_WAIT.lock().unwrap() = now + rand_time;
}

fn play_random_audio(
    audio_system: &sdl2::AudioSubsystem,
    loaded_audio_paths: &Vec<path::PathBuf>,
    audio_device: &mut Option<AudioDeviceType>,
    currently_playing: &mut bool,
    currently_playing_index: &mut usize,
    started_at: &mut f64,
    loaded_audio: &mut Option<CopiedData>,
) {
    let mut rng = rand::rng();
    let random_i = rng.random_range(..loaded_audio_paths.len());

    if let Some(audio_path) = loaded_audio_paths.get(random_i) {
        setup_audio(audio_path, loaded_audio);

        match loaded_audio {
            Some(existing_audio) => {
                let audio_spec = AudioSpecDesired {
                    freq: Some(existing_audio.freq),
                    channels: Some(existing_audio.channels),
                    samples: None,
                };

                match existing_audio.format {
                    AudioFormat::U8 => {
                        let callback = U8AudioCallback {
                            bytes: existing_audio.bytes.clone(),
                            position: 0,
                        };

                        let device = audio_system
                            .open_playback(None, &audio_spec, move |_spec| callback)
                            .unwrap();

                        device.resume();
                        *audio_device = Some(AudioDeviceType::U8(device));
                    }
                    AudioFormat::I16 => {
                        let samples: Vec<i16> = existing_audio
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
                        let samples: Vec<i32> = existing_audio
                            .bytes
                            .chunks_exact(4)
                            .map(|chunk| {
                                i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                            })
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

                *started_at = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time error")
                    .as_secs_f64();

                *currently_playing = true;
                *currently_playing_index = random_i;
            }
            None => {
                println!("Error playing the audio: not loaded properly")
            }
        }
    }
}

fn setup_audio(entry_path: &PathBuf, loaded_audio: &mut Option<CopiedData>) {
    if let Ok(audio_wav) = AudioSpecWAV::load_wav(&entry_path) {
        let format = match audio_wav.format {
            sdl2::audio::AudioFormat::U8 => crate::player::AudioFormat::U8,
            sdl2::audio::AudioFormat::S16LSB | sdl2::audio::AudioFormat::S16MSB => {
                crate::player::AudioFormat::I16
            }
            sdl2::audio::AudioFormat::S32LSB | sdl2::audio::AudioFormat::S32MSB => {
                crate::player::AudioFormat::I32
            }
            _ => {
                println!(
                    "Unsupported audio format {:?} for {:?}, skipping",
                    audio_wav.format, entry_path
                );
                return;
            }
        };

        let total_bytes = audio_wav.buffer().len();
        let bytes_per_sample = bytes_per_sample(audio_wav.format);

        let samples_per_second = audio_wav.freq as usize * audio_wav.channels as usize;
        let bytes_per_second = samples_per_second * bytes_per_sample;

        let seconds = total_bytes as f64 / bytes_per_second as f64;

        let copied_data = crate::player::CopiedData {
            bytes: audio_wav.buffer().to_vec(),
            position: 0,
            freq: audio_wav.freq,
            channels: audio_wav.channels,
            format,
            length: seconds,
        };

        *loaded_audio = Some(copied_data);
    }
}

#[inline]
fn bytes_per_sample(format: sdl2::audio::AudioFormat) -> usize {
    match format {
        sdl2::audio::AudioFormat::U8 => 1,
        sdl2::audio::AudioFormat::S16LSB | sdl2::audio::AudioFormat::S16MSB => 2,
        sdl2::audio::AudioFormat::S32LSB | sdl2::audio::AudioFormat::S32MSB => 4,
        _ => panic!("Unsupported format"),
    }
}
