use serde::{Deserialize, Serialize};

use crate::{
    entity::player::Player,
    utility::animation::Animation,
    world::{
        space::{Direction, GamePos},
        time::Clock,
    },
};

use super::{get_default_anim, Tile};

#[derive(Debug, Serialize, Deserialize)]
pub struct Arrow {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Arrow {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }
}

impl Arrow {
    pub fn new(pos: GamePos, direction: Direction) -> Arrow {
        Arrow {
            pos,
            anim: get_default_anim(match direction {
                Direction::Up => (2, 1),
                Direction::Down => (0, 1),
                Direction::Left => (2, 2),
                Direction::Right => (0, 2),
            }),
        }
    }
}
