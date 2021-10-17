use speedy2d::Graphics2D;

use crate::{screen::camera::Camera, ui::img::ImgManager, utility::animation::AnimationSelectError, world::space::GamePos};

pub mod player;
pub mod tile;

pub trait Entity {
    fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, camera: &Camera);
    fn moove(&mut self, change_pos: (f32, f32));
    fn set_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError>;
    fn intercept_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError>;
    fn remove_anim(&mut self);
    fn get_pos(&self) -> GamePos;
    fn update(&mut self);
}