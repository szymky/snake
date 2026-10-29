use std::collections::VecDeque;

use macroquad::color::*;

use crate::{
    constants::S_COUNT,
    helper::{draw_square, wrap},
    orientation::Orientation,
    pos::Pos,
};

#[derive(Debug)]
pub struct Snake {
    pub body: VecDeque<Pos>,
    pub dir: Orientation,
}

impl Snake {
    pub fn new(begin: Pos, orientation: Orientation) -> Self {
        let mut body = VecDeque::new();

        body.push_back(Pos::new(begin.x, begin.y));
        body.push_back(Pos::new(begin.x - 1, begin.y));
        body.push_back(Pos::new(begin.x - 2, begin.y));

        Self {
            body,
            dir: orientation,
        }
    }

    pub fn draw(&self) {
        for b in self.body.iter().enumerate() {
            let color = if b.0 == 0 {
                RED
            } else {
                Color {
                    r: 0.0,
                    g: 1.0 - (b.0 as f32 / 100.0),
                    b: 0.19,
                    a: 1.0,
                }
            };

            draw_square(b.1.x, b.1.y, color);
        }
    }

    pub fn move_next(&mut self, has_eaten_food: bool) -> bool {
        let first = self.body.front().expect("Body can't be empty");

        let new_head = match self.dir {
            Orientation::Up => Pos::new(first.x, wrap(first.y - 1, S_COUNT)),
            Orientation::Down => Pos::new(first.x, wrap(first.y + 1, S_COUNT)),
            Orientation::Left => Pos::new(wrap(first.x - 1, S_COUNT), first.y),
            Orientation::Right => Pos::new(wrap(first.x + 1, S_COUNT), first.y),
        };

        for b in self.body.iter() {
            if b == &new_head {
                return false;
            }
        }

        self.body.push_front(new_head);

        if !has_eaten_food {
            self.body.pop_back();
        }
        return true;
    }
}
