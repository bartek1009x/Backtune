use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::{Window, WindowContext};

use std::collections::HashMap;
use std::path;

const TEXTURE_NAMES: [&str; 10] = [
    "button_inactive",
    "button_hover",
    "button_mouse_down",
    "play",
    "stop",
    "folder",
    "reload",
    "value_active",
    "value_hover",
    "value_inactive",
];

pub fn init<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a sdl2::render::TextureCreator<WindowContext>,
    textures: &mut HashMap<String, Texture<'a>>,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    font: &mut Option<sdl2::ttf::Font<'a, 'a>>,
) {
    let _image_context = sdl2::image::init(InitFlag::PNG);

    clear(canvas);

    let attempted_font = ttf_context.load_font(
        path::Path::new("assets/Roboto/static/Roboto-Black.ttf"),
        128,
    );
    match attempted_font {
        Ok(mut existing_font) => {
            existing_font.set_style(sdl2::ttf::FontStyle::NORMAL);
            *font = Some(existing_font);
        }
        Err(err) => {
            println!("Error loading font: {}", err);
        }
    }

    for texture_name in TEXTURE_NAMES {
        match texture_creator
            .load_texture(path::Path::new(&format!("assets/{}.png", &texture_name)))
        {
            Ok(texture) => {
                textures.insert(texture_name.to_string(), texture);
            }
            Err(err) => {
                println!("Error loading texture {}: {}", texture_name, err);
                return;
            }
        }
    }
}

pub fn update(
    canvas: &mut Canvas<Window>,
    texture_creator: &sdl2::render::TextureCreator<WindowContext>,
    textures: &mut HashMap<String, Texture>,
    font: &mut Option<sdl2::ttf::Font>,
    last_button_states: &mut [bool; 6],
    play: &mut bool,
    audio_device: &mut Option<crate::player::AudioDeviceType>,
    loaded_audio_paths: &mut Vec<std::path::PathBuf>,
    mouse_x: i32,
    mouse_y: i32,
    mouse_down: bool,
    capture_text: &mut i32,
    captured_text: &mut String,
) {
    clear(canvas);

    match font {
        Some(existing_font) => {
            draw_text(
                canvas,
                texture_creator,
                existing_font,
                "Backtune",
                Rect::new(960 / 2 - 175, 0, 350, 100),
            );

            present_settings(
                canvas,
                texture_creator,
                textures,
                existing_font,
                last_button_states,
                mouse_x,
                mouse_y,
                mouse_down,
                capture_text,
                captured_text,
            );
        }
        _ => {}
    }

    present_buttons(
        canvas,
        textures,
        last_button_states,
        play,
        audio_device,
        loaded_audio_paths,
        mouse_x,
        mouse_y,
        mouse_down,
    );
}

#[inline]
fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(32, 32, 32));
    canvas.clear();
}

