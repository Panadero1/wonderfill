use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        entity::{player::Player, Entity},
        space::GamePos,
        tile::{get_default_anim, AlternatorState, Tile, TileVariant, PostOperation},
        time::Clock, World, TileManager,
    },
};

use super::warp::Warp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sun {
    pos: GamePos,
    anim: Animation,
    state: AlternatorState,
}

#[typetag::serde]
impl Tile for Sun {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) -> PostOperation {
        if let AlternatorState::Up = self.state {
            PostOperation::Move(-move_pos)
        }
        else {
            PostOperation::None
        }
    }
    fn on_update(&mut self, clock: &Clock) {
        self.state = if clock.is_day() {
            AlternatorState::Up
        } else {
            AlternatorState::Down
        };
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Warp::default()))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Sun::new(pos))
    }
}

impl Sun {
    pub fn new(pos: GamePos) -> Sun {
        Sun {
            pos,
            anim: get_default_anim((8, 0)),
            state: AlternatorState::Up,
        }
    }
}
