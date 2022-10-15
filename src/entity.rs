use rand::random;
use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::ops::RangeInclusive;

use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

/* SPACESHIP CONSTANTS */
const SPACESHIP_SPEED: i64 = 5;
const SPACESHIP_BODY_WIDTH: u32 = 50;
const SPACESHIP_BODY_HEIGHT: u32 = 125;
const SPACESHIP_BODY_COLOR: Color = Color::WHITE;
const SPACESHIP_PORTHOLE_COLOR: Color = Color::BLUE;
const SPACESHIP_HEAD_SIZE: u32 = SPACESHIP_BODY_WIDTH;
const SPACESHIP_HEAD_COLOR: Color = Color::RED;
const SPACESHIP_TAIL_SIZE: i16 = (SPACESHIP_BODY_WIDTH / 4 * 3) as i16;
const SPACESHIP_TAIL_COLOR: Color = Color::RED;
const SPACESHIP_P1_X: i32 = (WINDOW_WIDTH / 4 - SPACESHIP_BODY_WIDTH / 2) as i32;
const SPACESHIP_P1_Y: i32 = (WINDOW_HEIGHT - SPACESHIP_BODY_HEIGHT) as i32;
const SPACESHIP_P2_X: i32 = SPACESHIP_P1_X + (WINDOW_WIDTH / 2) as i32;
const SPACESHIP_P2_Y: i32 = SPACESHIP_P1_Y;


/* MISSILE CONSTANTS */
const MISSILE_HEIGHT: u32 = 5;
const MISSILE_WIDTH: u32 = 10;
const MISSILE_SPEED: i64 = 2;
const MISSILE_TAIL_SIZE: u32 = MISSILE_HEIGHT;
const MISSILE_HEAD_SIZE: u32 = MISSILE_HEIGHT;
const MISSILE_BODY_COLOR: Color = Color::WHITE;
const MISSILE_TAIL_COLOR: Color = Color::GREY;
const MISSILE_HEAD_COLOR: Color = Color::RED;
const MISSILE_QUANTITY: usize = 10;
const MISSILE_SPAWN_RANGE_X: RangeInclusive<u32> = 0..=WINDOW_WIDTH;
const MISSILE_SPAWN_RANGE_Y: RangeInclusive<u32> = 0..=(WINDOW_HEIGHT - 200);

pub struct GameState {
    pub missiles: Vec<Missile>,
    pub spaceship_p1: Spaceship,
    pub spaceship_p2: Spaceship,
    pub is_game_over: bool,
    pub is_game_restarted: bool,
    pub score_p1: u32,
    pub score_p2: u32,
}

impl GameState {
    pub fn new() -> GameState {

        let mut random_missiles = Vec::new();

        for _ in 0..MISSILE_QUANTITY {
            random_missiles.push(Missile::new());
        }

        return GameState {
            is_game_over: false,
            is_game_restarted: true,
            missiles: random_missiles,
            spaceship_p1: Spaceship::new(SPACESHIP_P1_X, SPACESHIP_P1_Y),
            spaceship_p2: Spaceship::new(SPACESHIP_P2_X, SPACESHIP_P2_Y),
            score_p1: 0,
            score_p2: 0,
        };
    }
}

// ** SPACESHIP ** //

pub struct Spaceship {
    pub body: SpaceshipBody,
    pub head: SpaceshipHead,
    pub tail: SpaceshipTail,
}

impl Spaceship {
    pub fn new(pos_x: i32, pos_y: i32) -> Spaceship {
        return Spaceship {
            body: SpaceshipBody::new(pos_x, pos_y, SPACESHIP_BODY_WIDTH, SPACESHIP_BODY_HEIGHT),
            head: SpaceshipHead::new(pos_x as i16, pos_y as i16),
            tail: SpaceshipTail::new(pos_x as i16, pos_y as i16),
        };
    }

    pub fn move_upward(&mut self){
        self.body.move_upward();
        self.head.move_upward();
        self.tail.move_upward();
    }

    pub fn move_downward(&mut self){
        self.body.move_downward();
        self.head.move_downward();
        self.tail.move_downward();
    }
}

pub struct SpaceshipBody {
    pub rect: Rect,
    pub body_color: Color,
    pub porthole_1: (i16, i16),
    pub porthole_2: (i16, i16),
    pub porthole_r: i16,
    pub porthole_color: Color,
}

