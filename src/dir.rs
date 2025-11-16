use directories::BaseDirs;
use std::fs;
use std::path;
use std::sync::OnceLock;

use sdl2::audio::AudioSpecWAV;

const APP_NAME: &str = "Backtune";
static BASE_DIRS: OnceLock<BaseDirs> = OnceLock::new();

pub fn init_dir() -> path::PathBuf {
    let base_dirs =
        BASE_DIRS.get_or_init(|| BaseDirs::new().expect("Could not determine OS directories"));

    let app_dir = base_dirs.data_dir().join(APP_NAME);

    fs::create_dir_all(&app_dir).expect("Could not create app directory");

    let sounds_dir = app_dir.join("sounds");
    fs::create_dir_all(&sounds_dir).expect("Could not create sounds directory");

    let settings_file = app_dir.join("settings.json");
    if !settings_file.exists() {
        crate::settings::init_settings(&settings_file);
    }

    return app_dir;
}

pub fn get_sounds_dir() -> path::PathBuf {
    let base_dirs = BASE_DIRS.get().expect("BASE_DIRS not initialized");
    return base_dirs.data_dir().join(APP_NAME).join("sounds");
}

pub fn get_settings() -> path::PathBuf {
    let base_dirs = BASE_DIRS.get().expect("BASE_DIRS not initialized");
    return base_dirs.data_dir().join(APP_NAME).join("settings.json");
}

pub fn load_audio(loaded_audios: &mut Vec<crate::player::CopiedData>) {
    for entry in fs::read_dir(get_sounds_dir()).expect("Could not read directory") {
        let entry = entry.expect("Could not read directory entry");
        let entry_path = entry.path();

        if let Ok(audio_wav) = AudioSpecWAV::load_wav(&entry_path) {
            // Detect audio format based on WAV spec
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
                    continue;
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

            loaded_audios.push(copied_data);
        }
    }
}

fn bytes_per_sample(format: sdl2::audio::AudioFormat) -> usize {
    match format {
        sdl2::audio::AudioFormat::U8 => 1,
        sdl2::audio::AudioFormat::S16LSB | sdl2::audio::AudioFormat::S16MSB => 2,
        sdl2::audio::AudioFormat::S32LSB | sdl2::audio::AudioFormat::S32MSB => 4,
        _ => panic!("Unsupported format"),
    }
}
