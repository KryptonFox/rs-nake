use crate::constants::*;
use crate::entities::apple::Apple;
use crate::entities::snake::{Direction, Snake};
use crate::state::{GameState, Scene};
use macroquad::prelude::*;

pub struct Game {
    snake: Snake,
    apple: Apple,
    move_duration: f64,
    last_move: f64,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self::default();
        game.apple.regen(&game.snake);
        game
    }
    pub fn update(&mut self, game_state: &mut GameState) {
        Self::render_field();
        draw_text(
            format!("speed: {:.2}", 1.0 / self.move_duration / 5.55).as_str(),
            280.0,
            20.0,
            20.0,
            BLACK,
        );
        // keyboard input
        {
            if is_key_pressed(KeyCode::Up) {
                self.snake.change_direction(Direction::Up);
            }
            if is_key_pressed(KeyCode::Down) {
                self.snake.change_direction(Direction::Down);
            }
            if is_key_pressed(KeyCode::Left) {
                self.snake.change_direction(Direction::Left);
            }
            if is_key_pressed(KeyCode::Right) {
                self.snake.change_direction(Direction::Right);
            }
        }
        // move snake
        if get_time() - self.last_move > self.move_duration {
            self.last_move = get_time();
            if !self.snake.move_snake() {
                game_state.set_scene(Scene::GameOver);
                game_state.gameover = true;
            }

            // feed snake
            if self.snake.head == self.apple.position {
                self.snake.feed();
                self.apple.regen(&self.snake);
                game_state.apples += 1;
                self.move_duration *= 0.98;
            }
        }
        // render snake
        self.snake.render();
        // render apple
        self.apple.render();
    }

    fn render_field() {
        // render field lines
        // up line
        draw_line(
            FIELD_X_OFFSET,
            FIELD_Y_OFFSET - 1.0,
            FIELD_X_OFFSET + FIELD_WIDTH as f32 * TILE_SIZE,
            FIELD_Y_OFFSET - 1.0,
            1.0,
            BLACK,
        );
        // left line
        draw_line(
            FIELD_X_OFFSET - 1.0,
            FIELD_Y_OFFSET,
            FIELD_X_OFFSET - 1.0,
            FIELD_Y_OFFSET + FIELD_HEIGHT as f32 * TILE_SIZE,
            1.0,
            BLACK,
        );
        // down line
        draw_line(
            FIELD_X_OFFSET,
            FIELD_Y_OFFSET + FIELD_HEIGHT as f32 * TILE_SIZE + 1.0,
            FIELD_X_OFFSET + FIELD_WIDTH as f32 * TILE_SIZE,
            FIELD_Y_OFFSET + FIELD_HEIGHT as f32 * TILE_SIZE + 1.0,
            1.0,
            BLACK,
        );
        // right line
        draw_line(
            FIELD_X_OFFSET + FIELD_WIDTH as f32 * TILE_SIZE + 1.0,
            FIELD_Y_OFFSET,
            FIELD_X_OFFSET + FIELD_WIDTH as f32 * TILE_SIZE + 1.0,
            FIELD_Y_OFFSET + FIELD_HEIGHT as f32 * TILE_SIZE,
            1.0,
            BLACK,
        );
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            snake: Snake::new(),
            apple: Apple::new(),
            move_duration: 0.18,
            last_move: get_time(),
        }
    }
}
