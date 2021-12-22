use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, match_directions, Tile, TileVariant},
    },
};

use super::grass::Grass;

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Edge {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Grass::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Edge::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Edge {
    pub fn new(pos: GamePos, direction: TileVariant) -> Edge {
        Edge {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 1))),
        }
    }
}
