use serde::{Deserialize, Serialize};

use crate::{entity::{Entity, player::Player}, ui::img::Img, utility::animation::Animation, world::{space::GamePos, time::Clock}};

use super::{Tile, TileEnum, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePillar {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for BasePillar {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {
        player.moove(-move_pos);
    }
    
    fn get_tile_enum(&self) -> TileEnum {
        TileEnum::BasePillar
    }
}

impl BasePillar {
    pub fn new(pos: GamePos, anim_frame: (u16, u16)) -> BasePillar {
        BasePillar {
            pos,
            anim: get_default_anim(anim_frame)
        }
    }
    pub fn default(pos: GamePos) -> BasePillar {
        BasePillar::new(pos, (2, 0))
    }
}