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

use super::stair::Stair;

#[derive(Debug, Serialize, Deserialize)]
pub struct Moon {
    pos: GamePos,
    anim: Animation,
    state: AlternatorState,
}

#[typetag::serde]
impl Tile for Moon {
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
            AlternatorState::Down
        } else {
            AlternatorState::Up
        };
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Stair::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Moon::new(pos))
    }
}

impl Moon {
    pub fn new(pos: GamePos) -> Moon {
        Moon {
            pos,
            anim: get_default_anim((6, 0)),
            state: AlternatorState::Down,
        }
    }
}
