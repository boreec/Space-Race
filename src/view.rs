use sdl2::pixels::Color;

const COLOR_BACKGROUND: Color = Color::BLACK;

fn draw_background(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(COLOR_BACKGROUND);
    canvas.clear();

}

pub fn draw_game(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    draw_background(canvas);
    canvas.present();

}
