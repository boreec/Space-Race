use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::ops::RangeInclusive;

use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

const MISSILE_HEIGHT: u32 = 5;
const MISSILE_WIDTH: u32 = 10;
const MISSILE_BODY_COLOR: Color = Color::WHITE;
const MISSILE_QUANTITY: usize = 10;
const MISSILE_SPAWN_RANGE_X: RangeInclusive<u32> = 0..=(WINDOW_WIDTH - MISSILE_WIDTH);
const MISSILE_SPAWN_RANGE_Y: RangeInclusive<u32> = 0..=(WINDOW_HEIGHT - 200);

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
    pub body: Rect,
    pub color_body: Color,
}

impl Missile {
    pub fn new() -> Missile {
        let mut rng = rand::thread_rng();
        let pos_x = rng.gen_range(MISSILE_SPAWN_RANGE_X) as i32;
        let pos_y = rng.gen_range(MISSILE_SPAWN_RANGE_Y) as i32;
        let r = Rect::new(pos_x, pos_y, MISSILE_WIDTH, MISSILE_HEIGHT);

        return Missile {
            x: pos_x,
            y: pos_y,
            body: r,
            color_body: MISSILE_BODY_COLOR,
        }
    }
}
