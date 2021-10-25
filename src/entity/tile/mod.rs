use std::{collections::{HashMap, HashSet}, fmt::Debug};

use speedy2d::{Graphics2D, color::Color};

use crate::{screen::camera::Camera, ui::img::{Img, ImgManager}, utility::animation::{Animation, AnimationSelectError}, world::{space::GamePos, time::Clock}};

use super::{Entity, player::Player};

pub mod test_ground;
pub mod test_pillar;
pub mod arrow;
pub mod stair;

const HEIGHT_GAMEPOS: f32 = 1.0 / 0.7;

#[typetag::serde(tag = "type")]
pub trait Tile: Debug {
    fn get_pos(&self) -> GamePos;
    fn get_anim(&mut self) -> &mut Animation;
    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {}
    fn update_anim(&mut self, clock: &Clock) {
        match clock.get_hour() {
            0 => self.get_anim().select("light").unwrap(),
            6 => self.get_anim().select("dark").unwrap(),
            _ => (),
        }
    }
    fn on_update(&mut self, clock: &Clock) {}
    fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, camera: &Camera) {
        let pos = self.get_pos();
        self.get_anim().draw(
            graphics,
            manager,
            camera.rect_from_offset(pos, (1.0, HEIGHT_GAMEPOS).into(), (0.0, 1.0 - HEIGHT_GAMEPOS).into()),
            Color::WHITE,
        );
    }
}

fn get_default_anim(frame: (u16, u16)) -> Animation {
    let mut frames: HashMap<String, (bool, Vec<(u16, u16)>)> = HashMap::new();

    frames.insert(String::from("light"), (true, vec![frame]));
    // Dark frame is always one to the right from light frame
    frames.insert(String::from("dark"), (true, vec![(frame.0 + 1, frame.1)]));

    Animation::new(Img::new(String::from("assets\\img\\tiles.png")), (7, 10), frames, (5, 0), 500)
}