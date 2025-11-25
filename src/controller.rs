use sdl2::keyboard::Keycode;

use std::io::Error;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn play_button(
    audio_device: &mut Option<crate::player::AudioDeviceType>,
    play: &mut bool,
) -> Result<(), Error> {
    *play = true;
    crate::player::stop_audio(
        audio_device,
        &crate::settings::get_cloned_settings(),
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_secs_f64(),
        None,
        None,
    );
    Ok(())
}

pub fn stop_button(
    audio_device: &mut Option<crate::player::AudioDeviceType>,
    play: &mut bool,
) -> Result<(), Error> {
    *play = false;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs_f64();

    match audio_device {
        Some(_) => {
            crate::player::stop_audio(
                audio_device,
                &crate::settings::get_cloned_settings(),
                &now,
                None,
                None,
            );
        }
        _ => {}
    }

    Ok(())
}

pub fn folder_button() -> Result<(), Error> {
    let path = crate::dir::get_sounds_dir();
    if cfg!(target_os = "windows") {
        Command::new("explorer").arg(path).spawn()?;
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn()?;
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn()?;
    } else {
        eprintln!("Unsupported OS");
    }

    Ok(())
}

pub fn reload_button(loaded_audio_paths: &mut Vec<std::path::PathBuf>) -> Result<(), Error> {
    *loaded_audio_paths = crate::dir::load_audio_paths();
    Ok(())
}

pub fn get_input_char(keycode: Option<Keycode>, captured_text: &String) -> Option<char> {
    return match keycode {
        Some(Keycode::NUM_0) => Some('0'),
        Some(Keycode::NUM_1) => Some('1'),
        Some(Keycode::NUM_2) => Some('2'),
        Some(Keycode::NUM_3) => Some('3'),
        Some(Keycode::NUM_4) => Some('4'),
        Some(Keycode::NUM_5) => Some('5'),
        Some(Keycode::NUM_6) => Some('6'),
        Some(Keycode::NUM_7) => Some('7'),
        Some(Keycode::NUM_8) => Some('8'),
        Some(Keycode::NUM_9) => Some('9'),
        Some(Keycode::PERIOD) if captured_text.find('.') == None => Some('.'),
        _ => None,
    };
}
