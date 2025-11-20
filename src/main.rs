extern crate sdl2;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::render::Texture;
use sdl2::ttf::Font;

use std::collections::HashMap;
use std::path;
use std::time::Duration;

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
    let mut loaded_audio_paths: Vec<path::PathBuf> = Vec::new();
    let mut loaded_audio: Option<player::CopiedData> = None;

    settings::load_settings();
    let ref_settings = settings::get_cloned_settings();
    player::init(&mut loaded_audio_paths);

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut textures: HashMap<String, Texture> = HashMap::new();
    let mut font: Option<Font> = None;
    renderer::init(
        &mut canvas,
        &texture_creator,
        &mut textures,
        &ttf_context,
        &mut font,
        &mut play,
    );

    let mut last_button_states = [false; 3];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        let mouse_state = event_pump.mouse_state();

        renderer::update(
            &mut canvas,
            &texture_creator,
            &mut textures,
            &mut font,
            &mut last_button_states,
            &mut play,
            &mut audio_device,
            mouse_state.x(),
            mouse_state.y(),
            mouse_state.is_mouse_button_pressed(MouseButton::Left),
        );

        if play {
            player::update(
                &audio_system,
                &loaded_audio_paths,
                &mut audio_device,
                &ref_settings,
                &mut loaded_audio,
            );
        }

        ::std::thread::sleep(THREAD_SLEEP_TIME);
    }
}
