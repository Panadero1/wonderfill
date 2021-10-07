use std::collections::HashMap;

use speedy2d::{color::Color, image::ImageHandle, shape::Rectangle};

use crate::{utility::animation::{Animation, AnimationSelectError}, world::space::GamePos};

use super::Entity;

#[derive(Debug)]
pub enum PlayerHat {
    None,
    Helmet,
    Acid,
    Teardrop,
}

pub struct Player {
    pos: GamePos,
    anim: Animation,
    size: GamePos,
    hat: PlayerHat
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

    fn update(&mut self) {
        if let Err(AnimationSelectError::NotFound) = self.anim.select(&(format!("{:?}", self.hat).to_lowercase())[..]) {
            panic!("Animation not found");
        }
    }
}

impl Player {
    pub fn new(src: ImageHandle,) -> Player {
        let mut frames: HashMap<&str, (bool, Vec<(u16, u16)>)> = HashMap::new();

        frames.insert("none", (true, vec![(2, 3)]));

        frames.insert("helmet", (true, vec![(0, 0)]));

        frames.insert("acid", (true, vec![(2, 4)]));

        frames.insert("teardrop", (true, vec![(2, 2)]));

        Player {
            pos: (0.0, 0.0).into(),
            anim: Animation::new(src, (7, 7), frames, (9, 0), 100),
            size: (0.7, 0.7).into(),
            hat: PlayerHat::None,
        }
    }
    pub fn set_hat(&mut self, hat: PlayerHat) {
        self.hat = hat;
    }
}