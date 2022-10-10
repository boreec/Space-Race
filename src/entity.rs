pub struct GameState {
    pub is_game_over: bool,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState {
            is_game_over: false,
        };
    }
}
