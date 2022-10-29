extern crate sdl2;

use crate::disclaimer::*;
use crate::game_event::*;
use crate::game_font::GameFont;
use crate::game_over::*;
use crate::game_sfx::GameSFX;
use crate::game_state::GameState;
use crate::spaceship::*;

use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

mod disclaimer;
mod game_event;
mod game_font;
mod game_over;
mod game_sfx;
mod game_state;
mod missile;
mod spaceship;
mod view;

// Window constants used to setup the game's window.
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "space race";

// The time between two frames in milliseconds.
const FRAME_DURATION: u32 = 50;

// The time for 1 game in seconds.
const GAME_DURATION: Duration = Duration::new(45, 0);

const MISSILE_QUANTITY: usize = 20;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let gf = GameFont::new();
    show_disclaimer(&gf, &mut canvas);
    run_game(&sdl_context, &mut canvas, &gf);
}

fn run_game(context: &sdl2::Sdl, canvas: &mut Canvas<Window>, gf: &GameFont) {
    let mut gs: GameState = GameState::new(MISSILE_QUANTITY, GAME_DURATION);
    let sounds: GameSFX = GameSFX::new();
    let mut event_pump = context.event_pump().unwrap();
    let ev = context.event().unwrap();
    ev.register_custom_event::<FrameEvent>().unwrap();

    let timer_subsystem = context.timer().unwrap();
    let _timer = timer_subsystem.add_timer(
        FRAME_DURATION,
        Box::new(|| {
            ev.push_custom_event(FrameEvent).unwrap();
            FRAME_DURATION
        }),
    );

    while gs.is_game_restarted {
        gs = GameState::new(MISSILE_QUANTITY, GAME_DURATION);
        gs.is_game_restarted = false;
        while !gs.is_game_over && !gs.is_game_elapsed() && !gs.is_game_restarted {
            handle_game_events(&mut gs, &mut event_pump, &sounds, canvas, gf);
        }
        if gs.is_game_elapsed() {
            gs.is_game_restarted = show_game_over(&mut gs, gf, canvas, &mut event_pump);
        }
    }
}

fn update_cpu(gs: &mut GameState, sounds: &GameSFX) {
    if !gs.spaceship_p2.is_alive {
        if gs.spaceship_p2.can_respawn() {
            gs.reset_spaceship_p2();
            return;
        } else {
            // The delay to respawn is not over.
            return;
        }
    }
    gs.spaceship_p2.move_upward();
    if GameState::has_spaceship_scored(&gs.spaceship_p2) {
        sounds.play_score();
        gs.score_p2 += 1;
        gs.reset_spaceship_p2();
    }
}
