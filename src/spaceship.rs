use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::Duration;
use std::time::Instant;

use crate::missile::*;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

const SPACESHIP_SPEED: i64 = 8;
const SPACESHIP_BODY_WIDTH: u32 = 25;
const SPACESHIP_BODY_HEIGHT: u32 = 60;
const SPACESHIP_BODY_COLOR: Color = Color::WHITE;
const SPACESHIP_PORTHOLE_COLOR: Color = Color::BLUE;
const SPACESHIP_HEAD_SIZE: u32 = SPACESHIP_BODY_WIDTH;
const SPACESHIP_HEAD_COLOR: Color = Color::RED;
pub const SPACESHIP_TAIL_SIZE: i16 = (SPACESHIP_BODY_WIDTH / 4 * 3) as i16;
const SPACESHIP_TAIL_COLOR: Color = Color::RED;
pub const SPACESHIP_P1_X: i32 = (WINDOW_WIDTH / 4 - SPACESHIP_BODY_WIDTH / 2) as i32;
pub const SPACESHIP_P1_Y: i32 = (WINDOW_HEIGHT - SPACESHIP_BODY_HEIGHT) as i32;
pub const SPACESHIP_P2_X: i32 = SPACESHIP_P1_X + (WINDOW_WIDTH / 2) as i32;
pub const SPACESHIP_P2_Y: i32 = SPACESHIP_P1_Y;
const SPACESHIP_DEATH_TIME: Duration = Duration::new(1, 0);

pub struct Spaceship {
    pub body: SpaceshipBody,
    pub head: SpaceshipHead,
    pub tail: SpaceshipTail,
    pub is_alive: bool,
    pub death_instant: Option<Instant>,
}

impl Spaceship {
    pub fn new(pos_x: i32, pos_y: i32) -> Spaceship {
        Spaceship {
            body: SpaceshipBody::new(pos_x, pos_y, SPACESHIP_BODY_WIDTH, SPACESHIP_BODY_HEIGHT),
            head: SpaceshipHead::new(pos_x as i16, pos_y as i16),
            tail: SpaceshipTail::new(pos_x as i16, pos_y as i16),
            is_alive: true,
            death_instant: Some(Instant::now()),
        }
    }

    pub fn die(&mut self) {
        self.is_alive = false;
        self.death_instant = Some(Instant::now());
    }

    pub fn move_upward(&mut self) {
        self.body.move_upward();
        self.head.move_upward();
        self.tail.move_upward();
    }

    pub fn move_downward(&mut self) {
        self.body.move_downward();
        self.head.move_downward();
        self.tail.move_downward();
    }

    pub fn can_move_downward(&self) -> bool {
        SPACESHIP_HEAD_SIZE + self.body.rect.y() as u32 + 2 * SPACESHIP_TAIL_SIZE as u32
            <= WINDOW_HEIGHT
    }

    pub fn can_respawn(&self) -> bool {
        !self.is_alive
            && self
                .death_instant
                .expect("no last death registered!")
                .elapsed()
                .as_secs()
                > SPACESHIP_DEATH_TIME.as_secs()
    }

    pub fn collide_with(&self, missile: &Missile) -> bool {
        let mut head_collision = false;
        let mut body_collision = false;
        let mut tail_collision = false;

        let m_points = missile.points();
        let mut h_i = 0;
        while !head_collision && !body_collision && !tail_collision && h_i < m_points.len() {
            head_collision = self.head.is_point_within(m_points[h_i].x, m_points[h_i].y);
            body_collision = self.body.is_point_within(m_points[h_i].x, m_points[h_i].y);
            tail_collision = self.tail.is_point_within(m_points[h_i].x, m_points[h_i].y);
            h_i += 1;
        }

        head_collision || body_collision || tail_collision
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

        SpaceshipBody {
            rect: Rect::new(pos_x, pos_y, width, height),
            body_color: SPACESHIP_BODY_COLOR,
            porthole_1: (
                pos_x as i16 + width as i16 / 2,
                pos_y as i16 + porthole_radius * 2,
            ),
            porthole_2: (
                pos_x as i16 + width as i16 / 2,
                pos_y as i16 + porthole_radius * 5,
            ),
            porthole_r: porthole_radius,
            porthole_color: SPACESHIP_PORTHOLE_COLOR,
        }
    }

