use std::collections::HashSet;
use macroquad::rand::ChooseRandom;

use crate::{constants::S_COUNT, pos::Pos};

#[derive(Debug, PartialEq, Eq)]
pub struct Food {
    pub pos: Pos,
}

impl Food {
    pub fn spawn<'a>(blacklist: impl IntoIterator<Item = &'a Pos>) -> Self {
        
        let blocked: HashSet<Pos> = blacklist.into_iter().copied().collect();

        let greenlist: Vec<Pos> = (0..S_COUNT)
            .flat_map(|y| (0..S_COUNT).map(move |x| Pos { x, y }))
            .filter(|p| !blocked.contains(p))
            .collect();

        Self {
             pos: greenlist.choose().copied().unwrap_or(Pos::new(0, 0)),
        }
    }
}
