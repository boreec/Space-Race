use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

use rand::random;
use rand::Rng;

use std::ops::RangeInclusive;

use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

/// The height of a **MissileBody** rectangle (in pixels).
const MISSILE_HEIGHT: u32 = 5;

/// The width of a **MissileBody** rectangle (in pixels).
const MISSILE_WIDTH: u32 = 10;

/// The speed of **Missile** entity and its components (in pixels).
const MISSILE_SPEED: i64 = 2;

/// The size used to build the triangle representing the **MissileHead**.
/// It corresponds to the height of the triangle. For the sake of aesthetics,
/// The value is the same as **MISSILE_HEIGHT** and **MISSILE_TAIL_SIZE**.
const MISSILE_HEAD_SIZE: u32 = MISSILE_HEIGHT;

/// The size used to build the triangles representing the **MissileTail**.
/// It corresponds to the height of the triangles. For the sake of aesthetics,
/// The value is the same as **MISSILE_HEIGHT** and **MISSILE_HEAD_SIZE**. 
const MISSILE_TAIL_SIZE: u32 = MISSILE_HEIGHT;

/// The color of the triangle representing the **MissileHead**.
const MISSILE_HEAD_COLOR: Color = Color::RED;

/// The color of the rectangle representing the **MissileBody**.
const MISSILE_BODY_COLOR: Color = Color::WHITE;

/// The color of the triangles representing the **MissileTail**. 
const MISSILE_TAIL_COLOR: Color = Color::GREY;

/// The X axis range where a **Missile** can spawn.
/// It's basically the whole screen's width.
const MISSILE_SPAWN_RANGE_X: RangeInclusive<u32> = 0..=WINDOW_WIDTH;

/// The Y axis range where a **Missile** can spawn.
/// It's from the top of the screen to the **Spaceship** area,
/// in order to let them spawn without killing them instantly.
const MISSILE_SPAWN_RANGE_Y: RangeInclusive<u32> = 0..=(WINDOW_HEIGHT - 200);

/// Define the main direction of a missile and its components.
/// A missile can only move in two directions: to the *left* or to the *right*.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MissileDirection {
    Left,
    Right,
}

/// The **Missile** entity that must be avoided in order to win 
/// the game. The struct is comprised three substruct: 
/// **MissileHead**, **MissileBody** and **MissileTail**.
pub struct Missile {
    pub x: i32,
    pub y: i32,
    pub body: MissileBody,
    pub tail: MissileTail,
    pub head: MissileHead,
    pub direction: MissileDirection,
}

/// The **MissileBody** struct represents the main part of a **Missile**.
/// It is made of a single rectangular shape.
pub struct MissileBody {
    pub rect: Rect,
    pub color: Color,
}

/// The **MissileTail** struct represents the side opposite to the **Missile**'s
/// direction. It is made of two triangles on each sides of the **MissileBody**.
pub struct MissileTail {
    pub top_triangle_x: [i16; 3],
    pub top_triangle_y: [i16; 3],
    pub bot_triangle_x: [i16; 3],
    pub bot_triangle_y: [i16; 3],
    pub color: Color,
}

/// The **MissileHead** struct is the *top* of the **Missile**. Its orientation is
/// the same as the **Missile**'s direction. It is made of a single triangle pointing
/// toward the **Missile**'s direction.
pub struct MissileHead {
    pub triangle_x: [i16; 3],
    pub triangle_y: [i16; 3],
    pub color: Color,
}

impl Missile {
    pub fn new() -> Missile {
        let mut rng = rand::thread_rng();
        let pos_x = rng.gen_range(MISSILE_SPAWN_RANGE_X) as i32;
        let pos_y = rng.gen_range(MISSILE_SPAWN_RANGE_Y) as i32;

        let d: MissileDirection = if random::<u8>() % 2 == 0 {
            MissileDirection::Left
        } else {
            MissileDirection::Right
        };
        let r = Rect::new(pos_x, pos_y, MISSILE_WIDTH, MISSILE_HEIGHT);
        let m = MissileBody::new(r);
        let t = MissileTail::new(pos_x as i16, pos_y as i16, &d);
        let h = MissileHead::new(pos_x as i16, pos_y as i16, &d);

        Missile {
            x: pos_x,
            y: pos_y,
            direction: d,
            body: m,
            tail: t,
            head: h,
        }
    }

