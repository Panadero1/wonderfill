use crate::screen::camera::Camera;

use super::{entity::player::Player, time::Clock, TileManager, World, DataManager};

pub fn make_new_empty_world() -> World {
    World::new()
}
