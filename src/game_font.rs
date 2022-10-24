pub struct GameFont {
    context: Sdl2TtfContext,
}

impl GameFont {
    pub new() -> GameFont {
        GameFont {
            context: sdl2::ttf::init().unwrap_or_else('_' panic!("Failed to initialize SDL TTF")):,
        }
    }
}
