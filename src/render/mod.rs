mod chunk;
mod player;

use macroquad::color;
use macroquad::prelude::{draw_line, screen_height, screen_width};
pub use chunk::*;
pub use player::*;

pub fn draw_crosshair() {
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    const SIZE: f32 = 10.0;
    const WIDTH: f32 = 2.0;
    draw_line(cx - SIZE, cy, cx + SIZE, cy, WIDTH, color::GREEN);
    draw_line(cx, cy - SIZE, cx, cy + SIZE, WIDTH, color::GREEN);
}