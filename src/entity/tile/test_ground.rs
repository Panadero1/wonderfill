use serde::{Deserialize, Serialize};

use crate::{entity::player::Player, ui::img::Img, utility::animation::Animation, world::{space::GamePos, time::Clock}};

use super::{Tile, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestGround {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for TestGround {
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

impl TestGround {
    pub fn new(pos: GamePos) -> TestGround {
        TestGround {
            pos,
            anim: get_default_anim((0, 0))
        }
    }
}