use crate::constants::{FIELD_WIDTH, FIELD_X_OFFSET, TILE_SIZE};
use macroquad::color::Color;
use macroquad::prelude::draw_text;
use macroquad::text::measure_text;

fn get_window_width() -> f32 {
    FIELD_X_OFFSET * 2.0 + FIELD_WIDTH as f32 * TILE_SIZE
}

pub fn draw_text_center(text: &str, y: f32, font_size: u16, color: Color) {
    let dims = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        (get_window_width() - dims.width) / 2.0,
        y,
        font_size as f32,
        color,
    );
}
