use crate::update_cpu;
use crate::GameFont;
use crate::GameSFX;
use crate::GameState;
use crate::view::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct FrameEvent;

pub fn handle_game_events(
    gs: &mut GameState,
    event_pump: &mut EventPump,
    sounds: &GameSFX,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    gf: &GameFont
) {
    let event = event_pump.wait_event();

    if event.is_user_event() {
        // update missile coordinates here
        for m in &mut gs.missiles {
            m.update();
        }

        if gs.spaceship_p1.is_alive && gs.collision_occurred_for(&gs.spaceship_p1) {
            gs.spaceship_p1.die();
            sounds.play_collision();
        }
        if gs.spaceship_p2.is_alive && gs.collision_occurred_for(&gs.spaceship_p2) {
            gs.spaceship_p2.die();
            sounds.play_collision();
        }
        update_cpu(gs, sounds);
        if !gs.spaceship_p1.is_alive && gs.spaceship_p1.can_respawn() {
            gs.reset_spaceship_p1();
        }
        draw_game(canvas, gs, gf);
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
                        sounds.play_score();
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