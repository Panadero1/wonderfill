use serde::{Deserialize, Serialize};

use crate::{entity::{Entity, player::Player}, ui::img::Img, utility::animation::Animation, world::{space::GamePos, time::Clock}};

use super::{AlternatorState, Tile, get_default_anim};

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

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }
    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {
        if let AlternatorState::Up = self.state {
            player.moove(-move_pos);
        }
    }
    fn on_update(&mut self, clock: &Clock) {
        self.state = if clock.is_day() {
            AlternatorState::Down
        }
        else {
            AlternatorState::Up
        };
    }
}

impl Moon {
    pub fn new(pos: GamePos) -> Moon {
        Moon {
            pos,
            anim: get_default_anim((6, 0)),
            state: AlternatorState::Down
        }
    }
}