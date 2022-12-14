use std::time::Duration;
use std::time::Instant;

use crate::missile::*;
use crate::spaceship::*;

/// A **GameState** struct represents the state of the game
/// at any point during the execution. This struct contains
/// many attributes that handle the game entities like **Missile**
/// and **Spaceships**. It also handle the scores, the game duration,
/// and others.
pub struct GameState {
    pub missiles: Vec<Missile>,
    pub spaceship_p1: Spaceship,
    pub spaceship_p2: Spaceship,
    pub is_game_over: bool,
    pub is_game_restarted: bool,
    pub score_p1: u32,
    pub score_p2: u32,
    pub starting_time: Instant,
    pub game_duration: Duration,
}

impl GameState {
    pub fn new(m_quantity: usize, g_duration: Duration) -> GameState {
        let mut random_missiles = Vec::new();

        for _ in 0..m_quantity {
            random_missiles.push(Missile::new());
        }

        GameState {
            is_game_over: false,
            is_game_restarted: true,
            missiles: random_missiles,
            spaceship_p1: Spaceship::new(SPACESHIP_P1_X, SPACESHIP_P1_Y),
            spaceship_p2: Spaceship::new(SPACESHIP_P2_X, SPACESHIP_P2_Y),
            score_p1: 0,
            score_p2: 0,
            starting_time: Instant::now(),
            game_duration: g_duration,
        }
    }

    pub fn has_spaceship_scored(spaceship: &Spaceship) -> bool {
        spaceship.body.rect.y() + spaceship.body.rect.height() as i32 + SPACESHIP_TAIL_SIZE as i32
            <= 0
    }

    pub fn reset_spaceship_p1(&mut self) {
        self.spaceship_p1 = Spaceship::new(SPACESHIP_P1_X, SPACESHIP_P1_Y);
    }

    pub fn reset_spaceship_p2(&mut self) {
        self.spaceship_p2 = Spaceship::new(SPACESHIP_P2_X, SPACESHIP_P2_Y);
    }

    pub fn is_game_elapsed(&self) -> bool {
        self.starting_time.elapsed().as_secs() > self.game_duration.as_secs()
    }

    pub fn collision_occurred_for(&self, spaceship: &Spaceship) -> bool {
        for m in &self.missiles {
            if spaceship.collide_with(m) {
                return true;
            }
        }
        false
    }
}
