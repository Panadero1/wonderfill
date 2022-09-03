use serde::{Serialize, Deserialize};
use speedy2d::color::Color;

use crate::{world::{space::{GamePos, Direction}, operation::PostOperation, minigame::smiley_win::SmileyWin}, draw::animation::Animation};

use super::{Entity, get_default_anim, utility::Button};

#[derive(Debug, Serialize, Deserialize)]
/// Test thing. Don't let it escape lol
pub struct MoveLeft {
    pos: GamePos,
    anim: Animation,
    should_move: bool,
}

#[typetag::serde]
impl Entity for MoveLeft {
    fn draw_color(&self) -> Color {
        Color::from_hex_argb(0xFF00FF00)
    }

    fn moove(&mut self, change_pos: GamePos) {
        self.pos += change_pos;
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn create(&self, pos: GamePos, _direction: Direction) -> Box<dyn Entity> {
        Box::new(MoveLeft::new(pos))
    }

    fn pick(&self) -> Box<dyn Entity> {
        Box::new(MoveLeft::new(GamePos::origin()))
    }

    fn request_moves(&mut self, move_pos: &mut Vec<GamePos>, _player_pos: GamePos) {
        if self.should_move {
            move_pos.extend([(-1, 0).into(), (0, 1).into(), (0, -1).into()].iter());
        }
    }

    fn next (&self) -> Box<dyn Entity> {
        Box::new(SmileyMan::new(GamePos::origin()))
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        // Player moving onto my tile means I am free to move to their tile. Default behavior for friendlies
        self.moove(-move_pos);
        self.should_move = false;
        PostOperation::new_empty()
    }

    fn do_turn(&mut self) -> PostOperation {
        self.should_move = true;
        PostOperation::new_empty()
    }
}

impl MoveLeft {
    pub fn new(pos: GamePos) -> MoveLeft {
        MoveLeft {
            pos,
            anim: get_default_anim((0, 1)),
            should_move: true
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Test thing. Don't let it escape lol
pub struct SmileyMan {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Entity for SmileyMan {
    fn draw_color(&self) -> Color {
        Color::from_hex_argb(0xFF00FF00)
    }

    fn moove(&mut self, change_pos: GamePos) {
        self.pos += change_pos;
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn create(&self, pos: GamePos, _direction: Direction) -> Box<dyn Entity> {
        Box::new(SmileyMan::new(pos))
    }

    fn pick(&self) -> Box<dyn Entity> {
        Box::new(SmileyMan::new(GamePos::origin()))
    }

    fn next (&self) -> Box<dyn Entity> {
        Box::new(Button::default())
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos).with_minigame(Box::new(SmileyWin::new()))
    }
}

impl SmileyMan {
    pub fn new(pos: GamePos) -> SmileyMan {
        SmileyMan {
            pos,
            anim: get_default_anim((0, 2)),
        }
    }
}