impl SpaceshipBody {
    pub fn new(pos_x: i32, pos_y: i32, width: u32, height: u32) -> SpaceshipBody {
        let porthole_radius = (width / 3) as i16;

        return SpaceshipBody {
            rect: Rect::new(pos_x, pos_y, width, height),
            body_color: SPACESHIP_BODY_COLOR,
            porthole_1: (pos_x as i16 + width as i16 / 2, pos_y as i16 + porthole_radius * 2),
            porthole_2: (pos_x as i16 + width as i16 / 2, pos_y as i16 + porthole_radius * 5),
            porthole_r: porthole_radius,
            porthole_color: SPACESHIP_PORTHOLE_COLOR,
        };
    }

    pub fn move_upward(&mut self){
        self.rect.set_y(self.rect.y() - SPACESHIP_SPEED as i32);
        self.porthole_1.1 -= SPACESHIP_SPEED as i16;
        self.porthole_2.1 -= SPACESHIP_SPEED as i16;
    }

    pub fn move_downward(&mut self){
        self.rect.set_y(self.rect.y() + SPACESHIP_SPEED as i32);
        self.porthole_1.1 += SPACESHIP_SPEED as i16;
        self.porthole_2.1 += SPACESHIP_SPEED as i16;
    }
}

pub struct SpaceshipHead {
    pub triangle_x: [i16;3],
    pub triangle_y: [i16;3],
    pub color: Color,
}

impl SpaceshipHead {
    pub fn new(pos_x: i16, pos_y: i16) -> SpaceshipHead {
        return SpaceshipHead {
            triangle_x: [pos_x, pos_x + (SPACESHIP_HEAD_SIZE / 2) as i16, pos_x + SPACESHIP_BODY_WIDTH as i16],
            triangle_y: [pos_y, pos_y - SPACESHIP_HEAD_SIZE as i16, pos_y],
            color: SPACESHIP_HEAD_COLOR,
        };
    }

    pub fn move_upward(&mut self){
        self.triangle_y = self.triangle_y.map(|v| v - SPACESHIP_SPEED as i16);
    }

    pub fn move_downward(&mut self){
        self.triangle_y = self.triangle_y.map(|v| v + SPACESHIP_SPEED as i16);
    }
}

pub struct SpaceshipTail {
    pub left_triangle_x: [i16; 3],
    pub left_triangle_y: [i16; 3],
    pub right_triangle_x: [i16; 3],
    pub right_triangle_y: [i16; 3],
    pub color: Color,
}

impl SpaceshipTail {
    pub fn new(pos_x: i16, pos_y: i16) -> SpaceshipTail {
        return SpaceshipTail {
            left_triangle_x: [pos_x, pos_x, pos_x - SPACESHIP_TAIL_SIZE],
            left_triangle_y: [
                pos_y + SPACESHIP_BODY_HEIGHT as i16 - SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16 + SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16],
            right_triangle_x : [
                pos_x + SPACESHIP_BODY_WIDTH as i16,
                pos_x + SPACESHIP_BODY_WIDTH as i16,
                pos_x + SPACESHIP_BODY_WIDTH as i16 + SPACESHIP_TAIL_SIZE],
            right_triangle_y: [
                pos_y + SPACESHIP_BODY_HEIGHT as i16 - SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16 + SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16],
            color: SPACESHIP_TAIL_COLOR,
        };
    }

    pub fn move_upward(&mut self){
        self.left_triangle_y = self.left_triangle_y.map(|v| v - SPACESHIP_SPEED as i16);
        self.right_triangle_y = self.right_triangle_y.map(|v| v - SPACESHIP_SPEED as i16);
    }

    pub fn move_downward(&mut self){
        self.left_triangle_y = self.left_triangle_y.map(|v| v + SPACESHIP_SPEED as i16);
        self.right_triangle_y = self.right_triangle_y.map(|v| v + SPACESHIP_SPEED as i16);
    }

}

// ** MISSILES ** //
#[derive(PartialEq)]
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
    pub direction: MissileDirection,
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

    fn move_towards(&mut self, direction: &MissileDirection){
        match direction {
            MissileDirection::LEFT => { self.rect.set_x(self.rect.x() - MISSILE_SPEED as i32); }
            MissileDirection::RIGHT => { self.rect.set_x(self.rect.x() + MISSILE_SPEED as i32); }
        }
    }
}

