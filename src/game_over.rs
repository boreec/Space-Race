use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::cmp::Ordering;
use std::path::Path;
use std::time::Duration;

use crate::GameFont;
use crate::GameState;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

const VICTORY_TITLE: &str = "VICTORY";
const DEFEAT_TITLE: &str = "DEFEAT";
const DRAW_TITLE: &str= "DRAW";

const TITLE_WIDTH: u32 = 200;
const TITLE_HEIGHT: u32 = 100;

pub fn show_game_over(gs: &mut GameState, gf: &GameFont, canvas: &mut Canvas<Window>) -> bool {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    
    let poetsen_font_path: &Path = Path::new("asset/font/poetsen_one/PoetsenOne-Regular.ttf");
    let texture_creator = canvas.texture_creator();

    let big_font = gf.context
        .load_font(poetsen_font_path, 128)
        .unwrap_or_else(|_| panic!("Failed to load font {}", poetsen_font_path.display()));
    
    let small_font = gf.context
        .load_font(poetsen_font_path, 24)
        .unwrap_or_else(|_| panic!("Failed to load font {}", poetsen_font_path.display()));

    let title_str: &str = 
        match gs.score_p1.cmp(&gs.score_p2) {
            Ordering::Less => { DEFEAT_TITLE },
            Ordering::Equal => { DRAW_TITLE },
            Ordering::Greater => { VICTORY_TITLE },
        };
    
    let surface_title = big_font
        .render(title_str)
        .blended(Color::WHITE)
        .expect("Failed to create font surface for Game over's screen!");
    
    let rect_title = Rect::new(
        (WINDOW_WIDTH / 2 - TITLE_WIDTH / 2) as i32,
        0,
        TITLE_WIDTH,
        TITLE_HEIGHT,
    );
    
    let mut texture_title = texture_creator
        .create_texture_from_surface(&surface_title)
        .expect("Failed to created texture for Game Over's screen title!");
    
    canvas
        .copy(&texture_title, None, rect_title)
        .expect("Failed to copy Game Over's Title's texture to canvas!");
    canvas.present();
    ::std::thread::sleep(Duration::new(3, 0));
    
    true
}