    pub fn update(&mut self) {
        // Check if the missile is going out of the screen.
        self.check_screen_wrapping();

        // Update Missile and its components coordinates according to its direction.
        self.move_toward(&self.direction.clone());
        self.body.move_toward(&self.direction);
        self.head.move_toward(&self.direction);
        self.tail.move_toward(&self.direction);
    }

    fn check_screen_wrapping(&mut self) {
        if self.direction == MissileDirection::Left && self.x < 0 {
            self.x = WINDOW_WIDTH as i32;
            let r = Rect::new(self.x, self.y, MISSILE_WIDTH, MISSILE_HEIGHT);
            self.body = MissileBody::new(r);
            self.tail = MissileTail::new(self.x as i16, self.y as i16, &self.direction);
            self.head = MissileHead::new(self.x as i16, self.y as i16, &self.direction);
        } else if self.direction == MissileDirection::Right && self.x > WINDOW_WIDTH as i32 {
            self.x = 0;
            let r = Rect::new(self.x, self.y, MISSILE_WIDTH, MISSILE_HEIGHT);
            self.body = MissileBody::new(r);
            self.tail = MissileTail::new(self.x as i16, self.y as i16, &self.direction);
            self.head = MissileHead::new(self.x as i16, self.y as i16, &self.direction);
        }
    }
}

impl MissileBody {
    fn new(r: Rect) -> MissileBody {
        MissileBody {
            rect: r,
            color: MISSILE_BODY_COLOR,
        }
    }
}

