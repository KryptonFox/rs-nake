use crate::state::{GameState, Scene};
use macroquad::prelude::*;
use crate::utils::draw_text_center;

pub struct Menu;

impl Menu {
    pub fn new() -> Menu {
        Menu
    }

    pub fn update(&self, game_state: &mut GameState) {
        draw_text_center("Welcome to RS-nake v0.2", 180.0, 30, GRAY);
        draw_text_center("Press enter to start", 200.0, 30, GRAY);
        if is_key_pressed(KeyCode::Enter) {
            game_state.set_scene(Scene::Game);
        }
    }
}
