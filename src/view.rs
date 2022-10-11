use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::GameState;

const COLOR_BACKGROUND: Color = Color::BLACK;

fn draw_background(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(COLOR_BACKGROUND);
    canvas.clear();
}

pub fn draw_missiles(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    gs: &GameState
){
    for m in &gs.missiles {
        // draw body
        canvas.set_draw_color(m.body.color);
        canvas.fill_rect(m.body.rect).expect("Drawing failed for missile!");
    }
}

pub fn draw_game(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    gs: &GameState
){
    draw_background(canvas);
    draw_missiles(canvas, &gs);
    canvas.present();
}
