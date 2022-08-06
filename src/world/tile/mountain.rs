use serde::{Deserialize, Serialize};

use crate::world::{
    tile::{get_default_anim, match_directions, Animation},
    GamePos, PostOperation, Tile, TileVariant,
};

// Boulder

#[derive(Debug, Serialize, Deserialize)]
pub struct Boulder {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Boulder {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(CliffFace::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Boulder::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Boulder {
    pub fn new(pos: GamePos, direction: TileVariant) -> Boulder {
        Boulder {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 10))),
        }
    }
}

// CliffFace

#[derive(Debug, Serialize, Deserialize)]
pub struct CliffFace {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for CliffFace {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Rock::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(CliffFace::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }
}

impl CliffFace {
    pub fn new(pos: GamePos) -> CliffFace {
        CliffFace {
            pos,
            anim: get_default_anim((0, 6)),
        }
    }
}

// Rock

#[derive(Debug, Serialize, Deserialize)]
pub struct Rock {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Rock {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(super::beehive::HoneyComb::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Rock::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Rock {
    pub fn new(pos: GamePos) -> Rock {
        Rock {
            pos,
            anim: get_default_anim((0, 4)),
        }
    }
}
