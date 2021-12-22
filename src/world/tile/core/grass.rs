use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, match_directions, Tile, TileVariant},
    },
};

use super::moon::Moon;

#[derive(Debug, Serialize, Deserialize)]
pub struct Grass {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Grass {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Moon::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Grass::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Grass {
    pub fn new(pos: GamePos, direction: TileVariant) -> Grass {
        Grass {
            pos,
            anim: get_default_anim(match_directions(direction, (10, 1))),
        }
    }
}
