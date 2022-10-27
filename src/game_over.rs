use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::Window;

use std::cmp::Ordering;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

use crate::GameFont;
use crate::GameState;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

const VICTORY_TITLE: &str = "VICTORY";
const DEFEAT_TITLE: &str = "DEFEAT";
const DRAW_TITLE: &str= "DRAW";

const TITLE_WIDTH: u32 = 200;
const TITLE_HEIGHT: u32 = 100;
const REPLAY_WIDTH: u32 = 600;
const REPLAY_HEIGHT: u32 = 30;

const SCREEN_PADDING: i32 = 20;
const SCREEN_DURATION: u64 = 10;

pub fn show_game_over(gs: &mut GameState, gf: &GameFont, canvas: &mut Canvas<Window>) -> bool {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    
    let texture_creator = canvas.texture_creator();

    let big_font = gf.get_font(&gf.poetsen_path, 128);
    
    let small_font = gf.get_font(&gf.poetsen_path, 128);

    let title_str: &str = 
        match gs.score_p1.cmp(&gs.score_p2) {
            Ordering::Less => { DEFEAT_TITLE },
            Ordering::Equal => { DRAW_TITLE },
            Ordering::Greater => { VICTORY_TITLE },
        };
    
    let message_str: &str = 
        match gs.score_p1.cmp(&gs.score_p2) {
            Ordering::Less => {"You lost! Are you going to stop on a defeat ?"},
            Ordering::Equal => { "It's a tie! One single point would have been enough to win!"},
            Ordering::Greater => { "You won! You have nothing to prove anymore!"},        
        };
    
    let replay_str: &str = "Press [space] key to replay now, or the game will end!";
    
    let surface_title = gf.surface_from_str(title_str, &big_font, Color::WHITE);
    
    let surface_message = gf.surface_from_str(message_str, &small_font, Color::WHITE);
    
    let surface_replay = gf.surface_from_str(replay_str, &small_font, Color::WHITE);
    
    let rect_title = Rect::new(
        (WINDOW_WIDTH / 2 - TITLE_WIDTH / 2) as i32,
        0,
        TITLE_WIDTH,
        TITLE_HEIGHT,
    );
    
    let rect_message = Rect::new(
        SCREEN_PADDING,
        (WINDOW_HEIGHT / 2) as i32,
        WINDOW_WIDTH - 2 * SCREEN_PADDING as u32,
        30
    );
    
    let rect_replay = Rect::new(
        (WINDOW_WIDTH / 2 - REPLAY_WIDTH / 2) as i32,
        (WINDOW_HEIGHT / 2) as i32 + SCREEN_PADDING + 30,
        REPLAY_WIDTH,
        REPLAY_HEIGHT
    ); 
    
    let texture_title = texture_creator
        .create_texture_from_surface(&surface_title)
        .expect("Failed to create texture for Game Over's screen title!");
    
    let texture_message = texture_creator
        .create_texture_from_surface(&surface_message)
        .expect("Failed to create texture for Game Over's main message!");
    
    let texture_replay = texture_creator
        .create_texture_from_surface(&surface_replay)
        .expect("Failed to write replay's line in Game Over's screen!");
    
    canvas
        .copy(&texture_title, None, rect_title)
        .expect("Failed to copy Game Over's Title's texture to canvas!");
    canvas
        .copy(&texture_message, None, rect_message)
        .expect("Failed to copy Game Over's message's texture to canvas!");
    canvas
        .copy(&texture_replay, None, rect_replay)
        .expect("Failed to copy Game Over's replay's texture to canvas!");
    
    canvas.present();
    
    return handle_game_over_events();
}

fn handle_game_over_events() -> bool {
    // to do
    let start = Instant::now();
    while start.elapsed().as_secs() < SCREEN_DURATION {
        ::std::thread::sleep(Duration::new(1,0));    
    }
    return true;
}
