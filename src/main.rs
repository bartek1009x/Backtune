extern crate sdl2;

use sdl2::event::Event;
use sdl2::render::Texture;
use sdl2::sys::{SDL_GetDisplayMode, SDL_GetWindowDisplayMode};
use sdl2::ttf::Font;

use std::collections::HashMap;
use std::path;
use std::time::Duration;

mod dir;
mod player;
mod renderer;
mod settings;

fn main() {
    dir::init_dir();

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

    let mut settings: Option<settings::Settings> = None;
    settings::load_settings(&mut settings);
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
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseMotion {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mousestate: _,
                    x,
                    y,
                    xrel: _,
                    yrel: _,
                } => {
                    renderer::update(
                        &mut canvas,
                        &texture_creator,
                        &mut textures,
                        &mut font,
                        x,
                        y,
                    );
                }
                _ => {}
            }
        }

        player::update(
            &audio_system,
            &loaded_audio_paths,
            &mut audio_device,
            settings.as_ref(),
            &mut loaded_audio,
        );

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
