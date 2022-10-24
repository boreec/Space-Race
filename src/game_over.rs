use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::path::Path;

use crate::GameFont;
use crate::GameState;

const VICTORY_TITLE: &str = "VICTORY";
const DEFEAT_TITLE: &str = "DEFEAT";
const DRAW_TITLE: &str= "DRAW";

pub fn show_game_over(gs: &mut GameState, gf: &GameFont, canvas: &mut Canvas<Window>) -> bool {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    
    let poetsen_font_path: &Path = Path::new("asset/font/poetsen_one/PoetsenOne-Regular.ttf");

    let big_font = gf.context
        .load_font(poetsen_font_path, 128)
        .unwrap_or_else(|_| panic!("Failed to load font {}", poetsen_font_path.display()));
    
    let small_font = gf.context
        .load_font(poetsen_font_path, 24)
        .unwrap_or_else(|_| panic!("Failed to load font {}", poetsen_font_path.display()));


    true
}

