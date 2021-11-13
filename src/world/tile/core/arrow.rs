use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, match_directions, Tile, TileVariant},
    },
};

use super::base_ground::BaseGround;

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

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(BaseGround::default((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Arrow::new(pos, variant))
    }
}

impl Arrow {
    pub fn new(pos: GamePos, direction: TileVariant) -> Arrow {
        Arrow {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 7))),
        }
    }
}
