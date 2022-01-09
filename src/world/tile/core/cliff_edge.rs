use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{get_default_anim, match_directions, PostOperation, Tile, TileVariant},
    },
};

use super::door::Door;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliffEdge {
    pos: GamePos,
    anim: Animation,
    direction: TileVariant,
}

#[typetag::serde]
impl Tile for CliffEdge {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Door::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(CliffEdge::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            direction: TileVariant::Center,
        })
    }

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) -> Vec<PostOperation> {
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

impl CliffEdge {
    pub fn new(pos: GamePos, direction: TileVariant) -> CliffEdge {
        CliffEdge {
            pos,
            anim: get_default_anim(match_directions(direction, (10, 4))),
            direction,
        }
    }
}
