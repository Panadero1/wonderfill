use speedy2d::shape::Rectangle;

use crate::world::space::GamePos;

use super::get_resolution;

pub struct Camera {
    pub pos: GamePos,
    pub width: f32,
    pub height: f32,
    velocity: GamePos,
}
impl Camera {
    pub fn new(pos: (f32, f32), width: f32, height: f32) -> Camera {
        Camera {
            pos: pos.into(),
            width,
            height,
            velocity: (0.0, 0.0).into(),
        }
    }
    pub fn moove(&mut self, change_pos: (f32, f32)) {
        self.pos += (change_pos.0, change_pos.1).into();
    }
    pub fn game_to_pix(&self, point: GamePos) -> (f32, f32) {
        let res = get_resolution();
        let a_pos = (
            ((self.width / 2.0) + point.x - self.pos.x) / self.width,
            ((self.height / 2.0) + point.y - self.pos.y) / self.height,
        );
        let resul = (a_pos.0 * (res.0 as f32), a_pos.1 * (res.1 as f32));
        resul
    }
}