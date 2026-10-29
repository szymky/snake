use std::collections::VecDeque;

use anyhow::Result;
use macroquad::{
    color::{BLACK, GREEN, RED},
    input::{KeyCode, get_last_key_pressed},
    text::draw_text,
    time::get_frame_time,
    window::{Conf, clear_background, next_frame},
};

use crate::{
    constants::{MOVEMENT_INTERVAL, WINDOW_HEIGHT, WINDOW_WIDTH},
    food::Food,
    helper::{draw_board, draw_square},
    orientation::Orientation,
    pos::Pos,
    snake::Snake,
};

pub mod constants;
pub mod food;
pub mod helper;
pub mod orientation;
pub mod pos;
pub mod snake;

fn config() -> Conf {
    Conf {
        window_title: "snake".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() -> Result<()> {
    let mut snake = Snake::new(Pos::new(3, 3), Orientation::Right);
    let mut time_since_last_move = 0.0;
    let mut food = Food::spawn();

    let mut alive = true;

    loop {
        time_since_last_move += get_frame_time();

        if time_since_last_move >= MOVEMENT_INTERVAL {
            let has_eaten_food =
                if snake.body.front().expect("Snake body should not be None") == &food.pos {
                    food = Food::spawn();

                    true
                } else {
                    false
                };
            if !snake.move_next(has_eaten_food) {
                alive = false;
            }
            time_since_last_move = 0.0;
        }

        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Up => {
                    if snake.dir != Orientation::Down {
                        snake.dir = Orientation::Up;
                    }
                }
                KeyCode::Down => {
                    if snake.dir != Orientation::Up {
                        snake.dir = Orientation::Down;
                    }
                }
                KeyCode::Left => {
                    if snake.dir != Orientation::Right {
                        snake.dir = Orientation::Left;
                    }
                }
                KeyCode::Right => {
                    if snake.dir != Orientation::Left {
                        snake.dir = Orientation::Right;
                    }
                }
                _ => {}
            }
        }

        clear_background(BLACK);
        draw_board();
        draw_square(food.pos.x, food.pos.y, GREEN);

        snake.draw();

        if !alive {
            draw_text(
                "YOU DIED".to_string(),
                WINDOW_WIDTH as f32 / 2.0,
                WINDOW_HEIGHT as f32 / 2.0,
                100.0,
                RED,
            );
        }

        next_frame().await
    }
}
