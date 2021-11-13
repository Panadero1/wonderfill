use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, Tile, TileVariant},
    },
};

use super::sun::Sun;

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

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Sun::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Stair::new(pos, variant))
    }
}

impl Stair {
    pub fn new(pos: GamePos, direction: TileVariant) -> Stair {
        Stair {
            pos,
            anim: get_default_anim(match direction {
                TileVariant::Left => (0, 1),
                TileVariant::Right => (0, 1),
                TileVariant::Top => (2, 1),
                TileVariant::Bottom => (2, 1),
                TileVariant::CornerBL => (0, 3),
                TileVariant::CornerBR => (2, 3),
                TileVariant::CornerTR => (2, 2),
                TileVariant::CornerTL => (0, 2),
                TileVariant::Center => (0, 0),
            })
        }
    }
}