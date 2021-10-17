use crate::{entity::{Entity, player::Player, tile::Tile}, screen::camera::Camera, ui::img::{Img, ImgManager}, utility::animation::{Animation, AnimationSelectError}};

use self::space::GamePos;
use serde::{Deserialize, Serialize};
use speedy2d::{color::Color, Graphics2D};

pub mod space;

//#[derive(Serialize, Deserialize)]
pub struct World {
    pub regions: Vec<Region>,
    pub player: Player,
}

impl World {
    pub fn new(regions: Vec<Region>, player: Player) -> World {
        World { regions, player }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    tiles: Vec<Tile>,
}

impl Region {
    pub fn new(tiles: Vec<Tile>) -> Region {
        Region {
            tiles,
        }
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, camera: &Camera) {
        for tile in &mut self.tiles {
            tile.draw(graphics, manager, camera);
        }
    }
}
