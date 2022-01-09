use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{get_default_anim, match_directions, PostOperation, Tile, TileVariant},
    },
};

use super::{base_ground::BaseGround, stair::Stair};

#[derive(Debug, Serialize, Deserialize)]
pub struct OneWay {
    pos: GamePos,
    anim: Animation,
    direction: TileVariant,
}

#[typetag::serde]
impl Tile for OneWay {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Stair::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(OneWay::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            direction: TileVariant::Center,
        })
    }

    fn on_player_enter(&mut self, _player: &mut Player, move_pos: GamePos) -> Vec<PostOperation> {
        let mut result = Vec::new();

        let dir_vec = self.direction.direction_vector();

        if (dir_vec.x != 0. && dir_vec.x == -(move_pos.x))
            || (dir_vec.y != 0. && dir_vec.y == -(move_pos.y))
        {
            result.push(PostOperation::MovePlayer(-move_pos));
        }

        result
    }
}

impl OneWay {
    pub fn new(pos: GamePos, direction: TileVariant) -> OneWay {
        OneWay {
            pos,
            anim: get_default_anim(match_directions(direction, (10, 4))),
            direction,
        }
    }
}
