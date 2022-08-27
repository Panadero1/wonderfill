use serde::{Deserialize, Serialize};
use speedy2d::Graphics2D;

use crate::{
    draw::{
        animation::{Animation, AnimationSelectError},
        ui::img::{Img, ImgManager},
    },
    screen::camera::Camera,
    world::{space::GamePos, time::Clock},
};

use super::{
    tile::{operation::PostOperation, Tile},
    VIEW_DIST,
};

pub mod player;

#[typetag::serde(tag = "type")]
pub trait Entity {
    fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
    );
    fn moove(&mut self, change_pos: GamePos);
    fn get_anim_mut(&mut self) -> &mut Animation;
    fn get_pos(&self) -> GamePos;
    fn update(&mut self) {}
    fn update_anim(&mut self, clock: &Clock);
    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }
    fn do_turn(&mut self) -> PostOperation {
        PostOperation::new_empty()
    }
}