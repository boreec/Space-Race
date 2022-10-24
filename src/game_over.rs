use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::GameState;

const VICTORY_TITLE: &str = "VICTORY";
const DEFEAT_TITLE: &str = "DEFEAT";
const DRAW_TITLE: &str= "DRAW";

pub fn show_game_over(gs: &mut GameState, canvas: &mut Canvas<Window>) -> bool {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    // to do
    true
}

