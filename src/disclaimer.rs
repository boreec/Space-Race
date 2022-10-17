use sdl2::pixels::Color;

use std::time::Duration;

pub fn show_disclaimer(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>
){
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(3, 0));
}
