use std::{collections::{HashMap, HashSet}, fmt::Debug};

use serde::{Deserialize, Serialize};
use speedy2d::{Graphics2D, color::Color};

use crate::{screen::camera::Camera, ui::img::{Img, ImgManager}, utility::animation::{Animation, AnimationSelectError}, world::{space::GamePos, time::Clock}};

use super::{Entity, player::Player};

pub mod test_ground;
pub mod test_pillar;
pub mod edge;
pub mod arrow;
pub mod stair;
pub mod sun;
pub mod moon;

const HEIGHT_GAMEPOS: f32 = 1.0 / 0.7;

#[derive(PartialEq, Eq)]
pub enum TileVariant {
    Left,
    Right,
    Top,
    Bottom,
    CornerBL,
    CornerBR,
    CornerTR,
    CornerTL,
    Center,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlternatorState {
    Up,
    Down,
}

#[typetag::serde(tag = "type")]
pub trait Tile: Debug {
    fn get_pos(&self) -> GamePos;
    fn get_anim(&mut self) -> &mut Animation;
    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {}
    fn update_anim(&mut self, clock: &Clock) {
        self.get_anim().select("base").unwrap();
    }
    fn on_update(&mut self, clock: &Clock) {}
    fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, clock: &Clock, camera: &Camera) {
        let pos = self.get_pos();
        self.get_anim().draw(
            graphics,
            manager,
            clock,
            camera.rect_from_offset(pos, (1.0, HEIGHT_GAMEPOS).into(), (0.0, 1.0 - HEIGHT_GAMEPOS).into()),
            Color::WHITE,
        );
    }
}

fn get_default_anim(frame: (u16, u16)) -> Animation {
    let mut frames: HashMap<String, (bool, Vec<(u16, u16)>)> = HashMap::new();

    frames.insert(String::from("base"), (true, vec![frame]));

    Animation::new(Img::new(String::from("assets\\img\\tiles.png")), (7, 10), frames, (5, 0), 500)
}