use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{beehive::honeycomb::HoneyComb, get_default_anim, PostOperation, Tile, TileVariant},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rock {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Rock {
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
        Some(Box::new(HoneyComb::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Rock::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Rock {
    pub fn new(pos: GamePos) -> Rock {
        Rock {
            pos,
            anim: get_default_anim((0, 4)),
        }
    }
}
