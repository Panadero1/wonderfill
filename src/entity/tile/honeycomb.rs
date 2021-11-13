use serde::{Deserialize, Serialize};

use crate::{entity::{Entity, player::Player}, ui::img::Img, utility::animation::Animation, world::{space::GamePos, time::Clock}};

use super::{Tile, TileEnum, TileVariant, get_default_anim, match_directions};

#[derive(Debug, Serialize, Deserialize)]
pub struct HoneyComb {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for HoneyComb {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }
    
    fn get_tile_enum(&self) -> TileEnum {
        TileEnum::BasePillar
    }
}

impl HoneyComb {
    pub fn new(pos: GamePos, direction: TileVariant) -> HoneyComb {
        HoneyComb {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 4)))
        }
    }
}