    pub fn move_upward(&mut self) {
        self.rect.set_y(self.rect.y() - SPACESHIP_SPEED as i32);
        self.porthole_1.1 -= SPACESHIP_SPEED as i16;
        self.porthole_2.1 -= SPACESHIP_SPEED as i16;
    }

    pub fn move_downward(&mut self) {
        self.rect.set_y(self.rect.y() + SPACESHIP_SPEED as i32);
        self.porthole_1.1 += SPACESHIP_SPEED as i16;
        self.porthole_2.1 += SPACESHIP_SPEED as i16;
    }

    pub fn is_point_within(&self, x: i32, y: i32) -> bool {
        (self.rect.x() <= x && self.rect.x() + self.rect.width() as i32 >= x)
            && (self.rect.y() <= y && self.rect.y() + self.rect.height() as i32 >= y)
    }
}

pub struct SpaceshipHead {
    pub triangle_x: [i16; 3],
    pub triangle_y: [i16; 3],
    pub color: Color,
}

impl SpaceshipHead {
    pub fn new(pos_x: i16, pos_y: i16) -> SpaceshipHead {
        SpaceshipHead {
            triangle_x: [
                pos_x,
                pos_x + (SPACESHIP_HEAD_SIZE / 2) as i16,
                pos_x + SPACESHIP_BODY_WIDTH as i16,
            ],
            triangle_y: [pos_y, pos_y - SPACESHIP_HEAD_SIZE as i16, pos_y],
            color: SPACESHIP_HEAD_COLOR,
        }
    }

    pub fn move_upward(&mut self) {
        self.triangle_y = self.triangle_y.map(|v| v - SPACESHIP_SPEED as i16);
    }

    pub fn move_downward(&mut self) {
        self.triangle_y = self.triangle_y.map(|v| v + SPACESHIP_SPEED as i16);
    }

    pub fn is_point_within(&self, x: i32, y: i32) -> bool {
        let area = |x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32| -> f64 {
            let ab = vec![x2 - x1, y2 - y1];
            let ac = vec![x3 - x1, y3 - y1];
            let cross_product = ab[0] * ac[1] - ab[1] * ac[0];
            (cross_product as f64 / 2.0).abs()
        };

        let is_point_within_triangle =
            |triangle_x: &[i16; 3], triangle_y: &[i16; 3], x: i32, y: i32| -> bool {
                let xs = [
                    triangle_x[0] as i32,
                    triangle_x[1] as i32,
                    triangle_x[2] as i32,
                ];
                let ys = [
                    triangle_y[0] as i32,
                    triangle_y[1] as i32,
                    triangle_y[2] as i32,
                ];

                let a = area(xs[0], ys[0], xs[1], ys[1], xs[2], ys[2]);
                let a1 = area(x, y, xs[1], ys[1], xs[2], ys[2]);
                let a2 = area(xs[0], ys[0], x, y, xs[2], ys[2]);
                let a3 = area(xs[1], ys[1], xs[2], ys[2], x, y);

                let min_x = xs.into_iter().reduce(i32::min).unwrap();
                let max_x = xs.into_iter().reduce(i32::max).unwrap();
                let min_y = ys.into_iter().reduce(i32::min).unwrap();
                let max_y = ys.into_iter().reduce(i32::max).unwrap();

                a == a1 + a2 + a3 && x > min_x && x < max_x && y > min_y && y < max_y
            };

        is_point_within_triangle(&self.triangle_x, &self.triangle_y, x, y)
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
        SpaceshipTail {
            left_triangle_x: [pos_x, pos_x, pos_x - SPACESHIP_TAIL_SIZE],
            left_triangle_y: [
                pos_y + SPACESHIP_BODY_HEIGHT as i16 - SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16 + SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16,
            ],
            right_triangle_x: [
                pos_x + SPACESHIP_BODY_WIDTH as i16,
                pos_x + SPACESHIP_BODY_WIDTH as i16,
                pos_x + SPACESHIP_BODY_WIDTH as i16 + SPACESHIP_TAIL_SIZE,
            ],
            right_triangle_y: [
                pos_y + SPACESHIP_BODY_HEIGHT as i16 - SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16 + SPACESHIP_TAIL_SIZE,
                pos_y + SPACESHIP_BODY_HEIGHT as i16,
            ],
            color: SPACESHIP_TAIL_COLOR,
        }
    }

