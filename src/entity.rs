use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::ops::RangeInclusive;

use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

const MISSILE_HEIGHT: u32 = 5;
const MISSILE_WIDTH: u32 = 10;
const MISSILE_TAIL_SIZE: u32 = MISSILE_HEIGHT;
const MISSILE_HEAD_SIZE: u32 = MISSILE_HEIGHT;
const MISSILE_BODY_COLOR: Color = Color::WHITE;
const MISSILE_TAIL_COLOR: Color = Color::GREY;
const MISSILE_HEAD_COLOR: Color = Color::RED;
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

pub enum MissileDirection {
    LEFT,
    RIGHT,
}

pub struct Missile {
    pub x: i32,
    pub y: i32,
    pub body: MissileBody,
    pub tail: MissileTail,
    pub head: MissileHead,
}

pub struct MissileBody {
    pub rect: Rect,
    pub color: Color,
}

pub struct MissileTail {
    pub top_triangle_x: [i16;3],
    pub top_triangle_y: [i16;3],
    pub bot_triangle_x: [i16;3],
    pub bot_triangle_y: [i16;3],
    pub color: Color,
}

pub struct MissileHead {
    pub triangle_x: [i16;3],
    pub triangle_y: [i16;3],
    pub color: Color,
}

impl MissileBody {
    fn new(r: Rect) -> MissileBody {
        return MissileBody {
            rect: r,
            color: MISSILE_BODY_COLOR,
        };
    }
}

impl MissileTail {
    pub fn new(origin_x: i16, origin_y: i16) -> MissileTail {

        return MissileTail {
            top_triangle_x: [origin_x, origin_x, origin_x + MISSILE_TAIL_SIZE as i16],
            top_triangle_y: [origin_y, origin_y - MISSILE_TAIL_SIZE as i16, origin_y],
            bot_triangle_x: [origin_x, origin_x, origin_x + MISSILE_TAIL_SIZE as i16],
            bot_triangle_y: [origin_y + MISSILE_HEIGHT as i16, origin_y + (MISSILE_HEIGHT + MISSILE_TAIL_SIZE) as i16, origin_y + MISSILE_HEIGHT as i16],
            color: MISSILE_TAIL_COLOR,
        };
    }
}

impl MissileHead {
    pub fn new(origin_x: i16, origin_y: i16) -> MissileHead {
        return MissileHead {
            triangle_x: [origin_x + MISSILE_WIDTH as i16, origin_x + (MISSILE_WIDTH + MISSILE_HEAD_SIZE) as i16, origin_x + MISSILE_WIDTH as i16],
            triangle_y: [origin_y, origin_y + MISSILE_HEIGHT as i16 / 2, origin_y + MISSILE_HEIGHT as i16],
            color: MISSILE_HEAD_COLOR,
        };
    }
}

impl Missile {
    pub fn new() -> Missile {

        let mut rng = rand::thread_rng();
        let pos_x = rng.gen_range(MISSILE_SPAWN_RANGE_X) as i32;
        let pos_y = rng.gen_range(MISSILE_SPAWN_RANGE_Y) as i32;

        let r = Rect::new(pos_x, pos_y, MISSILE_WIDTH, MISSILE_HEIGHT);
        let m = MissileBody::new(r);
        let t = MissileTail::new(pos_x as i16, pos_y as i16);
        let h = MissileHead::new(pos_x as i16, pos_y as i16);

        return Missile {
            x: pos_x,
            y: pos_y,
            body: m,
            tail: t,
            head: h,
        };
    }
}
