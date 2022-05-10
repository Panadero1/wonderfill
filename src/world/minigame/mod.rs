use speedy2d::{Graphics2D, window::VirtualKeyCode};

use crate::{ui::img::ImgManager, screen::camera::Camera};

use super::{time::Clock, space::GamePos};

pub mod smiley_win;

pub enum GameResult {
    Success,
    Processing,
    Failure,
}

#[typetag::serde(tag = "type")]
pub trait Minigame {

    fn update(&mut self) -> GameResult;

    fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        camera: &Camera,
    );

    fn key_down(&mut self, key: &VirtualKeyCode);

    fn key_up(&mut self, key: &VirtualKeyCode);

    /// minigame resets upon reloading the game from file if the minigame was in progress
    /// this has to be done because NInstant is not serializable
    fn reset(&mut self);
}
