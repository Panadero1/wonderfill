use crate::{
    entity::{player::Player, Entity},
    screen::camera::Camera,
    ui::img::Img,
    utility::animation::{Animation, AnimationSelectError},
};

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

// Tile & region
#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pos: GamePos,
    anim: Animation,
}

impl Entity for Tile {
    fn draw(&mut self, graphics: &mut Graphics2D, camera: &Camera) {
        self.anim.draw(
            graphics,
            camera.rect_from_center(self.pos, (1.0, 1.0).into()),
            Color::WHITE,
        );
    }

    fn moove(&mut self, change_pos: (f32, f32)) {
        // Don't do anything; tiles shouldn't move
    }

    fn set_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        self.anim.select(anim_name)
    }

    fn intercept_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        self.anim.intercept(anim_name)
    }

    fn remove_anim(&mut self) {
        self.anim.deselect();
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn update(&mut self) {
        todo!()
    }
}

impl Tile {
    pub fn new(pos: GamePos, anim: Animation) -> Tile {
        Tile {
            pos,
            anim
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    tiles: Vec<Option<Tile>>,
    origin: GamePos,
}

impl Region {
    pub fn new(tiles: Vec<Option<Tile>>, origin: GamePos, path: String) -> Region {
        Region {
            tiles,
            origin,
        }
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D, camera: &Camera) {
        for tile in &mut self.tiles {
            if let Some(tile) = tile {
                tile.draw(graphics, camera);
            }
        }
    }
}
