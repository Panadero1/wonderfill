use speedy2d::Graphics2D;

use crate::{
    draw::{animation::AnimationSelectError, ui::img::ImgManager},
    screen::camera::Camera,
    world::{space::GamePos, time::Clock},
};

pub mod player;

pub trait Entity {
    fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
    );
    fn moove(&mut self, change_pos: GamePos);
    fn set_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError>;
    fn intercept_anim(&mut self, anim_name: &str) -> Result<(), AnimationSelectError>;
    fn remove_anim(&mut self);
    fn get_pos(&self) -> GamePos;
    fn update(&mut self);
}
