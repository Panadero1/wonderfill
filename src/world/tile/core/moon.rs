use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{get_default_anim, Obstruction, PostOperation, Tile, TileVariant},
        time::Clock,
    },
};

use super::{stair::Stair, one_way::OneWay};

#[derive(Debug, Serialize, Deserialize)]
pub struct Moon {
    pos: GamePos,
    anim: Animation,
    state: Obstruction,
}

#[typetag::serde]
impl Tile for Moon {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }
    fn on_player_enter(&self,  move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_when_obstructing(move_pos, self.state)
    }
    fn on_update(&mut self, clock: &Clock) {
        self.state = if clock.is_day() {
            Obstruction::Free
        } else {
            Obstruction::Blocking
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
            state: Obstruction::Free,
        })
    }
}

impl Moon {
    pub fn new(pos: GamePos) -> Moon {
        Moon {
            pos,
            anim: get_default_anim((6, 0)),
            state: Obstruction::Free,
        }
    }
}
