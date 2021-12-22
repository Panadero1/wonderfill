use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{get_default_anim, match_directions, PostOperation, Tile, TileVariant},
    },
};

use super::rock::Rock;

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

    fn on_player_enter(&mut self, _player: &mut Player, move_pos: GamePos) -> Vec<PostOperation> {
        vec![PostOperation::MovePlayer(-move_pos)]
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Rock::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Boulder::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
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
