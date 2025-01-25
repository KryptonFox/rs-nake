use crate::constants::{FIELD_HEIGHT, FIELD_WIDTH};
use crate::Direction;
use macroquad::math::Vec2;

pub struct Snake {
    pub head: Vec2,
    pub body: Vec<Vec2>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            head: Vec2::new(1.0, 3.0),
            body: vec![Vec2::new(1.0, 2.0), Vec2::new(2.0, 2.0)],
            direction: Direction::Down,
            next_direction: Direction::Down,
        }
    }

    pub fn move_snake(&mut self) -> bool {
        self.direction = self.next_direction.clone();
        let next_move = match self.direction {
            Direction::Left => Vec2::new(self.head.x - 1.0, self.head.y),
            Direction::Right => Vec2::new(self.head.x + 1.0, self.head.y),
            Direction::Up => Vec2::new(self.head.x, self.head.y - 1.0),
            Direction::Down => Vec2::new(self.head.x, self.head.y + 1.0),
        };
        if next_move.x >= 0.0
            && next_move.y >= 0.0
            && next_move.x <= (FIELD_WIDTH - 1) as f32
            && next_move.y <= (FIELD_HEIGHT - 1) as f32
        {
            self.body.pop();
            self.body.insert(0, self.head);
            self.head = next_move;
            !self.body.contains(&next_move)
        } else {
            false
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.direction != Direction::Right {
                    self.next_direction = direction;
                }
            }
            Direction::Right => {
                if self.direction != Direction::Left {
                    self.next_direction = direction;
                }
            }
            Direction::Up => {
                if self.direction != Direction::Down {
                    self.next_direction = direction;
                }
            }
            Direction::Down => {
                if self.direction != Direction::Up {
                    self.next_direction = direction;
                }
            }
        }
    }

    pub fn feed(&mut self) {
        self.body.push(self.body.last().unwrap().clone());
    }
}
