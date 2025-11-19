use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::{Window, WindowContext};

use std::collections::HashMap;
use std::path;

const TEXTURE_NAMES: [&str; 6] = [
    "button_inactive",
    "button_hover",
    "button_mouse_down",
    "play",
    "stop",
    "folder",
];

pub fn init<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a sdl2::render::TextureCreator<WindowContext>,
    textures: &mut HashMap<String, Texture<'a>>,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    font: &mut Option<sdl2::ttf::Font<'a, 'a>>,
) {
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    clear(canvas);

    let attempted_font = ttf_context.load_font(
        path::Path::new("assets/Roboto/static/Roboto-Black.ttf"),
        128,
    );
    match attempted_font {
        Ok(mut existing_font) => {
            existing_font.set_style(sdl2::ttf::FontStyle::NORMAL);

            let surface = existing_font
                .render("Backtune")
                .blended(Color::RGBA(255, 255, 255, 255))
                .map_err(|e| e.to_string());
            let texture = texture_creator
                .create_texture_from_surface(&surface.unwrap())
                .map_err(|e| e.to_string());

            let _ = canvas.copy(
                &mut texture.as_ref().unwrap(),
                None,
                Rect::new(960 / 2 - 175, 0, 350, 100),
            );

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

    present_buttons(canvas, textures, 0, 0);
}

pub fn update(
    canvas: &mut Canvas<Window>,
    texture_creator: &sdl2::render::TextureCreator<WindowContext>,
    textures: &mut HashMap<String, Texture>,
    font: &mut Option<sdl2::ttf::Font>,
    mouse_x: i32,
    mouse_y: i32,
) {
    clear(canvas);

    font.as_mut()
        .unwrap()
        .set_style(sdl2::ttf::FontStyle::NORMAL);
    let surface = font
        .as_ref()
        .unwrap()
        .render("Backtune")
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string());
    let texture = texture_creator
        .create_texture_from_surface(&surface.unwrap())
        .map_err(|e| e.to_string());

    let _ = canvas.copy(
        &mut texture.as_ref().unwrap(),
        None,
        Rect::new(960 / 2 - 175, 0, 350, 100),
    );

    present_buttons(canvas, textures, mouse_x, mouse_y);
}

#[inline]
fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(32, 32, 32));
    canvas.clear();
}

fn present_buttons(
    canvas: &mut Canvas<Window>,
    textures: &mut HashMap<String, Texture>,
    mouse_x: i32,
    mouse_y: i32,
) {
    let button1_rect = Rect::new(960 / 2 - 165, 100, 100, 100);
    let button2_rect = Rect::new(960 / 2 - 55, 100, 100, 100);
    let button3_rect = Rect::new(960 / 2 + 55, 100, 100, 100);

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
        let _ = canvas.copy(&textures.get("button_hover").unwrap(), None, button1_rect);
    } else {
        let _ = canvas.copy(
            &textures.get("button_inactive").unwrap(),
            None,
            button1_rect,
        );
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
        let _ = canvas.copy(&textures.get("button_hover").unwrap(), None, button2_rect);
    } else {
        let _ = canvas.copy(
            &textures.get("button_inactive").unwrap(),
            None,
            button2_rect,
        );
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
        let _ = canvas.copy(&textures.get("button_hover").unwrap(), None, button3_rect);
    } else {
        let _ = canvas.copy(
            &textures.get("button_inactive").unwrap(),
            None,
            button3_rect,
        );
    }
    let _ = canvas.copy(&textures.get("folder").unwrap(), None, button3_rect);

    canvas.present();
}

#[inline]
fn is_mouse_over_button(x: i32, y: i32, x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    return x > x1 && x < x2 && y > y1 && y < y2;
}
