use serde::{Deserialize, Serialize};
use speedy2d::color::Color;

use crate::{
    draw::animation::{self, Animation},
    world::{
        operation::PostOperation,
        space::{Direction, GamePos},
    },
};

use super::{friendly::MoveLeft, get_default_anim, Entity};

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pos: GamePos,
    anim: Animation,
    effect_pos: GamePos,
}

#[typetag::serde]
impl Entity for Button {
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
        Box::new(Button::new(pos))
    }

    fn pick(&self) -> Box<dyn Entity> {
        Box::new(Button::new(GamePos::origin()))
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        let effect_pos = self.effect_pos;
        PostOperation::new_empty()
            .with_block_player(move_pos)
            .with_custom(move |w, p| {
                if let Some((_, tile)) = &mut w.mgr.get_tile_at_pos(effect_pos) {
                    tile.change_self();
                }
            })
    }

    fn next(&self) -> Box<dyn Entity> {
        Box::new(OneWay::new(GamePos::origin(), Direction::Center))
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
            pos: GamePos::origin(),
            anim: get_default_anim((2, 4)),
            effect_pos: GamePos::origin(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OneWay {
    pos: GamePos,
    anim: Animation,
    direction: Direction,
}

#[typetag::serde]
impl Entity for OneWay {
    fn draw_color(&self) -> Color {
        Color::from_hex_argb(0xFFAAAAAA)
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

    fn create(&self, pos: GamePos, direction: Direction) -> Box<dyn Entity> {
        Box::new(OneWay::new(pos, direction))
    }

    fn pick(&self) -> Box<dyn Entity> {
        Box::new(OneWay::new(GamePos::origin(), Direction::Center))
    }

    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        let direction = self.direction;
        PostOperation::new_empty().with_block_when(
            move |_| {
                let dir_vec = direction.direction_vector();
                ((dir_vec.x * move_pos.x) < 0.) || ((dir_vec.y * move_pos.y) < 0.)
            },
            move_pos,
        )
    }

    fn next(&self) -> Box<dyn Entity> {
        Box::new(MoveLeft::new(GamePos::origin()))
    }
}

impl OneWay {
    pub fn new(pos: GamePos, direction: Direction) -> OneWay {
        OneWay {
            pos,
            anim: get_default_anim(animation::match_directions(direction, (2, 1))),
            direction,
        }
    }
}
