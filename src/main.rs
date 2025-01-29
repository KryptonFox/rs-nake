#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod constants;
mod entities;
mod scenes;
mod state;
mod utils;

use crate::scenes::{Game, GameOver, Menu};
use crate::state::{GameState, Scene};
use constants::*;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    // init game state
    let mut game_state = GameState::new();

    // init scenes
    let menu_scene = Menu::new();
    let mut game_scene = Game::new();
    let gameover_scene = GameOver::new();

    // main loop
    loop {
        clear_background(DARKGRAY);
        draw_text("RS-NAKE V0.2", 20.0, 20.0, 30.0, BLACK);
        draw_text(
            format!("fps: {:.1}", get_fps()).as_str(),
            400.0,
            20.0,
            20.0,
            BLACK,
        );
        // render current scene
        match game_state.scene {
            Scene::Menu => {
                menu_scene.update(&mut game_state);
            }
            Scene::Game => {
                game_scene.update(&mut game_state);
                if game_state.gameover {
                    game_scene = Game::new();
                }
            }
            Scene::GameOver => {
                gameover_scene.update(&mut game_state);
            }
        }
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "rs-nake ©KryptonFox".to_string(),
        window_width: (FIELD_X_OFFSET * 2.0 + FIELD_WIDTH as f32 * TILE_SIZE).round() as i32,
        window_height: (FIELD_Y_OFFSET * 2.0 + FIELD_HEIGHT as f32 * TILE_SIZE).round() as i32,
        window_resizable: false,
        ..Default::default()
    }
}
