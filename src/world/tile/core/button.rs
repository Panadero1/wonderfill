use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        entity::Entity,
        space::GamePos,
        tile::{get_default_anim, operation::*, Tile, TileVariant},
    },
};

use super::door::Door;

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pos: GamePos,
    anim: Animation,
    effect_pos: GamePos,
}

#[typetag::serde]
impl Tile for Button {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Door::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Button::new(pos))
    }

    fn on_player_enter(&self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty()
            .with_block_player(move_pos)
            .with_custom(Box::new(move |w, p| {
                if let Some((_, effect_tile)) = w.tile_mgr.tile_at_pos(p.pos[0]) {
                    effect_tile.update_self();
                }
            }))
            .params(Params::new_empty().add_pos(self.effect_pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            effect_pos: (0, 0).into(),
        })
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
            anim: get_default_anim((0, 5)),
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
