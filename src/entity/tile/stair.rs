use serde::{Deserialize, Serialize};

use crate::{entity::player::Player, ui::img::Img, utility::animation::Animation, world::{space::{Direction, GamePos}, time::Clock}};

use super::{Tile, get_default_anim};

pub enum StairDirection {
    Vertical,
    Horizontal,
    CornerBL,
    CornerBR,
    CornerTR,
    CornerTL,
}

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
}

impl Stair {
    pub fn new(pos: GamePos, direction: StairDirection) -> Stair {
        Stair {
            pos,
            anim: get_default_anim(match direction {
                StairDirection::Vertical => (0, 3),
                StairDirection::Horizontal => (2, 3),
                StairDirection::CornerBL => (0, 5),
                StairDirection::CornerBR => (2, 5),
                StairDirection::CornerTR => (2, 4),
                StairDirection::CornerTL => (0, 4),
            })
        }
    }
}