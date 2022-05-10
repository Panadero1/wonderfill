use serde::{Deserialize, Serialize};
use speedy2d::shape::Rectangle;

use crate::{screen::get_resolution, world::space::GamePos};

/// the in-game camera that follows the player
#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub pos: GamePos,
    pub width: f32,
    pub height: f32,
}
impl Camera {
    pub fn new(pos: GamePos, width: f32, height: f32) -> Camera {
        Camera {
            pos: pos,
            width,
            height,
        }
    }
    pub fn moove(&mut self, change_pos: GamePos) {
        self.pos += change_pos;
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
    pub fn pix_to_game(&self, point: (u32, u32)) -> GamePos {
        let res = get_resolution();
        let rel_pos = (
            (point.0 as f32) / (res.0 as f32),
            (point.1 as f32) / (res.1 as f32),
        );
        let true_pos = (
            ((rel_pos.0 - 0.5) * self.width) + self.pos.x,
            ((rel_pos.1 - 0.5) * self.height) + self.pos.y,
        );
        true_pos.into()
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
            self.game_to_pix(pos + (size / 2.0) + (offset / 2.0)),
        )
    }
}
