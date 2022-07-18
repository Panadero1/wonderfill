use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, match_directions, PostOperation, Tile, TileVariant},
    },
};

use super::cliff_face::CliffFace;

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

    fn on_player_enter(&self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(CliffFace::new((0, 0).into())))
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
