use serde::{Deserialize, Serialize};

use crate::{
    entity::player::Player,
    utility::animation::Animation,
    world::{
        space::{Direction, GamePos},
        time::Clock,
    },
};

use super::{Tile, TileEnum, TileVariant, get_default_anim};

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
    
    fn get_tile_enum(&self) -> TileEnum {
        TileEnum::Arrow
    }
}

impl Arrow {
    pub fn new(pos: GamePos, direction: TileVariant) -> Arrow {
        Arrow {
            pos,
            anim: get_default_anim(match direction {
                TileVariant::Top => (2, 1),
                TileVariant::Bottom => (0, 1),
                TileVariant::Left => (2, 2),
                TileVariant::CornerBL => (2, 2),
                TileVariant::CornerTL => (2, 2),
                TileVariant::Right => (0, 2),
                TileVariant::CornerBR => (0, 2),
                TileVariant::CornerTR => (0, 2),
                TileVariant::Center => (0, 0),
            }),
        }
    }
}
