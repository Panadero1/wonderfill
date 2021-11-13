use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        entity::{player::Player, Entity},
        space::GamePos,
        tile::{core::edge::Edge, get_default_anim, match_directions, Tile, TileVariant},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Boulder {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Boulder {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {
        player.moove(-move_pos);
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Edge::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Boulder::new(pos, variant))
    }
}

impl Boulder {
    pub fn new(pos: GamePos, direction: TileVariant) -> Boulder {
        Boulder {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 10))),
        }
    }
}
