use directories::BaseDirs;
use std::fs;
use std::path;
use std::sync::OnceLock;

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

#[inline]
pub fn get_sounds_dir() -> path::PathBuf {
    let base_dirs = BASE_DIRS.get().expect("BASE_DIRS not initialized");
    return base_dirs.data_dir().join(APP_NAME).join("sounds");
}

#[inline]
pub fn get_settings() -> path::PathBuf {
    let base_dirs = BASE_DIRS.get().expect("BASE_DIRS not initialized");
    return base_dirs.data_dir().join(APP_NAME).join("settings.json");
}

pub fn load_audio_paths(loaded_audio_paths: &mut Vec<path::PathBuf>) {
    for entry in fs::read_dir(get_sounds_dir()).expect("Could not read directory") {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();

                loaded_audio_paths.push(entry_path);
            }
            Err(err) => {
                eprintln!("Error reading directory entry: {}", err);
            }
        }
    }
}
