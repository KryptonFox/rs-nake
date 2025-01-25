use macroquad::math::Vec2;
use macroquad::rand::gen_range;
use crate::constants::{FIELD_HEIGHT, FIELD_WIDTH};
use crate::snake::Snake;

pub struct Apple {
    pub position : Vec2,
}
impl Apple {
    pub fn new() -> Self {
        Self {
            position : Vec2::new(-10.0, -10.0),
        }
    }

    pub fn regen(&mut self, snake: &Snake) {
        let mut exclude = snake.body.clone();
        exclude.push(snake.head);
        self.position = Self::gen_pos_exclude(exclude);
    }

    fn gen_pos() -> Vec2 {
        Vec2::new(gen_range(0, FIELD_WIDTH) as f32, gen_range(0, FIELD_HEIGHT) as f32)
    }

    fn gen_pos_exclude(exclude: Vec<Vec2>) -> Vec2 {
        let mut pos = Self::gen_pos();
        while exclude.contains(&pos) {
            pos = Self::gen_pos();
        }
        pos
    }
}