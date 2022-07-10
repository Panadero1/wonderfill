use std::collections::HashMap;

use speedy2d::color::Color;

use crate::{
    draw::{
        animation::{Animation, AnimationSelectError},
        ui::img::{Img, ImgManager},
    },
    screen::camera::Camera,
    world::{space::GamePos, time::Clock},
};

use serde::{Deserialize, Serialize};

use super::Entity;

#[derive(Serialize, Deserialize, Debug)]
pub enum PlayerHat {
    None,
    Helmet,
    Acid,
    Teardrop,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pos: GamePos,
    anim: Animation,
    size: GamePos,
    hat: PlayerHat,
}
impl Entity for Player {
    fn draw(
        &mut self,
        graphics: &mut speedy2d::Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
    ) {
        self.anim.draw_overworld(
            graphics,
            manager,
            clock,
            camera.rect_from_center(self.pos, self.size),
            Color::WHITE,
        );
    }

    fn moove(&mut self, change_pos: GamePos) {
        self.pos += change_pos;
    }

    fn set_anim(
        &mut self,
        anim_name: &str,
    ) -> Result<(), AnimationSelectError> {
        self.anim.select(anim_name)
    }

    fn intercept_anim(
        &mut self,
        anim_name: &str,
    ) -> Result<(), AnimationSelectError> {
        self.anim.intercept(anim_name)
    }

    fn remove_anim(&mut self) {
        self.anim.deselect()
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn update(&mut self) {
        if let Err(AnimationSelectError::NotFound) = self
            .anim
            .select(&(format!("{:?}", self.hat).to_lowercase())[..])
        {
            panic!("Animation not found");
        }
    }
}

impl Player {
    pub fn new() -> Player {
        let mut frames: HashMap<String, (bool, Vec<(u16, u16)>)> = HashMap::new();

        frames.insert(String::from("none"), (true, vec![(2, 3)]));

        frames.insert(String::from("helmet"), (true, vec![(0, 0)]));

        frames.insert(String::from("acid"), (true, vec![(2, 4)]));

        frames.insert(String::from("teardrop"), (true, vec![(2, 2)]));

        Player {
            pos: (0.0, 0.0).into(),
            anim: Animation::new(
                Img::new(String::from("assets/img/player.png")),
                (7, 7),
                frames,
                (9, 0),
                100,
            ),
            size: (1.0, 1.0).into(),
            hat: PlayerHat::None,
        }
    }
    pub fn set_hat(&mut self, hat: PlayerHat) {
        self.hat = hat;
    }
}
