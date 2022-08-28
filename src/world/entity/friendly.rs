use serde::{Serialize, Deserialize};
use speedy2d::color::Color;

use crate::{world::{space::GamePos, operation::PostOperation}, draw::animation::Animation};

use super::{Entity, get_default_anim, utility::Button};

#[derive(Debug, Serialize, Deserialize)]
/// Test thing. Don't let it escape lol
pub struct MoveLeft {
    pos: GamePos,
    anim: Animation,
    last_move_pos: GamePos,
}

#[typetag::serde]
impl Entity for MoveLeft {
    fn draw_color(&self) -> Color {
        Color::YELLOW
    }

    fn moove(&mut self, change_pos: GamePos) {
        self.pos += change_pos;
        self.last_move_pos = change_pos;
    }

    fn get_last_move_pos(&self) -> GamePos {
        self.last_move_pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn create(&self, pos: GamePos) -> Box<dyn Entity> {
        Box::new(MoveLeft::new(pos))
    }

    fn pick(&self) -> Box<dyn Entity> {
        Box::new(MoveLeft::new(GamePos::origin()))
    }

    fn do_turn(&mut self) -> PostOperation {
        self.moove((-1, 0).into());
        PostOperation::new_empty()
    }

    fn next (&self) -> Box<dyn Entity> {
        Box::new(Button::default())
    }
}

impl MoveLeft {
    pub fn new(pos: GamePos) -> MoveLeft {
        MoveLeft {
            pos,
            anim: get_default_anim((0, 1)),
            last_move_pos: GamePos::origin(),
        }
    }
}