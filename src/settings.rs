use std::fs;
use std::path;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::dir;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub min_wait_time: f64,
    pub max_wait_time: f64,
}

static SETTINGS: Mutex<Option<Settings>> = Mutex::new(None);

pub fn init_settings(settings_file: &path::PathBuf) {
    let settings = Settings {
        min_wait_time: 60.0,
        max_wait_time: 240.0,
    };

    fs::write(&settings_file, serde_json::to_string(&settings).unwrap())
        .expect("Could not create settings.json");
}

pub fn load_settings() {
    let settings_file = dir::get_settings();
    let settings_str = fs::read_to_string(&settings_file).expect("Could not read settings.json");
    *SETTINGS.lock().unwrap() =
        serde_json::from_str(&settings_str).expect("Could not parse settings.json");
}

pub fn get_cloned_settings() -> Settings {
    SETTINGS.lock().unwrap().as_ref().unwrap().clone()
}