fn present_settings(
    canvas: &mut Canvas<Window>,
    texture_creator: &sdl2::render::TextureCreator<WindowContext>,
    textures: &mut HashMap<String, Texture>,
    font: &mut sdl2::ttf::Font,
    last_button_states: &mut [bool; 6],
    mouse_x: i32,
    mouse_y: i32,
    mouse_pressed: bool,
    capture_text: &mut i32,
    captured_text: &mut String,
) {
    draw_text(
        canvas,
        texture_creator,
        font,
        "Minimum wait time",
        Rect::new(200, 250, 350, 50),
    );

    draw_text(
        canvas,
        texture_creator,
        font,
        "Maximum wait time",
        Rect::new(200, 315, 350, 50),
    );

    let button1_rect = Rect::new(660, 250, 100, 50);
    let button2_rect = Rect::new(660, 315, 100, 50);

    // BUTTON 1
    let hovered1 = is_mouse_over_button(
        mouse_x,
        mouse_y,
        button1_rect.x(),
        button1_rect.x() + button1_rect.width() as i32,
        button1_rect.y(),
        button1_rect.y() + button1_rect.height() as i32,
    );

    if hovered1 {
        if mouse_pressed {
            let _ = canvas.copy(&textures.get("value_active").unwrap(), None, button1_rect);

            last_button_states[4] = true;
        } else {
            if *capture_text == 0 {
                let _ = canvas.copy(&textures.get("value_active").unwrap(), None, button1_rect);
            } else {
                let _ = canvas.copy(&textures.get("value_hover").unwrap(), None, button1_rect);
            }
            if last_button_states[4] {
                *capture_text = 0;
                captured_text.clear();
            }
            last_button_states[4] = false;
        }
    } else {
        if *capture_text == 0 {
            let _ = canvas.copy(&textures.get("value_active").unwrap(), None, button1_rect);
        } else {
            let _ = canvas.copy(&textures.get("value_inactive").unwrap(), None, button1_rect);
        }
        last_button_states[4] = false;
    }

    if mouse_pressed && !hovered1 && *capture_text != -1 {
        crate::settings::set_setting(*capture_text, &captured_text);
        *capture_text = -1;
    }

    let mut wait_time = &crate::settings::get_cloned_settings()
        .min_wait_time
        .to_string();
    if *capture_text == 0 {
        wait_time = captured_text;
    }

    let mut scale = match wait_time.len() {
        1 => 0.25,
        2 => 0.50,
        3 => 0.75,
        _ => 1.0,
    };

    let mut full_w = button1_rect.width() as f32;
    let mut scaled_w = full_w * scale;

    let mut x = button1_rect.x + ((full_w - scaled_w) / 2.0) as i32;

    draw_text(
        canvas,
        texture_creator,
        font,
        &wait_time,
        Rect::new(x, button1_rect.y, scaled_w as u32, button1_rect.height()),
    );

    // BUTTON 2
    let hovered2 = is_mouse_over_button(
        mouse_x,
        mouse_y,
        button2_rect.x(),
        button2_rect.x() + button2_rect.width() as i32,
        button2_rect.y(),
        button2_rect.y() + button2_rect.height() as i32,
    );

    if hovered2 {
        if mouse_pressed {
            let _ = canvas.copy(&textures.get("value_active").unwrap(), None, button2_rect);

            last_button_states[5] = true;
        } else {
            if *capture_text == 1 {
                let _ = canvas.copy(&textures.get("value_active").unwrap(), None, button2_rect);
            } else {
                let _ = canvas.copy(&textures.get("value_hover").unwrap(), None, button2_rect);
            }
            if last_button_states[5] {
                *capture_text = 1;
                captured_text.clear();
            }
            last_button_states[5] = false;
        }
    } else {
        if *capture_text == 1 {
            let _ = canvas.copy(&textures.get("value_active").unwrap(), None, button2_rect);
        } else {
            let _ = canvas.copy(&textures.get("value_inactive").unwrap(), None, button2_rect);
        }
        last_button_states[5] = false;
    }

    if mouse_pressed && !hovered2 && *capture_text != -1 {
        crate::settings::set_setting(*capture_text, &captured_text);
        *capture_text = -1;
    }

    let mut wait_time = &crate::settings::get_cloned_settings()
        .max_wait_time
        .to_string();
    if *capture_text == 1 {
        wait_time = captured_text;
    }

    scale = match wait_time.len() {
        1 => 0.25,
        2 => 0.50,
        3 => 0.75,
        _ => 1.0,
    };

    full_w = button2_rect.width() as f32;
    scaled_w = full_w * scale;

    x = button2_rect.x + ((full_w - scaled_w) / 2.0) as i32;

    draw_text(
        canvas,
        texture_creator,
        font,
        &wait_time,
        Rect::new(x, button2_rect.y, scaled_w as u32, button2_rect.height()),
    );
}

