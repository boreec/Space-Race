use sdl2::ttf::*;

use std::path::Path;

pub struct GameFont<'a> {
    pub context: Sdl2TtfContext,
    pub poetsen_path: Box<&'a Path>,
}

impl GameFont<'_> {
    pub fn new() -> GameFont<'static> {

        let ct = sdl2::ttf::init().
            unwrap_or_else(|_| panic!("Failed to initialize SDL_TTF!"));
    
        GameFont {
            context: ct,
            poetsen_path: Box::new(Path::new("asset/font/poetsen_one/PoetsenOne-Regular.ttf")),
        }
    }

    pub fn get_font(&self, path: &Path, point_size: u16) -> Font {
        self.context
            .load_font(path, point_size)
            .unwrap_or_else(|_| panic!("Failed to load font {}", path.display()))
    }

} 

