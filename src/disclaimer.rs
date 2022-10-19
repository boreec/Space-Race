use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::path::Path;
use std::time::Duration;

use crate::WINDOW_WIDTH;

const TITLE_WIDTH: u32 = 200;
const TITLE_HEIGHT: u32 = 100;
const TITLE_STR: &str = "DISCLAIMER";

const SCREEN_DURATION: u64 = 5;

pub fn show_disclaimer(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>
){
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed!");
    let texture_creator = canvas.texture_creator();
    let poetsen_font_path: &Path = Path::new("font/poetsen_one/PoetsenOne-Regular.ttf");

    let font = ttf_context
        .load_font(poetsen_font_path, 128)
        .expect(&format!("Failed to load font {}", poetsen_font_path.display()));

    let surface_title = font
        .render(TITLE_STR)
        .blended(Color::WHITE)
        .expect("Failed to create font surface for Disclaimer's message's title!");

    let font_rect_title = Rect::new(
        (WINDOW_WIDTH / 2 - TITLE_WIDTH / 2) as i32,
        0,
        TITLE_WIDTH,
        TITLE_HEIGHT
    );

    let texture_title = texture_creator
        .create_texture_from_surface(&surface_title)
        .expect("Failed to create texte for Disclaimer's message's title!");

    canvas.copy(&texture_title, None, font_rect_title).expect("Failed to copy Disclaimer's title texture to canvas!");
    canvas.present();
    ::std::thread::sleep(Duration::new(SCREEN_DURATION, 0));
}