fn present_buttons(
    canvas: &mut Canvas<Window>,
    textures: &mut HashMap<String, Texture>,
    last_button_states: &mut [bool; 6],
    play: &mut bool,
    audio_device: &mut Option<crate::player::AudioDeviceType>,
    loaded_audio_paths: &mut Vec<std::path::PathBuf>,
    mouse_x: i32,
    mouse_y: i32,
    mouse_pressed: bool,
) {
    let button1_rect = Rect::new(960 / 2 - 215, 100, 100, 100);
    let button2_rect = Rect::new(960 / 2 - 105, 100, 100, 100);
    let button3_rect = Rect::new(960 / 2 + 5, 100, 100, 100);
    let button4_rect = Rect::new(960 / 2 + 115, 100, 100, 100);

    // BUTTON 1
    let hovered1 = is_mouse_over_button(
        mouse_x,
        mouse_y,
        button1_rect.x(),
        button1_rect.x() + button1_rect.width() as i32,
        button1_rect.y(),
        button1_rect.y() + button1_rect.height() as i32,
    );

    if hovered1 {
        if mouse_pressed {
            let _ = canvas.copy(
                &textures.get("button_mouse_down").unwrap(),
                None,
                button1_rect,
            );

            last_button_states[0] = true;
        } else {
            let _ = canvas.copy(&textures.get("button_hover").unwrap(), None, button1_rect);
            if last_button_states[0] {
                let _ = crate::controller::play_button(audio_device, play);
            }
            last_button_states[0] = false;
        }
    } else {
        let _ = canvas.copy(
            &textures.get("button_inactive").unwrap(),
            None,
            button1_rect,
        );
        last_button_states[0] = false;
    }
    let _ = canvas.copy(&textures.get("play").unwrap(), None, button1_rect);

    // BUTTON 2
    let hovered2 = is_mouse_over_button(
        mouse_x,
        mouse_y,
        button2_rect.x(),
        button2_rect.x() + button2_rect.width() as i32,
        button2_rect.y(),
        button2_rect.y() + button2_rect.height() as i32,
    );

    if hovered2 {
        if mouse_pressed {
            let _ = canvas.copy(
                &textures.get("button_mouse_down").unwrap(),
                None,
                button2_rect,
            );

            last_button_states[1] = true;
        } else {
            let _ = canvas.copy(&textures.get("button_hover").unwrap(), None, button2_rect);
            if last_button_states[1] {
                let _ = crate::controller::stop_button(audio_device, play);
            }
            last_button_states[1] = false;
        }
    } else {
        let _ = canvas.copy(
            &textures.get("button_inactive").unwrap(),
            None,
            button2_rect,
        );
        last_button_states[1] = false;
    }
    let _ = canvas.copy(&textures.get("stop").unwrap(), None, button2_rect);

    // BUTTON 3
    let hovered3 = is_mouse_over_button(
        mouse_x,
        mouse_y,
        button3_rect.x(),
        button3_rect.x() + button3_rect.width() as i32,
        button3_rect.y(),
        button3_rect.y() + button3_rect.height() as i32,
    );

    if hovered3 {
        if mouse_pressed {
            let _ = canvas.copy(
                &textures.get("button_mouse_down").unwrap(),
                None,
                button3_rect,
            );

            last_button_states[2] = true;
        } else {
            let _ = canvas.copy(&textures.get("button_hover").unwrap(), None, button3_rect);
            if last_button_states[2] {
                let _ = crate::controller::folder_button();
            }
            last_button_states[2] = false;
        }
    } else {
        let _ = canvas.copy(
            &textures.get("button_inactive").unwrap(),
            None,
            button3_rect,
        );

        last_button_states[2] = false;
    }
    let _ = canvas.copy(&textures.get("folder").unwrap(), None, button3_rect);

    // BUTTON 4
    let hovered4 = is_mouse_over_button(
        mouse_x,
        mouse_y,
        button4_rect.x(),
        button4_rect.x() + button4_rect.width() as i32,
        button4_rect.y(),
        button4_rect.y() + button4_rect.height() as i32,
    );

    if hovered4 {
        if mouse_pressed {
            let _ = canvas.copy(
                &textures.get("button_mouse_down").unwrap(),
                None,
                button4_rect,
            );

            last_button_states[3] = true;
        } else {
            let _ = canvas.copy(&textures.get("button_hover").unwrap(), None, button4_rect);
            if last_button_states[3] {
                let _ = crate::controller::reload_button(loaded_audio_paths);
            }
            last_button_states[3] = false;
        }
    } else {
        let _ = canvas.copy(
            &textures.get("button_inactive").unwrap(),
            None,
            button4_rect,
        );
        last_button_states[3] = false;
    }
    let _ = canvas.copy(&textures.get("reload").unwrap(), None, button4_rect);

    canvas.present();
}

#[inline]
fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &sdl2::render::TextureCreator<WindowContext>,
    font: &mut sdl2::ttf::Font,
    text: &str,
    rect: Rect,
) {
    if text.is_empty() {
        return;
    }

    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string());
    let texture = texture_creator
        .create_texture_from_surface(&surface.unwrap())
        .map_err(|e| e.to_string());

    let _ = canvas.copy(&mut texture.as_ref().unwrap(), None, rect);
}

#[inline]
fn is_mouse_over_button(x: i32, y: i32, x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    return x > x1 && x < x2 && y > y1 && y < y2;
}
