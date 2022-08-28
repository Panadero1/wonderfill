use serde::{Serialize, Deserialize};
use speedy2d::color::Color;

use crate::{world::{space::GamePos, operation::{PostOperation, Params}}, draw::animation::Animation};

use super::{Entity, get_default_anim, friendly::MoveLeft};

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pos: GamePos,
    anim: Animation,
    effect_pos: GamePos,
    last_move_pos: GamePos,
}

#[typetag::serde]
impl Entity for Button {
    fn moove(&mut self, change_pos: GamePos) {
        self.pos += change_pos;
        self.last_move_pos = change_pos;
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn create(&self, pos: GamePos) -> Box<dyn Entity> {
        Box::new(Button::new(pos))
    }

    fn pick(&self) -> Box<dyn Entity> {
        Box::new(Button::new(GamePos::origin()))
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        println!("player enters");
        PostOperation::new_empty().with_block_player(move_pos).with_custom(move |w, p| {
            if let Some((_, tile)) = &mut w.mgr.get_tile_at_pos(move_pos) {
                tile.change_self();
            }
        })
    }

    fn do_turn(&mut self) -> PostOperation {
        PostOperation::new_empty()
    }

    fn get_last_move_pos(&self) -> GamePos {
        self.last_move_pos
    }

    fn next (&self) -> Box<dyn Entity> {
        Box::new(MoveLeft::new(GamePos::origin()))
    }
}

impl Button {
    pub fn new(pos: GamePos) -> Button {
        let mut x = String::new();
        println!("Enter the x of the tile to be affected by the button: ");
        std::io::stdin().read_line(&mut x).unwrap();
        let mut y = String::new();
        println!("Enter the y of the tile to be affected by the button: ");
        std::io::stdin().read_line(&mut y).unwrap();
        let effect_pos = (
            x.trim().parse::<i32>().unwrap_or_default(),
            y.trim().parse::<i32>().unwrap_or_default(),
        )
            .into();
        Button {
            pos,
            anim: get_default_anim((0, 0)),
            effect_pos,
            last_move_pos: GamePos::origin(),
        }
    }
    pub fn default() -> Button {
        Button {
            pos: GamePos::origin(),
            anim: get_default_anim((2, 4)),
            effect_pos: GamePos::origin(),
            last_move_pos: GamePos::origin(),
        }
    }
}