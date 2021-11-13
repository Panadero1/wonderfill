use serde::{Deserialize, Serialize};

use crate::{utility::animation::Animation, world::{entity::{Entity, player::Player}, space::GamePos, tile::{get_default_anim, Tile, TileVariant}}};

use super::edge::Edge;

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePillar {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for BasePillar {
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
        Box::new(BasePillar::default(pos))
    }
}

impl BasePillar {
    pub fn new(pos: GamePos, anim_frame: (u16, u16)) -> BasePillar {
        BasePillar {
            pos,
            anim: get_default_anim(anim_frame),
        }
    }
    pub fn default(pos: GamePos) -> BasePillar {
        BasePillar::new(pos, (2, 0))
    }
}