impl MissileTail {
    pub fn new(origin_x: i16, origin_y: i16, direction: &MissileDirection) -> MissileTail {
        let top_triangle_xs: [i16; 3];
        let top_triangle_ys: [i16; 3];
        let bot_triangle_ys: [i16; 3];
        match direction {
            MissileDirection::Left => {
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
            MissileDirection::Right => {
                top_triangle_xs = [origin_x, origin_x, origin_x + MISSILE_TAIL_SIZE as i16];
                top_triangle_ys = [origin_y, origin_y - MISSILE_TAIL_SIZE as i16, origin_y];
                bot_triangle_ys = [
                    origin_y + MISSILE_HEIGHT as i16,
                    origin_y + (MISSILE_HEIGHT + MISSILE_TAIL_SIZE) as i16,
                    origin_y + MISSILE_HEIGHT as i16,
                ];
            }
        }
        MissileTail {
            top_triangle_x: top_triangle_xs,
            top_triangle_y: top_triangle_ys,
            bot_triangle_x: top_triangle_xs, // same coordinates
            bot_triangle_y: bot_triangle_ys,
            color: MISSILE_TAIL_COLOR,
        }
    }
}

impl MissileHead {
    pub fn new(origin_x: i16, origin_y: i16, direction: &MissileDirection) -> MissileHead {
        let triangle_xs: [i16; 3];
        let triangle_ys: [i16; 3];

        match direction {
            MissileDirection::Left => {
                triangle_xs = [origin_x, origin_x - MISSILE_HEAD_SIZE as i16, origin_x];
                triangle_ys = [
                    origin_y,
                    origin_y + MISSILE_HEAD_SIZE as i16 / 2,
                    origin_y + MISSILE_HEIGHT as i16,
                ];
            }
            MissileDirection::Right => {
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
        MissileHead {
            triangle_x: triangle_xs,
            triangle_y: triangle_ys,
            color: MISSILE_HEAD_COLOR,
        }
    }
}

/// Trait used to return a **Missile** and its components as a vector of points.
/// The points can then be used to compute the collisions with another object, like
/// a **Spaceship**.
pub trait ToPoints {
    fn points(&self) -> Vec<Point>;
}

impl ToPoints for Missile {
    fn points(&self) -> Vec<Point> {
        let mut result = Vec::new();
        result.append(&mut self.head.points());
        result.append(&mut self.body.points());
        result.append(&mut self.tail.points());
        result
    }
}

impl ToPoints for MissileHead {
    fn points(&self) -> Vec<Point> {
        vec![
            Point::new(self.triangle_x[0] as i32, self.triangle_y[0] as i32),
            Point::new(self.triangle_x[1] as i32, self.triangle_y[1] as i32),
            Point::new(self.triangle_x[2] as i32, self.triangle_y[2] as i32),
        ]
    }
}

impl ToPoints for MissileBody {
    fn points(&self) -> Vec<Point> {
        vec![
            Point::new(self.rect.x(), self.rect.y()),
            Point::new(self.rect.x() + self.rect.width() as i32, self.rect.y()),
            Point::new(self.rect.x(), self.rect.y() + self.rect.height() as i32),
            Point::new(
                self.rect.x() + self.rect.width() as i32,
                self.rect.y() + self.rect.height() as i32,
            ),
        ]
    }
}

impl ToPoints for MissileTail {
    fn points(&self) -> Vec<Point> {
        vec![
            Point::new(self.top_triangle_x[0] as i32, self.top_triangle_y[0] as i32),
            Point::new(self.bot_triangle_x[1] as i32, self.top_triangle_y[1] as i32),
            Point::new(self.bot_triangle_x[2] as i32, self.top_triangle_y[2] as i32),
            Point::new(self.bot_triangle_x[0] as i32, self.bot_triangle_y[0] as i32),
            Point::new(self.bot_triangle_x[1] as i32, self.bot_triangle_y[1] as i32),
            Point::new(self.bot_triangle_x[2] as i32, self.bot_triangle_y[2] as i32),
        ]
    }
}

/// Define the movements a **Missile** and its components can make.
/// The movements depends on the values defined by **MissileDirection** enum.
pub trait MissileMovement {
    fn move_toward(&mut self, direction: &MissileDirection) -> ();   
}

impl MissileMovement for Missile {
    fn move_toward(&mut self, direction: &MissileDirection) {
        match direction {
            MissileDirection::Left => {
                self.x -= MISSILE_SPEED as i32;
            }
            MissileDirection::Right => {
                self.x += MISSILE_SPEED as i32;
            }
        }
    }    
}

impl MissileMovement for MissileHead {
    fn move_toward(&mut self, direction: &MissileDirection) {
        match direction {
            MissileDirection::Left => {
                self.triangle_x = self.triangle_x.map(|v| v - MISSILE_SPEED as i16);
            }
            MissileDirection::Right => {
                self.triangle_x = self.triangle_x.map(|v| v + MISSILE_SPEED as i16);
            }
        }
    }
}

impl MissileMovement for MissileBody {
    fn move_toward(&mut self, direction: &MissileDirection) {
        match direction {
            MissileDirection::Left => {
                self.rect.set_x(self.rect.x() - MISSILE_SPEED as i32);
            }
            MissileDirection::Right => {
                self.rect.set_x(self.rect.x() + MISSILE_SPEED as i32);
            }
        }
    }    
}

impl MissileMovement for MissileTail {
    fn move_toward(&mut self, direction: &MissileDirection) {
        match direction {
            MissileDirection::Left => {
                self.top_triangle_x = self.top_triangle_x.map(|v| v - MISSILE_SPEED as i16);
                self.bot_triangle_x = self.bot_triangle_x.map(|v| v - MISSILE_SPEED as i16);
            }
            MissileDirection::Right => {
                self.top_triangle_x = self.top_triangle_x.map(|v| v + MISSILE_SPEED as i16);
                self.bot_triangle_x = self.bot_triangle_x.map(|v| v + MISSILE_SPEED as i16);
            }
        }
    }    
}
