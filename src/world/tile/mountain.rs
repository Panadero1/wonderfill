use serde::{Deserialize, Serialize};

use crate::{world::{
    GamePos, Tile, TileVariant,
}, draw::animation::Animation};

use super::{get_default_anim, match_directions};

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

    fn block_movement(&self) -> bool {
        true
    }

    fn next(&self) -> Box<dyn Tile> {
        Box::new(CliffFace::new(GamePos::origin()))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Boulder::new(pos, variant))
    }

    fn pick(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: GamePos::origin(),
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

    fn next(&self) -> Box<dyn Tile> {
        Box::new(Rock::new(GamePos::origin()))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(CliffFace::new(pos))
    }

    fn pick(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: GamePos::origin(),
            anim: get_default_anim((0, 0)),
        })
    }

    fn block_movement(&self) -> bool {
        true
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

    fn block_movement(&self) -> bool {
        true
    }

    fn next(&self) -> Box<dyn Tile> {
        Box::new(super::beehive::HoneyComb::new(GamePos::origin(), TileVariant::Center))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Rock::new(pos))
    }

    fn pick(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: GamePos::origin(),
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
