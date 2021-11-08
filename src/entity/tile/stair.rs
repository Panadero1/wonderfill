use serde::{Deserialize, Serialize};

use crate::{entity::player::Player, ui::img::Img, utility::animation::Animation, world::{space::{Direction, GamePos}, time::Clock}};

use super::{Tile, TileEnum, TileVariant, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stair {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for Stair {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }
    
    fn get_tile_enum(&self) -> TileEnum {
        TileEnum::Stair
    }
}

impl Stair {
    pub fn new(pos: GamePos, direction: TileVariant) -> Stair {
        Stair {
            pos,
            anim: get_default_anim(match direction {
                TileVariant::Left => (0, 3),
                TileVariant::Right => (0, 3),
                TileVariant::Top => (2, 3),
                TileVariant::Bottom => (2, 3),
                TileVariant::CornerBL => (0, 5),
                TileVariant::CornerBR => (2, 5),
                TileVariant::CornerTR => (2, 4),
                TileVariant::CornerTL => (0, 4),
                TileVariant::Center => (6, 2),
            })
        }
    }
}