extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::Texture;

use std::collections::HashMap;
use std::path;
use std::time::Duration;

mod audio_callbacks;
mod controller;
mod dir;
mod player;
mod renderer;
mod settings;

const THREAD_SLEEP_TIME: Duration = Duration::new(0, 1_000_000_000u32 / 60);

fn main() {
    let mut play = true;

    let _ = dir::init_dir();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Backtune", 960, 540)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let audio_system = sdl_context.audio().unwrap();
    let mut audio_device: Option<player::AudioDeviceType> = None;
    let mut loaded_audio_paths: Vec<path::PathBuf>;
    let mut loaded_audio: Option<player::CopiedData> = None;

    settings::load_settings();
    loaded_audio_paths = dir::load_audio_paths();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut textures: HashMap<String, Texture> = HashMap::new();
    let mut font: Option<sdl2::ttf::Font> = None;
    renderer::init(
        &mut canvas,
        &texture_creator,
        &mut textures,
        &ttf_context,
        &mut font,
    );

    let mut last_button_states = [false; 7];
    let mut capture_text = -1;
    let mut captured_text = String::new();
    let mut window_focused = true;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    settings::save_settings();
                    break 'running;
                }
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => {
                    if win_event == WindowEvent::FocusGained {
                        window_focused = true;
                    } else if win_event == WindowEvent::FocusLost {
                        window_focused = false;
                    }
                }
                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => {
                    if capture_text != -1 {
                        if keycode == Some(Keycode::ESCAPE) || keycode == Some(Keycode::RETURN) {
                            settings::set_setting(capture_text, &captured_text);
                            capture_text = -1;
                        } else if keycode == Some(Keycode::BACKSPACE) {
                            if captured_text.len() > 0 {
                                captured_text.pop();
                            }
                        } else {
                            let char = controller::get_input_char(keycode, &captured_text);
                            match char {
                                Some(char) => {
                                    captured_text.push(char);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if window_focused {
            let mouse_state = event_pump.mouse_state();

            renderer::update(
                &mut canvas,
                &texture_creator,
                &mut textures,
                &mut font,
                &mut last_button_states,
                &mut play,
                &mut audio_device,
                &mut loaded_audio_paths,
                mouse_state.x(),
                mouse_state.y(),
                mouse_state.is_mouse_button_pressed(MouseButton::Left),
                &mut capture_text,
                &mut captured_text,
            );
        }

        if play {
            player::update(
                &audio_system,
                &loaded_audio_paths,
                &mut audio_device,
                &mut loaded_audio,
            );
        }

        ::std::thread::sleep(THREAD_SLEEP_TIME);
    }
}
