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

pub fn save_settings() {
    fs::write(
        crate::dir::get_settings(),
        serde_json::to_string(&SETTINGS.lock().unwrap().as_ref()).unwrap(),
    )
    .expect("Could not create settings.json");
}

pub fn set_setting(which: i32, value: &String) {
    if value.is_empty() {
        return;
    }
    match which {
        0 => SETTINGS.lock().unwrap().as_mut().unwrap().min_wait_time = value.parse().unwrap(),
        1 => SETTINGS.lock().unwrap().as_mut().unwrap().max_wait_time = value.parse().unwrap(),
        _ => println!("Invalid setting index: {}", which),
    }
}
