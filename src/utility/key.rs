use speedy2d::window::VirtualKeyCode;

use crate::world::space::GamePos;

pub fn match_wasd_directions(key: &VirtualKeyCode) -> GamePos {
    match key {
        VirtualKeyCode::W => (0, -1),
        VirtualKeyCode::A => (-1, 0),
        VirtualKeyCode::S => (0, 1),
        VirtualKeyCode::D => (1, 0),
        _ => (0, 0)
    }
    .into()
}