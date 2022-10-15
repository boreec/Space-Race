use crate::sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::path::Path;

use crate::GameState;
use crate::Spaceship;
use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;

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
    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed!");
    let texture_creator = canvas.texture_creator();
    let schluber_font_path: &Path = Path::new("font/schluber/Schluber.ttf");

    let font = ttf_context
        .load_font(schluber_font_path, 128)
        .expect(&format!("Failed to load font {}", schluber_font_path.display()));

    let surface_p1 = font
        .render(&format!("{}", gs.score_p1))
        .blended(Color::WHITE)
        .expect("failed to create font surface for player #1 score");

    let surface_p2 = font
        .render(&format!("{}", gs.score_p2))
        .blended(Color::WHITE)
        .expect("failed to create font surface for player #2 score");

    let screen_padding = 20;
    let font_rect_width = 30;
    let font_rect_height = 20;
    let font_rect_p1 = Rect::new(
        screen_padding as i32,
        (WINDOW_HEIGHT - font_rect_height - screen_padding) as i32,
        font_rect_width,
        font_rect_height
    );

    let font_rect_p2 = Rect::new(
        (WINDOW_WIDTH - font_rect_width - screen_padding) as i32,
        (WINDOW_HEIGHT - font_rect_height - screen_padding) as i32,
        font_rect_width,
        font_rect_height
    );

    let texture_p1 = texture_creator
        .create_texture_from_surface(&surface_p1)
        .expect("Failed to create texture from surface for p1!");
    
    let texture_p2 = texture_creator
        .create_texture_from_surface(&surface_p2)
        .expect("Failed to create texture from surface for p2!");

    canvas.copy(&texture_p1, None, font_rect_p1).expect("Failed to copy p1 texture to canvas");
    canvas.copy(&texture_p2, None, font_rect_p2).expect("Failed to copy p2 texture to canvas");
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
