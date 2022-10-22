extern crate sdl2;

use crate::disclaimer::*;
use crate::entity::*;
use crate::view::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use soloud::*;

use std::time::Duration;
use std::path::Path;

mod disclaimer;
mod entity;
mod missile;
mod spaceship;
mod view;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "space race";

// The time between two frames in milliseconds.
const FRAME_DURATION: u32 = 50;

// The time for 1 game in seconds.
const GAME_DURATION: Duration = Duration::new(45, 0);

const MISSILE_QUANTITY: usize = 10;
struct FrameEvent;

struct GameSFX {
    soloud: Soloud,
    collision_wav: Wav,
}

impl GameSFX {
    pub fn new() -> GameSFX {
        let sl = Soloud::default().expect("Failed to get Soloud object!");

        let mut sounds = GameSFX {
            soloud: sl,
            collision_wav: audio::Wav::default(),
        };

        let sfx_collision_path: &Path = std::path::Path::new("asset/sfx/pew.wav");
        sounds
            .collision_wav
            .load(sfx_collision_path)
            .unwrap_or_else(|_| {
                panic!(
                    "failed to load sfx file {} for collision",
                    sfx_collision_path.display()
                )
            });

        sounds
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    show_disclaimer(&mut canvas);
    game_loop(&sdl_context, &mut canvas);
}

fn game_loop(context: &sdl2::Sdl, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
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
            handle_events(&mut gs, &mut event_pump, &sounds, canvas);
        }
        if gs.is_game_elapsed() {
            gs.is_game_restarted = true;
        }
    }
}

fn handle_events(
    gs: &mut GameState,
    event_pump: &mut EventPump,
    sounds: &GameSFX,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    let event = event_pump.wait_event();

    if event.is_user_event() {
        // update missile coordinates here
        for m in &mut gs.missiles {
            m.update();
        }

        if gs.spaceship_p1.is_alive && gs.collision_occurred_for(&gs.spaceship_p1) {
            gs.spaceship_p1.die();
            sounds.soloud.play(&sounds.collision_wav);
        }
        if gs.spaceship_p2.is_alive && gs.collision_occurred_for(&gs.spaceship_p2) {
            gs.spaceship_p2.die();
            sounds.soloud.play(&sounds.collision_wav);
        }
        update_cpu(gs);
        if !gs.spaceship_p1.is_alive && gs.spaceship_p1.can_respawn() {
            gs.reset_spaceship_p1();
        }
        draw_game(canvas, gs);
    } else {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                gs.is_game_over = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                if gs.spaceship_p1.is_alive {
                    gs.spaceship_p1.move_upward();
                    if GameState::has_spaceship_scored(&gs.spaceship_p1) {
                        gs.score_p1 += 1;
                        gs.reset_spaceship_p1();
                    }
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                if gs.spaceship_p1.is_alive && gs.spaceship_p1.can_move_downward() {
                    gs.spaceship_p1.move_downward();
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                gs.is_game_restarted = true;
            }
            _ => {}
        }
    }
}

fn update_cpu(gs: &mut GameState) {
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
        gs.score_p2 += 1;
        gs.reset_spaceship_p2();
    }
}
