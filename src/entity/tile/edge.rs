use serde::{Deserialize, Serialize};

use crate::{entity::player::Player, ui::img::Img, utility::animation::Animation, world::{space::{Direction, GamePos}, time::Clock}};

use super::{Tile, TileVariant, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for Edge {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }
}

impl Edge {
    pub fn new(pos: GamePos, direction: TileVariant) -> Edge {
        Edge {
            pos,
            anim: get_default_anim(match direction {
                TileVariant::Top => (6, 1),
                TileVariant::Bottom => (6, 3),
                TileVariant::Left => (4, 2),
                TileVariant::Right => (8, 2),
                TileVariant::CornerBL => (4, 3),
                TileVariant::CornerBR => (8, 3),
                TileVariant::CornerTR => (8, 1),
                TileVariant::CornerTL => (4, 1),
                TileVariant::Center => (6, 2),
            })
        }
    }
}