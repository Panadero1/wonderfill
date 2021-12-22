use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        space::GamePos,
        tile::{core::arrow::Arrow, get_default_anim, match_directions, Tile, TileVariant},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct HoneyComb {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for HoneyComb {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Arrow::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(HoneyComb::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl HoneyComb {
    pub fn new(pos: GamePos, direction: TileVariant) -> HoneyComb {
        HoneyComb {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 4))),
        }
    }
}
