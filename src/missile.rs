use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

use rand::random;
use rand::Rng;

use std::ops::RangeInclusive;

use crate::ToPoints;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

/* MISSILE CONSTANTS */
const MISSILE_HEIGHT: u32 = 5;
const MISSILE_WIDTH: u32 = 10;
const MISSILE_SPEED: i64 = 2;
const MISSILE_TAIL_SIZE: u32 = MISSILE_HEIGHT;
const MISSILE_HEAD_SIZE: u32 = MISSILE_HEIGHT;
const MISSILE_BODY_COLOR: Color = Color::WHITE;
const MISSILE_TAIL_COLOR: Color = Color::GREY;
const MISSILE_HEAD_COLOR: Color = Color::RED;
const MISSILE_SPAWN_RANGE_X: RangeInclusive<u32> = 0..=WINDOW_WIDTH;
const MISSILE_SPAWN_RANGE_Y: RangeInclusive<u32> = 0..=(WINDOW_HEIGHT - 200);

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
    pub top_triangle_x: [i16; 3],
    pub top_triangle_y: [i16; 3],
    pub bot_triangle_x: [i16; 3],
    pub bot_triangle_y: [i16; 3],
    pub color: Color,
}

pub struct MissileHead {
    pub triangle_x: [i16; 3],
    pub triangle_y: [i16; 3],
    pub color: Color,
}

impl MissileBody {
    fn new(r: Rect) -> MissileBody {
        return MissileBody {
            rect: r,
            color: MISSILE_BODY_COLOR,
        };
    }

    fn move_towards(&mut self, direction: &MissileDirection) {
        match direction {
            MissileDirection::LEFT => {
                self.rect.set_x(self.rect.x() - MISSILE_SPEED as i32);
            }
            MissileDirection::RIGHT => {
                self.rect.set_x(self.rect.x() + MISSILE_SPEED as i32);
            }
        }
    }
}

