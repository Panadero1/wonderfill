use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{get_default_anim, AlternatorState, PostOperation, Tile, TileVariant},
        time::Clock,
    },
};

use super::{stair::Stair, one_way::OneWay};

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
    fn on_player_enter(&mut self, _player: &mut Player, move_pos: GamePos) -> Vec<PostOperation> {
        let mut result = Vec::new();
        if let AlternatorState::Up = self.state {
            result.push(PostOperation::MovePlayer(-move_pos));
        }
        result
    }
    fn on_update(&mut self, clock: &Clock) {
        self.state = if clock.is_day() {
            AlternatorState::Down
        } else {
            AlternatorState::Up
        };
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(OneWay::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Moon::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            state: AlternatorState::Down,
        })
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
