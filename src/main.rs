extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;

use std::time::Duration;

mod dir;
mod player;
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

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(32, 32, 32));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let audio_system = sdl_context.audio().unwrap();
    let mut audio_device: Option<player::AudioDeviceType> = None;
    let mut loaded_audios: Vec<player::CopiedData> = Vec::new();

    let mut settings: Option<settings::Settings> = None;
    settings::load_settings(&mut settings);
    player::init(&mut loaded_audios);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        player::update(&audio_system, &loaded_audios, &mut audio_device, settings.as_ref());

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
