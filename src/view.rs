use crate::sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

use crate::GameState;
use crate::Spaceship;

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

pub fn draw_spaceship(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    spaceship: &Spaceship
){
    // body
    canvas.set_draw_color(spaceship.body.body_color);
    canvas.fill_rect(spaceship.body.rect).expect("Drawing failed for spaceship's body.");
    // body's porthole #1
    canvas.set_draw_color(spaceship.body.porthole_color);
    canvas.filled_circle(
        spaceship.body.porthole_1.0,
        spaceship.body.porthole_1.1,
        spaceship.body.porthole_r,
        spaceship.body.porthole_color
    ).expect("Drawing failed for spaceship's first porthole!");
    // body's porthole #2
    canvas.set_draw_color(spaceship.body.porthole_color);
    canvas.filled_circle(
        spaceship.body.porthole_2.0,
        spaceship.body.porthole_2.1,
        spaceship.body.porthole_r,
        spaceship.body.porthole_color
    ).expect("Drawing failed for spaceship's second porthole!");
    // head
    canvas.set_draw_color(spaceship.head.color);
    canvas
        .filled_polygon(&spaceship.head.triangle_x, &spaceship.head.triangle_y, spaceship.head.color)
        .expect("Drawing failed for spaceship's head!");
    // tail
    canvas.set_draw_color(spaceship.tail.color);
    canvas
        .filled_polygon(&spaceship.tail.left_triangle_x, &spaceship.tail.left_triangle_y, spaceship.tail.color)
        .expect("Drawing failed for spaceship's left leg!");
    canvas
        .filled_polygon(&spaceship.tail.right_triangle_x, &spaceship.tail.right_triangle_y, spaceship.tail.color)
        .expect("Drawing failed for spaceship's right leg!");
}

pub fn draw_score(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    gs: &GameState
){
    // to do
}

pub fn draw_game(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    gs: &GameState
){
    draw_background(canvas);
    draw_missiles(canvas, &gs);
    draw_spaceship(canvas, &gs.spaceship_p1);
    draw_spaceship(canvas, &gs.spaceship_p2);
    draw_score(canvas, &gs);
    canvas.present();
}
