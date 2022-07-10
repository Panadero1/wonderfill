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

    fn on_player_enter(&mut self, _player: &mut Player, move_pos: GamePos) -> Vec<PostOperation> {
        let mut result = Vec::new();
        if let AlternatorState::Up = self.state {
            result.push(PostOperation::MovePlayer(-move_pos));
        }
        result
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

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Sun::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            state: AlternatorState::Down,
        })
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
