use crate::entity::player::Player;

use self::space::{GamePos, Region};
use serde::{Serialize, Deserialize};

pub mod space;

#[derive(Serialize, Deserialize)]
pub struct World {
    regions: Vec<Region>,
    player: Player,
}

impl World {
    pub fn new(regions: Vec<Region>, player: Player) -> World {
        World {
            regions,
            player,
        }
    }
}