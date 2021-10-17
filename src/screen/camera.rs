use speedy2d::shape::Rectangle;

use crate::world::space::GamePos;

use super::get_resolution;

pub struct Camera {
    pub pos: GamePos,
    pub width: f32,
    pub height: f32,
}
impl Camera {
    pub fn new(pos: (f32, f32), width: f32, height: f32) -> Camera {
        Camera {
            pos: pos.into(),
            width,
            height,
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
        let result = (a_pos.0 * (res.0 as f32), a_pos.1 * (res.1 as f32));
        result
    }
    pub fn rect_from_center(&self, pos: GamePos, size: GamePos) -> Rectangle {
        Rectangle::from_tuples(
        self.game_to_pix(pos - (size / 2.0)),
        self.game_to_pix(pos + (size / 2.0)),
        )
    }
    pub fn rect_from_offset(&self, pos: GamePos, size: GamePos, offset: GamePos) -> Rectangle {
        Rectangle::from_tuples(
            self.game_to_pix(pos - (size / 2.0) + (offset / 2.0)),
            self.game_to_pix(pos + (size / 2.0) + (offset / 2.0))
        )
    }
}