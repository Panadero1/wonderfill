use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::{Animation, AnimationSelectError},
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{self, get_default_anim, AlternatorState, PostOperation, Tile, TileVariant},
        time::Clock,
    },
};

use super::edge::Edge;

#[derive(Debug, Serialize, Deserialize)]
pub struct Door {
    pos: GamePos,
    anim: Animation,
    state: AlternatorState,
}

#[typetag::serde]
impl Tile for Door {
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

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Edge::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Door::new(pos))
    }

    fn update_anim(&mut self, _clock: &Clock) {
        if let Err(AnimationSelectError::NotFound) = self.anim.select(match self.state {
            AlternatorState::Up => "base",
            AlternatorState::Down => "open",
        }) {
            panic!();
        };
    }

    fn update_self(&mut self) {
        self.state.toggle();
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            state: AlternatorState::Down,
        })
    }
}

impl Door {
    pub fn new(pos: GamePos) -> Door {
        let mut frames = HashMap::new();

        frames.insert(String::from("open"), (true, vec![(2, 6)]));
        frames.insert(String::from("base"), (true, vec![(2, 5)]));

        Door {
            pos,
            anim: tile::anim_with_frames(frames),
            state: AlternatorState::Up,
        }
    }
}
