use serde::{Deserialize, Serialize};

use crate::{entity::{Entity, player::Player}, ui::img::Img, utility::animation::Animation, world::{space::GamePos, time::Clock}};

use super::{Tile, TileEnum, TileVariant, get_default_anim, match_directions};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rock {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for Rock {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {
        player.moove(-move_pos);
    }
    
    fn get_tile_enum(&self) -> TileEnum {
        TileEnum::BasePillar
    }
}

impl Rock {
    pub fn new(pos: GamePos, direction: TileVariant) -> Rock {
        Rock {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 10)))
        }
    }
}