use std::collections::HashMap;

use speedy2d::{color::Color, image::ImageHandle, shape::Rectangle};

use crate::{utility::animation::Animation, world::space::GamePos};

use super::Entity;

pub struct Player {
    pos: GamePos,
    anim: Animation,
    size: GamePos,
}
impl Entity for Player {
    fn draw(
        &mut self,
        graphics: &mut speedy2d::Graphics2D,
        camera: &crate::screen::camera::Camera,
    ) {
        self.anim.draw(
            graphics,
            Rectangle::from_tuples(
                camera.game_to_pix(self.pos - (self.size / 2.0)).into(),
                camera.game_to_pix(self.pos + (self.size / 2.0)).into(),
            ),
            Color::WHITE,
        );
    }

    fn moove(&mut self, change_pos: (f32, f32)) {
        self.pos += change_pos.into();
    }

    fn set_anim(
        &mut self,
        anim_name: &str,
    ) -> Result<(), crate::utility::animation::AnimationSelectError> {
        self.anim.select(anim_name)
    }

    fn intercept_anim(
        &mut self,
        anim_name: &str,
    ) -> Result<(), crate::utility::animation::AnimationSelectError> {
        self.anim.intercept(anim_name)
    }

    fn remove_anim(&mut self) {
        self.anim.deselect()
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }
}

impl Player {
    pub fn new(src: ImageHandle,) -> Player {
        let frames: HashMap<&str, (bool, Vec<(u16, u16)>)> = HashMap::new();
        Player {
            pos: (0.0, 0.0).into(),
            anim: Animation::new(src, (7, 7), frames, (9, 0), 100),
            size: (0.7, 0.7).into(),
        }
    }
}