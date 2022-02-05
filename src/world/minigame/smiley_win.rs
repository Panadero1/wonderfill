use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use speedy2d::{window::VirtualKeyCode, Graphics2D, shape::Rectangle};

use crate::{
    screen::{camera::Camera, self},
    ui::img::{ImgManager, Img},
    utility::{animation::Animation, time::NInstant},
    world::{space::GamePos, time::Clock},
};

use super::{GameResult, Minigame};

const size: (u16, u16) = (400, 400);

#[derive(Debug, Serialize, Deserialize)]
pub struct SmileyWin {
    start: NInstant,
    anim: Animation,
}

impl SmileyWin {
    pub fn new() -> SmileyWin {
        SmileyWin {
            start: NInstant::now(),
            anim: Animation::new(
                Img::new(String::from("assets\\img\\smile.png")),
                (400, 400),
                HashMap::new(),
                (0, 0),
                1
            ),
        }
    }
}

#[typetag::serde]
impl Minigame for SmileyWin {
    fn update(&mut self) -> GameResult {
        if self.start.get_instant().elapsed().as_millis() > 5000 {
            return GameResult::Success;
        }
        GameResult::Processing
    }

    fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, camera: &Camera) {
        let res = screen::get_resolution();
        self.anim.draw(graphics, manager, Rectangle::from_tuples((50., 50.), ((res.0 - 50) as f32, (res.1 - 50) as f32)));
    }

    fn key_down(&mut self, key: &VirtualKeyCode) {
        
    }

    fn key_up(&mut self, key: &VirtualKeyCode) {
        
    }
}