impl MissileTail {
    pub fn new(origin_x: i16, origin_y: i16, direction: &MissileDirection) -> MissileTail {
        let top_triangle_xs: [i16;3];
        let top_triangle_ys: [i16;3];
        let bot_triangle_ys: [i16;3];
        match direction {
            MissileDirection::LEFT => {
                top_triangle_xs = [
                    origin_x + (MISSILE_WIDTH - MISSILE_TAIL_SIZE) as i16,
                    origin_x + MISSILE_WIDTH as i16,
                    origin_x + MISSILE_WIDTH as i16,
                ];
                top_triangle_ys = [origin_y, origin_y, origin_y - MISSILE_TAIL_SIZE as i16];
                bot_triangle_ys = [origin_y + MISSILE_HEIGHT as i16, origin_y + MISSILE_HEIGHT as i16, origin_y + (MISSILE_HEIGHT + MISSILE_TAIL_SIZE) as i16];
            }
            MissileDirection::RIGHT => {
                top_triangle_xs = [origin_x, origin_x, origin_x + MISSILE_TAIL_SIZE as i16];
                top_triangle_ys = [origin_y, origin_y - MISSILE_TAIL_SIZE as i16, origin_y];
                bot_triangle_ys = [origin_y + MISSILE_HEIGHT as i16, origin_y + (MISSILE_HEIGHT + MISSILE_TAIL_SIZE) as i16, origin_y + MISSILE_HEIGHT as i16];
            }
        }
        return MissileTail {
            top_triangle_x: top_triangle_xs,
            top_triangle_y: top_triangle_ys,
            bot_triangle_x: top_triangle_xs.clone(), // same coordinates
            bot_triangle_y: bot_triangle_ys,
            color: MISSILE_TAIL_COLOR,
        };
    }

    fn move_towards(&mut self, direction: &MissileDirection){
        match direction {
            MissileDirection::LEFT => {
                self.top_triangle_x = self.top_triangle_x.map(|v| v - MISSILE_SPEED as i16);
                self.bot_triangle_x = self.bot_triangle_x.map(|v| v - MISSILE_SPEED as i16);
            }
            MissileDirection::RIGHT => {
                self.top_triangle_x = self.top_triangle_x.map(|v| v + MISSILE_SPEED as i16);
                self.bot_triangle_x = self.bot_triangle_x.map(|v| v + MISSILE_SPEED as i16);
            }
        }
    }
}

impl MissileHead {
    pub fn new(origin_x: i16, origin_y: i16, direction: &MissileDirection) -> MissileHead {
        let triangle_xs: [i16;3];
        let triangle_ys: [i16;3];

        match direction {
            MissileDirection::LEFT => {
                triangle_xs = [origin_x, origin_x - MISSILE_HEAD_SIZE as i16, origin_x];
                triangle_ys = [origin_y, origin_y + MISSILE_HEAD_SIZE as i16 / 2, origin_y + MISSILE_HEIGHT as i16];
            }
            MissileDirection::RIGHT => {
                triangle_xs = [origin_x + MISSILE_WIDTH as i16, origin_x + (MISSILE_WIDTH + MISSILE_HEAD_SIZE) as i16, origin_x + MISSILE_WIDTH as i16];
                triangle_ys = [origin_y, origin_y + MISSILE_HEIGHT as i16 / 2, origin_y + MISSILE_HEIGHT as i16];

            }
        }
        return MissileHead {
            triangle_x: triangle_xs,
            triangle_y: triangle_ys,
            color: MISSILE_HEAD_COLOR,
        };
    }

    fn move_towards(&mut self, direction: &MissileDirection){
        match direction {
            MissileDirection::LEFT => {
                self.triangle_x = self.triangle_x.map(|v| v - MISSILE_SPEED as i16);
            }
            MissileDirection::RIGHT => {
                self.triangle_x = self.triangle_x.map(|v| v + MISSILE_SPEED as i16);
            }
        }
    }

}

impl Missile {
    pub fn new() -> Missile {

        let mut rng = rand::thread_rng();
        let pos_x = rng.gen_range(MISSILE_SPAWN_RANGE_X) as i32;
        let pos_y = rng.gen_range(MISSILE_SPAWN_RANGE_Y) as i32;

        let d: MissileDirection;
        if random::<u8>() % 2 == 0 {
            d = MissileDirection::LEFT;
        }else {
            d = MissileDirection::RIGHT;
        }
        let r = Rect::new(pos_x, pos_y, MISSILE_WIDTH, MISSILE_HEIGHT);
        let m = MissileBody::new(r);
        let t = MissileTail::new(pos_x as i16, pos_y as i16, &d);
        let h = MissileHead::new(pos_x as i16, pos_y as i16, &d);
        
        return Missile {
            x: pos_x,
            y: pos_y,
            direction: d,
            body: m,
            tail: t,
            head: h,
        };
    }

    pub fn update(&mut self) {
        self.body.move_towards(&self.direction);
        self.head.move_towards(&self.direction);
        self.tail.move_towards(&self.direction);
    }
}
