use sdl2::pixels::Color;

const MISSILE_HEIGHT: i32 = 5;
const MISSILE_WIDTH: i32 = 10;
const MISSILE_COLOR: Color = Color::WHITE;
const MISSILE_QUANTITY: usize = 10;

pub struct GameState {
    pub missiles: [Missile; MISSILE_QUANTITY],
    pub is_game_over: bool,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState {
            is_game_over: false,
            missiles: [Missile::new(); MISSILE_QUANTITY],
        };
    }
}

#[derive(Clone, Copy)]
pub struct Missile {
    x: i32,
    y: i32,
    height: i32,
    width: i32,
    color: Color,
}

impl Missile {
    pub fn new() -> Missile {
        return Missile {
            x: 0,
            y: 0,
            height: MISSILE_HEIGHT,
            width: MISSILE_WIDTH,
            color: MISSILE_COLOR,
        }
    }
}
