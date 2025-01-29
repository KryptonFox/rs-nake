use crate::state::{GameState, Scene};
use crate::utils::draw_text_center;
use macroquad::prelude::*;

pub struct GameOver;

impl GameOver {
    pub fn new() -> Self {
        Self
    }
    pub fn update(&self, game_state: &mut GameState) {
        draw_text_center("Gameover", 150.0, 40, RED);
        draw_text_center("Press Enter to reset", 180.0, 30, GRAY);
        if is_key_pressed(KeyCode::Enter) {
            game_state.set_scene(Scene::Game);
            game_state.gameover = false;
        }
    }
}
