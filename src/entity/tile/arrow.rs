use serde::{Deserialize, Serialize};

use crate::{entity::player::Player, ui::img::Img, utility::animation::Animation, world::{space::{Direction, GamePos}, time::Clock}};

use super::{Tile, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct Arrow {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for Arrow {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, _player: &mut Player, move_pos: GamePos) {
        // Doesn't need to do anything
    }
    
    fn on_update(&mut self, clock: &Clock) {
        // do nothing
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
            })
        }
    }
}