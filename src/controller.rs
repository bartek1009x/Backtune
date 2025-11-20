use std::io::Error;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn play_button(play: &mut bool) -> Result<(), Error> {
    *play = true;
    let mut x = None;
    crate::player::stop_audio(
        &mut x,
        &crate::settings::get_cloned_settings(),
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_secs_f64(),
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
            crate::player::stop_audio(audio_device, &crate::settings::get_cloned_settings(), &now);
        }
        None => {}
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
