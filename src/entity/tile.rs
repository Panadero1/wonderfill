use speedy2d::{Graphics2D, color::Color};

use crate::{screen::camera::Camera, ui::img::ImgManager, utility::animation::{Animation, AnimationSelectError}, world::space::GamePos};

use super::Entity;

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pos: GamePos,
    anim: Animation,
}

impl Entity for Tile {
    fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, camera: &Camera) {
        self.anim.draw(
            graphics,
            manager,
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
