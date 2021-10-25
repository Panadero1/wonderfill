use serde::{Deserialize, Serialize};

use crate::{entity::{Entity, player::Player}, ui::img::Img, utility::animation::Animation, world::{space::GamePos, time::Clock}};

use super::{Tile, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPillar {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for TestPillar {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {
        player.moove(-move_pos);
    }
}

impl TestPillar {
    pub fn new(pos: GamePos) -> TestPillar {
        TestPillar {
            pos,
            anim: get_default_anim((2, 0))
        }
    }
}