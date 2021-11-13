use serde::{Deserialize, Serialize};

use crate::{
    entity::player::Player,
    ui::img::Img,
    utility::animation::Animation,
    world::{space::GamePos, time::Clock},
};

use super::{Tile, TileEnum, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseGround {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for BaseGround {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn get_tile_enum(&self) -> TileEnum {
        TileEnum::BaseGround
    }
}

impl BaseGround {
    pub fn new(pos: GamePos, anim_frame: (u16, u16)) -> BaseGround {
        BaseGround {
            pos,
            anim: get_default_anim(anim_frame),
        }
    }
    pub fn default(pos: GamePos) -> BaseGround {
        BaseGround::new(pos, (0, 0))
    }
}
