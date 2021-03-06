use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        entity::Entity,
        space::GamePos,
        tile::{get_default_anim, operation::*, Tile, TileVariant},
    },
};

use super::button::Button;

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

    fn on_player_enter(&self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Button::default()))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(BasePillar::default(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
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