    pub fn move_upward(&mut self) {
        self.left_triangle_y = self.left_triangle_y.map(|v| v - SPACESHIP_SPEED as i16);
        self.right_triangle_y = self.right_triangle_y.map(|v| v - SPACESHIP_SPEED as i16);
    }

    pub fn move_downward(&mut self) {
        self.left_triangle_y = self.left_triangle_y.map(|v| v + SPACESHIP_SPEED as i16);
        self.right_triangle_y = self.right_triangle_y.map(|v| v + SPACESHIP_SPEED as i16);
    }

    pub fn is_point_within(&self, x: i32, y: i32) -> bool {
        let area = |x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32| -> f64 {
            let ab = vec![x2 - x1, y2 - y1];
            let ac = vec![x3 - x1, y3 - y1];
            let cross_product = ab[0] * ac[1] - ab[1] * ac[0];
            (cross_product as f64 / 2.0).abs()
        };

        let is_point_within_triangle =
            |triangle_x: &[i16; 3], triangle_y: &[i16; 3], x: i32, y: i32| -> bool {
                let xs = [
                    triangle_x[0] as i32,
                    triangle_x[1] as i32,
                    triangle_x[2] as i32,
                ];
                let ys = [
                    triangle_y[0] as i32,
                    triangle_y[1] as i32,
                    triangle_y[2] as i32,
                ];

                let a = area(xs[0], ys[0], xs[1], ys[1], xs[2], ys[2]);
                let a1 = area(x, y, xs[1], ys[1], xs[2], ys[2]);
                let a2 = area(xs[0], ys[0], x, y, xs[2], ys[2]);
                let a3 = area(xs[1], ys[1], xs[2], ys[2], x, y);

                let min_x = xs.into_iter().reduce(i32::min).unwrap();
                let max_x = xs.into_iter().reduce(i32::max).unwrap();
                let min_y = ys.into_iter().reduce(i32::min).unwrap();
                let max_y = ys.into_iter().reduce(i32::max).unwrap();

                a == a1 + a2 + a3 && x > min_x && x < max_x && y > min_y && y < max_y
            };

        is_point_within_triangle(&self.left_triangle_x, &self.left_triangle_y, x, y)
            || is_point_within_triangle(&self.right_triangle_x, &self.right_triangle_y, x, y)
    }
}

fn triangle_area(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> f64 {
    let ab = vec![x2 - x1, y2 - y1];
    let ac = vec![x3 - x1, y3 - y1];
    let cross_product = ab[0] * ac[1] - ab[1] * ac[0];
    (cross_product as f64 / 2.0).abs()
}

fn is_point_within_triangle(triangle_x: &[i16; 3], triangle_y: &[i16; 3], x: i32, y: i32) -> bool {
    let xs = [
        triangle_x[0] as i32,
        triangle_x[1] as i32,
        triangle_x[2] as i32,
    ];
    let ys = [
        triangle_y[0] as i32,
        triangle_y[1] as i32,
        triangle_y[2] as i32,
    ];

    let a = triangle_area(xs[0], ys[0], xs[1], ys[1], xs[2], ys[2]);
    let a1 = triangle_area(x, y, xs[1], ys[1], xs[2], ys[2]);
    let a2 = triangle_area(xs[0], ys[0], x, y, xs[2], ys[2]);
    let a3 = triangle_area(xs[1], ys[1], xs[2], ys[2], x, y);

    let min_x = xs.into_iter().reduce(i32::min).unwrap();
    let max_x = xs.into_iter().reduce(i32::max).unwrap();
    let min_y = ys.into_iter().reduce(i32::min).unwrap();
    let max_y = ys.into_iter().reduce(i32::max).unwrap();

    a == a1 + a2 + a3 && x > min_x && x < max_x && y > min_y && y < max_y
}
