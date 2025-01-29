use crate::constants::*;
use crate::entities::snake::Snake;
use macroquad::prelude::{rand::gen_range, *};

pub struct Apple {
    pub position: Vec2,
}
impl Apple {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(-10.0, -10.0),
        }
    }

    pub fn regen(&mut self, snake: &Snake) {
        let mut exclude = snake.body.clone();
        exclude.push(snake.head);
        self.position = Self::gen_pos_exclude(exclude);
    }

    pub fn render(&self) {
        draw_rectangle(
            FIELD_X_OFFSET + &self.position.x * TILE_SIZE,
            FIELD_Y_OFFSET + &self.position.y * TILE_SIZE,
            TILE_SIZE,
            TILE_SIZE,
            RED,
        );
    }

    fn gen_pos() -> Vec2 {
        Vec2::new(
            gen_range(0, FIELD_WIDTH) as f32,
            gen_range(0, FIELD_HEIGHT) as f32,
        )
    }

    fn gen_pos_exclude(exclude: Vec<Vec2>) -> Vec2 {
        let mut pos = Self::gen_pos();
        while exclude.contains(&pos) {
            pos = Self::gen_pos();
        }
        pos
    }
}
