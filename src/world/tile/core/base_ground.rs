use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, Tile, TileVariant},
    },
};

use super::base_pillar::BasePillar;

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

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(BasePillar::default((0, 0).into())))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(BaseGround::default(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
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
