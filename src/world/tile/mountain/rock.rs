use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        entity::{player::Player, Entity},
        space::GamePos,
        tile::{core::stair::Stair, get_default_anim, Tile, TileVariant, PostOperation, beehive::honeycomb::HoneyComb}, World, TileManager,
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

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) -> PostOperation {
        PostOperation::Move(-move_pos)
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(HoneyComb::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Rock::new(pos))
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
