use crate::sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

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

        // draw tail
        canvas.set_draw_color(m.tail.color);
        canvas
            .filled_polygon(&m.tail.top_triangle_x, &m.tail.top_triangle_y, m.tail.color)
            .expect("Drawing failed for top missile's tail!");
        canvas
            .filled_polygon(&m.tail.bot_triangle_x, &m.tail.bot_triangle_y, m.tail.color)
            .expect("Drawing failed for bottom missile's tail!");

        //draw head
        canvas.set_draw_color(m.head.color);
        canvas
            .filled_polygon(&m.head.triangle_x, &m.head.triangle_y, m.head.color)
            .expect("Drawing failed for missile's head!");

    }
}

pub fn draw_spaceships(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    gs: &GameState
){
    // draw p1 spaceship
    // body
    canvas.set_draw_color(gs.spaceship_p1.body.body_color);
    canvas.fill_rect(gs.spaceship_p1.body.rect).expect("Drawing failed for p1's spaceship's body.");
    // body's porthole #1
    canvas.set_draw_color(gs.spaceship_p1.body.porthole_color);
    canvas.filled_circle(
        gs.spaceship_p1.body.porthole_1.0,
        gs.spaceship_p1.body.porthole_1.1,
        gs.spaceship_p1.body.porthole_r,
        gs.spaceship_p1.body.porthole_color
    ).expect("Drawing failed for p1's first porthole!");
    // body's porthole #2
    canvas.set_draw_color(gs.spaceship_p1.body.porthole_color);
    canvas.filled_circle(
        gs.spaceship_p1.body.porthole_2.0,
        gs.spaceship_p1.body.porthole_2.1,
        gs.spaceship_p1.body.porthole_r,
        gs.spaceship_p1.body.porthole_color
    ).expect("Drawing failed for p1's second porthole!");
    // head
    canvas.set_draw_color(gs.spaceship_p1.head.color);
    canvas
        .filled_polygon(&gs.spaceship_p1.head.triangle_x, &gs.spaceship_p1.head.triangle_y, gs.spaceship_p1.head.color)
        .expect("Drawing failed for p1's spaceship's head");

    // draw p2 spaceship
    // body
    canvas.set_draw_color(gs.spaceship_p2.body.body_color);
    canvas.fill_rect(gs.spaceship_p2.body.rect).expect("Drawing failed for p2's spaceship's body.");
    // head
    canvas.set_draw_color(gs.spaceship_p2.head.color);
    canvas
        .filled_polygon(&gs.spaceship_p2.head.triangle_x, &gs.spaceship_p2.head.triangle_y, gs.spaceship_p2.head.color)
        .expect("Drawing failed for p1's spaceship's head");

}

pub fn draw_game(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    gs: &GameState
){
    draw_background(canvas);
    draw_missiles(canvas, &gs);
    draw_spaceships(canvas, &gs);
    canvas.present();
}
