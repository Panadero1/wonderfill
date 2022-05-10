use serde::{Deserialize, Serialize};

use crate::{
    utility::animation::Animation,
    world::{
        space::GamePos,
        tile::{get_default_anim, match_directions, Tile, TileVariant, self, PostOperation}, entity::player::Player, minigame::smiley_win::SmileyWin,
    },
};

use super::stair::Stair;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmileyMan {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for SmileyMan {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Stair::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(SmileyMan::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }

    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) -> Vec<PostOperation> {
        vec![PostOperation::MovePlayer(-move_pos), PostOperation::Minigame(Box::new(SmileyWin::new()))]
    }
}

impl SmileyMan {
    pub fn new(pos: GamePos) -> SmileyMan {
        SmileyMan {
            pos,
            anim: get_default_anim((0, 7)),
        }
    }
}