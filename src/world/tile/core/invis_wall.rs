use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, Tile, TileVariant, PostOperation}, entity::player::Player,
    },
};

use super::moon::Moon;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvisWall {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for InvisWall {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Moon::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(InvisWall::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }

    fn on_player_enter(&mut self, _player: &mut Player, move_pos: GamePos) -> Vec<PostOperation> {
        vec![PostOperation::MovePlayer(-move_pos)]
    }
}

impl InvisWall {
    pub fn new(pos: GamePos) -> InvisWall {
        InvisWall {
            pos,
            anim: get_default_anim((17, 0)),
        }
    }
}
