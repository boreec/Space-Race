use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;

use std::path::Path;

/// The **GameFont** struct contains the paths to the fonts
/// used in the game, especially for the disclaimer screen,
/// the scores  and the game over screen.
pub struct GameFont<'a> {
    pub context: Sdl2TtfContext,
    pub poetsen_path: &'a Path,
    pub schluber_path: &'a Path,
}

impl GameFont<'_> {
    pub fn new() -> GameFont<'static> {
        let ct = sdl2::ttf::init().unwrap_or_else(|_| panic!("Failed to initialize SDL_TTF!"));

        GameFont {
            context: ct,
            poetsen_path: Path::new("asset/font/poetsen_one/PoetsenOne-Regular.ttf"),
            schluber_path: Path::new("asset/font/schluber/Schluber.ttf"),
        }
    }

    pub fn get_font(&self, path: &Path, point_size: u16) -> Font {
        self.context
            .load_font(path, point_size)
            .unwrap_or_else(|_| panic!("Failed to load font {}", path.display()))
    }

    pub fn surface_from_str<'a>(
        &self,
        text: &str,
        font: &Font<'a, 'a>,
        color: Color,
    ) -> Surface<'a> {
        font.render(text)
            .blended(color)
            .unwrap_or_else(|_| panic!("Failed to create surface from str {}", text))
    }
}
