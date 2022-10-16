extern crate sdl2;

use crate::entity::*;
use crate::view::*;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use std::time::Duration;

mod entity;
mod view;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "space race";

const FRAME_DURATION: u32 = 50;

const GAME_DURATION: Duration = Duration::new(45, 0);

struct FrameEvent;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    game_loop(&sdl_context, &mut canvas);
}

fn game_loop(context: &sdl2::Sdl, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    let mut gs: GameState = GameState::new();
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
        gs = GameState::new();
        gs.is_game_restarted = false;
        while !gs.is_game_over && gs.starting_time.elapsed().as_secs() < GAME_DURATION.as_secs() && !gs.is_game_restarted {
            handle_events(&mut gs, &mut event_pump, canvas);
        }
    }
}

fn handle_events(
    gs: &mut GameState,
    event_pump: &mut EventPump,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){

    let event = event_pump.wait_event();

    if event.is_user_event() {
        // update missile coordinates here
        for m in &mut gs.missiles {
            m.update();
        }
        draw_game(canvas, &gs);
    }
    else {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                gs.is_game_over = true;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                gs.spaceship_p1.move_upward();
                if GameState::has_spaceship_scored(&gs.spaceship_p1) {
                    gs.score_p1 += 1;
                    gs.reset_spaceship_p1();
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                if gs.spaceship_p1.can_move_downward() {
                    gs.spaceship_p1.move_downward();
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                gs.is_game_restarted = true;
            }
            _ => {}
        }
    }
}
