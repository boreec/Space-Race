use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use std::path::Path;
use std::time::Duration;

use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

const TITLE_WIDTH: u32 = 200;
const TITLE_HEIGHT: u32 = 100;
const TITLE_STR: &str = "DISCLAIMER";

const MESSAGE_STR: &str = "
This game is an amateur reproduction of the Space Race game
originally developped by Atari, inc. in 1973. No media or
data has been stolen from the original game or from artists
 in order to make this reproduction.
\n
This game was done exclusively with the Rust language and the
bindings for the SDL2 library. The code is freely available
at gitlab.com/boreec/space-race. You can see more projects
of mine on boreec.fr.
\n
Have fun!";

const MESSAGE_MARGIN: u32 = 20;

const FADE_DURATION: u64 = 1;
const SCREEN_DURATION: u64 = 8;

pub fn show_disclaimer(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>
){
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed!");
    let texture_creator = canvas.texture_creator();
    let poetsen_font_path: &Path = Path::new("asset/font/poetsen_one/PoetsenOne-Regular.ttf");

    let big_font = ttf_context
        .load_font(poetsen_font_path, 128)
        .expect(&format!("Failed to load font {}", poetsen_font_path.display()));

    let small_font = ttf_context
        .load_font(poetsen_font_path, 24)
        .expect(&format!("Failed to load font {}", poetsen_font_path.display()));

    let surface_title = big_font
        .render(TITLE_STR)
        .blended(Color::WHITE)
        .expect("Failed to create font surface for Disclaimer's message's title!");

    let surface_message = small_font
        .render(MESSAGE_STR)
        .blended_wrapped(Color::WHITE, WINDOW_WIDTH - 2 * MESSAGE_MARGIN)
        .expect("Failed to create font surface for Disclaimer's message!");

    let rect_title = Rect::new(
        (WINDOW_WIDTH / 2 - TITLE_WIDTH / 2) as i32,
        0,
        TITLE_WIDTH,
        TITLE_HEIGHT
    );

    let rect_message = Rect::new(
        MESSAGE_MARGIN as i32,
        (TITLE_HEIGHT + MESSAGE_MARGIN) as i32,
        WINDOW_WIDTH - MESSAGE_MARGIN,
        WINDOW_HEIGHT - MESSAGE_MARGIN * 2 - TITLE_HEIGHT
    );

    let mut texture_title = texture_creator
        .create_texture_from_surface(&surface_title)
        .expect("Failed to create texte for Disclaimer's message's title!");

    let mut texture_message = texture_creator
        .create_texture_from_surface(&surface_message)
        .expect("Failed to create texte for Disclaimer's message!");

    // fade from black
    fade_message(&mut texture_title, &mut texture_message, rect_title, rect_message, canvas, FADE_DURATION, false);

    message_to_screen(&texture_title, &texture_message, rect_title, rect_message, canvas, SCREEN_DURATION);

    // fade fo black
    fade_message(&mut texture_title, &mut texture_message, rect_title, rect_message, canvas, FADE_DURATION, true);
}
fn message_to_screen(
    title: &Texture,
    message: &Texture,
    rect_title: Rect,
    rect_message: Rect,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    duration: u64
){
    canvas.copy(title, None, rect_title).expect("Failed to copy Disclaimer's title texture to canvas!");
    canvas.copy(message, None, rect_message).expect("Failed to copy Disclaimer's message texture to canvas!");
    canvas.present();
    ::std::thread::sleep(Duration::new(duration, 0));
}

fn fade_message(
    title: &mut Texture,
    message: &mut Texture,
    rect_title: Rect,
    rect_message: Rect,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    duration: u64,
    crescendo: bool
) {
    // fade to the black
    for i in 0..250 {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        if !crescendo {
            title.set_color_mod(i,i,i);
            message.set_color_mod(i,i,i);
        }else {
            title.set_color_mod(250 - i, 250 - i, 250 - i);
            message.set_color_mod(250 - i, 250 - i, 250 - i);
        }
        message_to_screen(title, message, rect_title, rect_message, canvas, duration / 250);
    }
}
