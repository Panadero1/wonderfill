use crate::world::TileManager;

mod room_1;
mod room_2;
mod room_3;

pub fn add(tile_mgr: &mut TileManager) {
    room_1::add(tile_mgr);
    room_2::add(tile_mgr);
    room_3::add(tile_mgr);
}