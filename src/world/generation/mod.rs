use crate::screen::camera::Camera;

use super::{entity::player::Player, time::Clock, TileManager, World};

pub fn make_new_empty_world() -> World {
    World::new(
        TileManager::new(String::from("empty"), Vec::new()),
        Player::new(),
        Camera::new((0, 0).into(), 10., 10.),
        Clock::new(),
    )
}
