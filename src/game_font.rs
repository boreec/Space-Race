use sdl2::ttf::*;

pub struct GameFont {
    context: Sdl2TtfContext,
}

impl GameFont {
    pub fn new() -> GameFont {
        GameFont {
            context: sdl2::ttf::init().unwrap_or_else(|_| panic!("Failed to initialize SDL TTF")),
        }
    }
}
