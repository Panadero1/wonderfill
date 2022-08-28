use serde::{Serialize, Deserialize};
use speedy2d::color::Color;

use crate::{world::{space::GamePos, operation::PostOperation}, draw::animation::Animation};

use super::{Entity, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pos: GamePos,
    anim: Animation,
    effect_pos: GamePos,
}

#[typetag::serde]
impl Entity for Button {
    fn draw_color(&self) -> Color {
        Color::YELLOW
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

    fn create(&self, pos: GamePos) -> Box<dyn Entity> {
        Box::new(Button::new(pos))
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos).with_custom(|w, _| {
            if let Some((_, tile)) = &mut w.mgr.get_tile_at_pos(w.player.get_pos()) {
                tile.change_self();
            }
        })
    }

    fn do_turn(&mut self) -> PostOperation {
        PostOperation::new_empty()
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
        }
    }
    pub fn default() -> Button {
        Button {
            pos: (0, 0).into(),
            anim: get_default_anim((2, 4)),
            effect_pos: (0, 0).into(),
        }
    }
}