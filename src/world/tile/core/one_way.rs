use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{get_default_anim, match_directions, operation::*, Tile, TileVariant},
    },
};

use super::smiley_man::SmileyMan;

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
        Some(Box::new(SmileyMan::new((0, 0).into())))
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

    fn on_player_enter(&self, move_pos: GamePos) -> PostOperation {

        PostOperation::new_empty().with_block_when(move |p|
        {
            let dir_vec = p.tile_variant.unwrap().direction_vector();
            ((dir_vec.x * move_pos.x) < 0.) || ((dir_vec.y * move_pos.y) < 0.)
        }
        , move_pos).params(Params::new_empty().with_tile_variant(self.direction))
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
