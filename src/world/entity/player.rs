use std::collections::HashMap;

use speedy2d::color::Color;

use crate::{
    draw::{
        animation::{Animation, AnimationSelectError},
        ui::img::Img,
    },
    world::{space::GamePos, time::Clock},
};

use serde::{Deserialize, Serialize};

use super::Entity;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PlayerHat {
    None,
    Helmet,
    Acid,
    Teardrop,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pos: GamePos,
    anim: Animation,
    size: GamePos,
    hat: PlayerHat,
    last_move_pos: GamePos,
}

#[typetag::serde]
impl Entity for Player {

    fn draw_color(&self) -> Color {
        Color::RED
    }

    fn moove(&mut self, change_pos: GamePos) {
        self.pos += change_pos;
        self.last_move_pos = change_pos;
    }

    fn create(&self, pos: GamePos) -> Box<dyn Entity> {
        unreachable!()
    }

    fn pick(&self) -> Box<dyn Entity> {
        unreachable!()
    }

    fn next(&self) -> Box<dyn Entity> {
        unreachable!()
    }

    fn get_frame_size_and_offset(&self) -> (GamePos, GamePos) {
        super::square_anim_size()
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn update_anim(&mut self, _clock: &Clock) {
        let hat = self.hat;
        if let Err(AnimationSelectError::NotFound) = self
            .get_anim_mut()
            .intercept(&(format!("{:?}", hat).to_lowercase())[..])
        {
            panic!("Animation not found. Trying to select: {:?}", self.hat);
        }
    }

    fn get_last_move_pos(&self) -> GamePos {
        self.last_move_pos
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
            pos: GamePos::origin(),
            anim: Animation::new(
                Img::new(String::from("assets/img/player.png")),
                (7, 7),
                frames,
                (9, 0),
                100,
            ),
            size: (1, 1).into(),
            hat: PlayerHat::None,
            last_move_pos: GamePos::origin()
        }
    }

    pub fn set_hat(&mut self, hat: PlayerHat) {
        self.hat = hat;
    }

    pub fn cycle_hat(&mut self) {
        self.hat = match self.hat {
            PlayerHat::None => PlayerHat::Helmet,
            PlayerHat::Helmet => PlayerHat::Acid,
            PlayerHat::Acid => PlayerHat::Teardrop,
            PlayerHat::Teardrop => PlayerHat::None,
        }
    }

}