impl MissileTail {
    pub fn new(origin_x: i16, origin_y: i16, direction: &MissileDirection) -> MissileTail {
        let top_triangle_xs: [i16; 3];
        let top_triangle_ys: [i16; 3];
        let bot_triangle_ys: [i16; 3];
        match direction {
            MissileDirection::LEFT => {
                top_triangle_xs = [
                    origin_x + (MISSILE_WIDTH - MISSILE_TAIL_SIZE) as i16,
                    origin_x + MISSILE_WIDTH as i16,
                    origin_x + MISSILE_WIDTH as i16,
                ];
                top_triangle_ys = [origin_y, origin_y, origin_y - MISSILE_TAIL_SIZE as i16];
                bot_triangle_ys = [
                    origin_y + MISSILE_HEIGHT as i16,
                    origin_y + MISSILE_HEIGHT as i16,
                    origin_y + (MISSILE_HEIGHT + MISSILE_TAIL_SIZE) as i16,
                ];
            }
            MissileDirection::RIGHT => {
                top_triangle_xs = [origin_x, origin_x, origin_x + MISSILE_TAIL_SIZE as i16];
                top_triangle_ys = [origin_y, origin_y - MISSILE_TAIL_SIZE as i16, origin_y];
                bot_triangle_ys = [
                    origin_y + MISSILE_HEIGHT as i16,
                    origin_y + (MISSILE_HEIGHT + MISSILE_TAIL_SIZE) as i16,
                    origin_y + MISSILE_HEIGHT as i16,
                ];
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

    fn move_towards(&mut self, direction: &MissileDirection) {
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
        let triangle_xs: [i16; 3];
        let triangle_ys: [i16; 3];

        match direction {
            MissileDirection::LEFT => {
                triangle_xs = [origin_x, origin_x - MISSILE_HEAD_SIZE as i16, origin_x];
                triangle_ys = [
                    origin_y,
                    origin_y + MISSILE_HEAD_SIZE as i16 / 2,
                    origin_y + MISSILE_HEIGHT as i16,
                ];
            }
            MissileDirection::RIGHT => {
                triangle_xs = [
                    origin_x + MISSILE_WIDTH as i16,
                    origin_x + (MISSILE_WIDTH + MISSILE_HEAD_SIZE) as i16,
                    origin_x + MISSILE_WIDTH as i16,
                ];
                triangle_ys = [
                    origin_y,
                    origin_y + MISSILE_HEIGHT as i16 / 2,
                    origin_y + MISSILE_HEIGHT as i16,
                ];
            }
        }
        return MissileHead {
            triangle_x: triangle_xs,
            triangle_y: triangle_ys,
            color: MISSILE_HEAD_COLOR,
        };
    }

    fn move_towards(&mut self, direction: &MissileDirection) {
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

impl ToPoints for Missile {
    fn points(&self) -> Vec<Point> {
        let mut result = Vec::new();
        result.append(&mut self.head.points());
        result.append(&mut self.body.points());
        result.append(&mut self.tail.points());
        return result;
    }
}

impl ToPoints for MissileHead {
    fn points(&self) -> Vec<Point> {
        return vec![
            Point::new(self.triangle_x[0] as i32, self.triangle_y[0] as i32),
            Point::new(self.triangle_x[1] as i32, self.triangle_y[1] as i32),
            Point::new(self.triangle_x[2] as i32, self.triangle_y[2] as i32),
        ];
    }
}

impl ToPoints for MissileBody {
    fn points(&self) -> Vec<Point> {
        return vec![
            Point::new(self.rect.x(), self.rect.y()),
            Point::new(self.rect.x() + self.rect.width() as i32, self.rect.y()),
            Point::new(self.rect.x(), self.rect.y() + self.rect.height() as i32),
            Point::new(
                self.rect.x() + self.rect.width() as i32,
                self.rect.y() + self.rect.height() as i32,
            ),
        ];
    }
}

impl ToPoints for MissileTail {
    fn points(&self) -> Vec<Point> {
        return vec![
            Point::new(self.top_triangle_x[0] as i32, self.top_triangle_y[0] as i32),
            Point::new(self.bot_triangle_x[1] as i32, self.top_triangle_y[1] as i32),
            Point::new(self.bot_triangle_x[2] as i32, self.top_triangle_y[2] as i32),
            Point::new(self.bot_triangle_x[0] as i32, self.bot_triangle_y[0] as i32),
            Point::new(self.bot_triangle_x[1] as i32, self.bot_triangle_y[1] as i32),
            Point::new(self.bot_triangle_x[2] as i32, self.bot_triangle_y[2] as i32),
        ];
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
        } else {
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
        // Check if the missile is going out of the screen.
        self.check_screen_wrapping();

        // Update Missile and its components coordinates according to its direction.
        self.move_towards();
        self.body.move_towards(&self.direction);
        self.head.move_towards(&self.direction);
        self.tail.move_towards(&self.direction);
    }

    fn check_screen_wrapping(&mut self) {
        if self.direction == MissileDirection::LEFT && self.x < 0 {
            self.x = WINDOW_WIDTH as i32;
            let r = Rect::new(self.x, self.y, MISSILE_WIDTH, MISSILE_HEIGHT);
            self.body = MissileBody::new(r);
            self.tail = MissileTail::new(self.x as i16, self.y as i16, &self.direction);
            self.head = MissileHead::new(self.x as i16, self.y as i16, &self.direction);
        } else if self.direction == MissileDirection::RIGHT && self.x > WINDOW_WIDTH as i32 {
            self.x = 0;
            let r = Rect::new(self.x, self.y, MISSILE_WIDTH, MISSILE_HEIGHT);
            self.body = MissileBody::new(r);
            self.tail = MissileTail::new(self.x as i16, self.y as i16, &self.direction);
            self.head = MissileHead::new(self.x as i16, self.y as i16, &self.direction);
        }
    }

    fn move_towards(&mut self) {
        match self.direction {
            MissileDirection::LEFT => {
                self.x -= MISSILE_SPEED as i32;
            }
            MissileDirection::RIGHT => {
                self.x += MISSILE_SPEED as i32;
            }
        }
    }
}
