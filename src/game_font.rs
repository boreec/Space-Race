use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;

use std::path::Path;

pub struct GameFont<'a> {
    pub context: Sdl2TtfContext,
    pub poetsen_path: Box<&'a Path>,
    pub schluber_path: Box<&'a Path>,
}

impl GameFont<'_> {
    pub fn new() -> GameFont<'static> {

        let ct = sdl2::ttf::init().
            unwrap_or_else(|_| panic!("Failed to initialize SDL_TTF!"));
    
        GameFont {
            context: ct,
            poetsen_path: Box::new(Path::new("asset/font/poetsen_one/PoetsenOne-Regular.ttf")),
            schluber_path: Box::new(Path::new("asset/font/schluber/Schluber.ttf")),
        }
    }

    pub fn get_font(&self, path: &Path, point_size: u16) -> Font {
        self.context
            .load_font(path, point_size)
            .unwrap_or_else(|_| panic!("Failed to load font {}", path.display()))
    }
    
    pub fn surface_from_str<'a>(text: &str, font: Font<'a, 'a>, color: Color) -> Surface<'a> {
        font
            .render(text)
            .blended(color)
            .unwrap_or_else(|_| panic!("Failed to create surface from str {}", text))
    }
} 

