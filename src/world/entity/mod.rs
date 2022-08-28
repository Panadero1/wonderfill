use std::collections::HashMap;

use speedy2d::{color::Color, Graphics2D};

use crate::{
    draw::{
        animation::Animation,
        ui::img::{Img, ImgManager},
    },
    screen::camera::Camera,
    world::{space::GamePos, time::Clock},
};

use super::{operation::PostOperation, space::SPRITE_EXTENSION_HEIGHT};

pub mod player;
pub mod utility;

#[typetag::serde(tag = "type")]
pub trait Entity {
    fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
    ) {
        let color = self.draw_color();
        let pos = self.get_pos();
        let (frame_size, offset) = self.get_frame_size_and_offset();
        self.get_anim_mut().draw_overworld(
            graphics,
            manager,
            clock,
            camera.rect_from_offset(pos, frame_size, offset),
            color,
        );
    }
    fn get_frame_size_and_offset(&self) -> (GamePos, GamePos) {
        (
            (1.0, SPRITE_EXTENSION_HEIGHT).into(),
            (0.0, 1.0 - SPRITE_EXTENSION_HEIGHT).into(),
        )
    }
    fn draw_color(&self) -> Color {
        Color::WHITE
    }
    fn moove(&mut self, change_pos: GamePos);
    fn get_anim_mut(&mut self) -> &mut Animation;
    fn get_pos(&self) -> GamePos;
    fn create(&self, pos: GamePos) -> Box<dyn Entity>;
    fn pick(&self) -> Box<dyn Entity>;
    fn update(&mut self) {}
    fn update_anim(&mut self, clock: &Clock) {
        self.get_anim_mut().select("base").unwrap();
    }
    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }
    fn do_turn(&mut self) -> PostOperation {
        PostOperation::new_empty()
    }
}

fn get_default_anim(frame: (u16, u16)) -> Animation {
    let mut frames: HashMap<String, (bool, Vec<(u16, u16)>)> = HashMap::new();

    frames.insert(String::from("base"), (true, vec![frame]));

    anim_with_frames(frames)
}

fn anim_with_frames(frames: HashMap<String, (bool, Vec<(u16, u16)>)>) -> Animation {
    Animation::new(
        Img::new(String::from("assets/img/entities.png")),
        (7, 10),
        frames,
        (5, 0),
        100,
    )
}

fn square_anim_size() -> (GamePos, GamePos) {
    ((1, 1).into(), (0, 0).into())
}
