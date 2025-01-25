mod apple;
mod constants;
mod snake;

use crate::apple::Apple;
use crate::snake::Snake;
use constants::*;
use macroquad::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut snake = Snake::new();
    let mut apple = Apple::new();
    let mut move_duration = 0.15;
    let mut last_move = get_time();
    let mut game_over = false;
    apple.regen(&snake);
    // game loop
    loop {
        draw_field();
        draw_text("RS-NAKE V0.1", 20.0, 20.0, 30.0, BLACK);
        draw_text(
            format!("fps: {:.1}", get_fps()).as_str(),
            400.0,
            20.0,
            20.0,
            BLACK,
        );
        if !game_over {
            draw_text(
                format!("speed: {:.2}", 1.0 / move_duration / 3.33).as_str(),
                280.0,
                20.0,
                20.0,
                BLACK,
            );
            // keyboard input
            {
                if is_key_pressed(KeyCode::Up) {
                    snake.change_direction(Direction::Up);
                }
                if is_key_pressed(KeyCode::Down) {
                    snake.change_direction(Direction::Down);
                }
                if is_key_pressed(KeyCode::Left) {
                    snake.change_direction(Direction::Left);
                }
                if is_key_pressed(KeyCode::Right) {
                    snake.change_direction(Direction::Right);
                }
            }
            // move snake
            if get_time() - last_move > move_duration {
                last_move = get_time();
                game_over = !snake.move_snake();
                if snake.head == apple.position {
                    snake.feed();
                    apple.regen(&snake);
                    // move_duration -= 0.01;
                }
            }
            // render snake
            draw_rectangle(
                FIELD_X_OFFSET + snake.head.x * TILE_SIZE,
                FIELD_Y_OFFSET + snake.head.y * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                DARKGREEN,
            );
            for block in snake.body.iter() {
                draw_rectangle(
                    FIELD_X_OFFSET + &block.x * TILE_SIZE,
                    FIELD_Y_OFFSET + &block.y * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    GREEN,
                );
            }
            // render apple
            draw_rectangle(
                FIELD_X_OFFSET + apple.position.x * TILE_SIZE,
                FIELD_Y_OFFSET + apple.position.y * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                RED,
            );
        } else {
            draw_text("Gameover", 50.0, 50.0, 30.0, RED);
            draw_text("Press Enter to reset", 50.0, 80.0, 30.0, GRAY);
            if is_key_pressed(KeyCode::Enter) {
                // reset game
                snake = Snake::new();
                apple.regen(&snake);
                // move_duration = 0.3;
                game_over = false;
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
        ..Conf::default()
    }
}

fn draw_field() {
    // render field lines
    clear_background(DARKGRAY);
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
