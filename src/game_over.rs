use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::path::Path;

use crate::GameState;

const VICTORY_TITLE: &str = "VICTORY";
const DEFEAT_TITLE: &str = "DEFEAT";
const DRAW_TITLE: &str= "DRAW";

pub fn show_game_over(gs: &mut GameState, canvas: &mut Canvas<Window>) -> bool {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    
    let ttf_context = sdl2::ttf::init().unwrap_or_else(|_| panic!("failed to initialize TTLF for SDL!"));
    let poetsen_font_path: &Path = Path::new("asset/font/poetsen_one/PoetsenOne-Regular.ttf");

    let big_font = ttf_context
        .load_font(poetsen_font_path, 128)
        .unwrap_or_else(|_| panic!("Failed to load font {}", poetsen_font_path.display()));
    
    let small_font = ttf_context
        .load_font(poetsen_font_path, 24)
        .unwrap_or_else(|_| panic!("Failed to load font {}", poetsen_font_path.display()));


    true
}

