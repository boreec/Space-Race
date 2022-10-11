use rand::Rng;

use sdl2::pixels::Color;

use std::ops::RangeInclusive;

use crate::WINDOW_WIDTH;

const MISSILE_HEIGHT: u32 = 5;
const MISSILE_WIDTH: u32 = 10;
const MISSILE_COLOR: Color = Color::WHITE;
const MISSILE_QUANTITY: usize = 10;
const MISSILE_SPAWN_RANGE_X: RangeInclusive<u32> = 0..=(WINDOW_WIDTH - MISSILE_WIDTH);

pub struct GameState {
    pub missiles: Vec<Missile>,
    pub is_game_over: bool,
}

impl GameState {
    pub fn new() -> GameState {

        let mut random_missiles = Vec::new();

        for _ in 0..MISSILE_QUANTITY {
            random_missiles.push(Missile::new());
        }

        return GameState {
            is_game_over: false,
            missiles: random_missiles,
        };
    }
}

#[derive(Copy, Clone)]
pub struct Missile {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
    pub color: Color,
}

impl Missile {
    pub fn new() -> Missile {
        let mut rng = rand::thread_rng();
        return Missile {
            x: rng.gen_range(MISSILE_SPAWN_RANGE_X) as i32,
            y: 0,
            height: MISSILE_HEIGHT,
            width: MISSILE_WIDTH,
            color: MISSILE_COLOR,
        }
    }
}
