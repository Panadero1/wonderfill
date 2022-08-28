use serde::{Deserialize, Serialize};

use crate::{world::{
    tile::{self, get_default_anim, Animation, match_directions, Obstruction},
    GamePos, Tile, TileVariant, Clock
}, draw::animation::AnimationSelectError};

use std::collections::HashMap;

use super::mountain::Boulder;

// Arrow

#[derive(Debug, Serialize, Deserialize)]
pub struct Arrow {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Arrow {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(BaseGround::default((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Arrow::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Arrow {
    pub fn new(pos: GamePos, direction: TileVariant) -> Arrow {
        Arrow {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 7))),
        }
    }
}

// BaseGround

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseGround {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for BaseGround {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(BasePillar::default((0, 0).into())))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(BaseGround::default(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl BaseGround {
    pub fn new(pos: GamePos, anim_frame: (u16, u16)) -> BaseGround {
        BaseGround {
            pos,
            anim: get_default_anim(anim_frame),
        }
    }
    pub fn default(pos: GamePos) -> BaseGround {
        BaseGround::new(pos, (0, 0))
    }
}

// BasePillar

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePillar {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for BasePillar {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn block_movement(&self) -> bool {
        true
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Door::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(BasePillar::default(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl BasePillar {
    pub fn new(pos: GamePos, anim_frame: (u16, u16)) -> BasePillar {
        BasePillar {
            pos,
            anim: get_default_anim(anim_frame),
        }
    }
    pub fn default(pos: GamePos) -> BasePillar {
        BasePillar::new(pos, (2, 0))
    }
}

// Button
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Button {
//     pos: GamePos,
//     anim: Animation,
//     effect_pos: GamePos,
// }
// #[typetag::serde]
// impl Tile for Button {
//     fn get_pos(&self) -> GamePos {
//         self.pos
//     }
//     fn get_anim_mut(&mut self) -> &mut Animation {
//         &mut self.anim
//     }
//     fn next(&self) -> Option<Box<dyn Tile>> {
//         Some(Box::new(Door::new((0, 0).into())))
//     }
//     fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
//         Box::new(Button::new(pos))
//     }
//     fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
//         PostOperation::new_empty()
//             .with_block_player(move_pos)
//             .with_custom(Box::new(move |w, p| {
//                 if let Some((_, effect_tile)) = w.tile_mgr.tile_at_pos(p.pos[0]) {
//                     effect_tile.update_self();
//                 }
//             }))
//             .params(Params::new_empty().add_pos(self.effect_pos))
//     }
//     fn pick_tile(&self) -> Box<dyn Tile> {
//         Box::new(Self {
//             pos: (0, 0).into(),
//             anim: get_default_anim((0, 0)),
//             effect_pos: (0, 0).into(),
//         })
//     }
// }
// impl Button {
//     pub fn new(pos: GamePos) -> Button {
//         let mut x = String::new();
//         println!("Enter the x of the tile to be affected by the button: ");
//         std::io::stdin().read_line(&mut x).unwrap();
//         let mut y = String::new();
//         println!("Enter the y of the tile to be affected by the button: ");
//         std::io::stdin().read_line(&mut y).unwrap();
//         let effect_pos = (
//             x.trim().parse::<i32>().unwrap_or_default(),
//             y.trim().parse::<i32>().unwrap_or_default(),
//         )
//             .into();
//         Button {
//             pos,
//             anim: get_default_anim((0, 5)),
//             effect_pos,
//         }
//     }
//     pub fn default() -> Button {
//         Button {
//             pos: (0, 0).into(),
//             anim: get_default_anim((2, 4)),
//             effect_pos: (0, 0).into(),
//         }
//     }
// }

// Door

#[derive(Debug, Serialize, Deserialize)]
pub struct Door {
    pos: GamePos,
    anim: Animation,
    state: Obstruction,
}

#[typetag::serde]
impl Tile for Door {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn block_movement(&self) -> bool {
        self.state == Obstruction::Blocking
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Edge::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Door::new(pos))
    }

    fn update_anim(&mut self) {
        if let Err(AnimationSelectError::NotFound) = self.anim.select(match self.state {
            Obstruction::Blocking => "base",
            Obstruction::Free => "open",
        }) {
            panic!();
        };
    }

    fn change_self(&mut self) {
        self.state.toggle();
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            state: Obstruction::Free,
        })
    }
}

impl Door {
    pub fn new(pos: GamePos) -> Door {
        let mut frames = HashMap::new();

        frames.insert(String::from("open"), (true, vec![(2, 6)]));
        frames.insert(String::from("base"), (true, vec![(2, 5)]));

        Door {
            pos,
            anim: tile::anim_with_frames(frames),
            state: Obstruction::Blocking,
        }
    }
}

// Edge

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Edge {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Grass::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Edge::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Edge {
    pub fn new(pos: GamePos, direction: TileVariant) -> Edge {
        Edge {
            pos,
            anim: get_default_anim(match_directions(direction, (4, 1))),
        }
    }
}

// Grass

#[derive(Debug, Serialize, Deserialize)]
pub struct Grass {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Grass {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(InvisWall::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Grass::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Grass {
    pub fn new(pos: GamePos, direction: TileVariant) -> Grass {
        Grass {
            pos,
            anim: get_default_anim(match_directions(direction, (10, 1))),
        }
    }
}

// InvisWall

#[derive(Debug, Serialize, Deserialize)]
pub struct InvisWall {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for InvisWall {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Moon::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(InvisWall::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }

    fn block_movement(&self) -> bool {
        true
    }
}

impl InvisWall {
    pub fn new(pos: GamePos) -> InvisWall {
        InvisWall {
            pos,
            anim: get_default_anim((17, 0)),
        }
    }
}

// Moon

#[derive(Debug, Serialize, Deserialize)]
pub struct Moon {
    pos: GamePos,
    anim: Animation,
    state: Obstruction,
}

#[typetag::serde]
impl Tile for Moon {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }
    fn block_movement(&self) -> bool {
        self.state == Obstruction::Blocking
    }
    fn update_state(&mut self, clock: &Clock) {
        self.state = if clock.is_day() {
            Obstruction::Free
        } else {
            Obstruction::Blocking
        };
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Stair::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Moon::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            state: Obstruction::Free,
        })
    }
}

impl Moon {
    pub fn new(pos: GamePos) -> Moon {
        Moon {
            pos,
            anim: get_default_anim((6, 0)),
            state: Obstruction::Free,
        }
    }
}

// OneWay
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OneWay {
//     pos: GamePos,
//     anim: Animation,
//     direction: TileVariant,
// }
// #[typetag::serde]
// impl Tile for OneWay {
//     fn get_pos(&self) -> GamePos {
//         self.pos
//     }
//     fn get_anim_mut(&mut self) -> &mut Animation {
//         &mut self.anim
//     }
//     fn next(&self) -> Option<Box<dyn Tile>> {
//         Some(Box::new(SmileyMan::new((0, 0).into())))
//     }
//     fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
//         Box::new(OneWay::new(pos, variant))
//     }
//     fn pick_tile(&self) -> Box<dyn Tile> {
//         Box::new(Self {
//             pos: (0, 0).into(),
//             anim: get_default_anim((0, 0)),
//             direction: TileVariant::Center,
//         })
//     }
//     fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
//         PostOperation::new_empty()
//             .with_block_when(
//                 move |p| {
//                     let dir_vec = p.tile_variant.unwrap().direction_vector();
//                     ((dir_vec.x * move_pos.x) < 0.) || ((dir_vec.y * move_pos.y) < 0.)
//                 },
//                 move_pos,
//             )
//             .params(Params::new_empty().with_tile_variant(self.direction))
//     }
// }
// impl OneWay {
//     pub fn new(pos: GamePos, direction: TileVariant) -> OneWay {
//         OneWay {
//             pos,
//             anim: get_default_anim(match_directions(direction, (10, 4))),
//             direction,
//         }
//     }
// }

// SmileyMan
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SmileyMan {
//     pos: GamePos,
//     anim: Animation,
// }
// #[typetag::serde]
// impl Tile for SmileyMan {
//     fn get_pos(&self) -> GamePos {
//         self.pos
//     }
//     fn get_anim_mut(&mut self) -> &mut Animation {
//         &mut self.anim
//     }
//     fn next(&self) -> Option<Box<dyn Tile>> {
//         Some(Box::new(Stair::new((0, 0).into(), TileVariant::Center)))
//     }
//     fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
//         Box::new(SmileyMan::new(pos))
//     }
//     fn pick_tile(&self) -> Box<dyn Tile> {
//         Box::new(Self {
//             pos: (0, 0).into(),
//             anim: get_default_anim((0, 0)),
//         })
//     }
//     fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
//         PostOperation::new_empty()
//             .with_minigame(Box::new(SmileyWin::new()))
//             .with_block_player(move_pos)
//     }
// }
// impl SmileyMan {
//     pub fn new(pos: GamePos) -> SmileyMan {
//         SmileyMan {
//             pos,
//             anim: get_default_anim((0, 7)),
//         }
//     }
// }

// Stair

#[derive(Debug, Serialize, Deserialize)]
pub struct Stair {
    pos: GamePos,
    anim: Animation,
}

#[typetag::serde]
impl Tile for Stair {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Sun::new((0, 0).into())))
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Stair::new(pos, variant))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
        })
    }
}

impl Stair {
    pub fn new(pos: GamePos, direction: TileVariant) -> Stair {
        Stair {
            pos,
            anim: get_default_anim(match direction {
                TileVariant::Left => (0, 1),
                TileVariant::Right => (0, 1),
                TileVariant::Top => (2, 1),
                TileVariant::Bottom => (2, 1),
                TileVariant::CornerBL => (0, 3),
                TileVariant::CornerBR => (2, 3),
                TileVariant::CornerTR => (2, 2),
                TileVariant::CornerTL => (0, 2),
                TileVariant::Center => (0, 0),
            }),
        }
    }
}

// Sun

#[derive(Debug, Serialize, Deserialize)]
pub struct Sun {
    pos: GamePos,
    anim: Animation,
    state: Obstruction,
}

#[typetag::serde]
impl Tile for Sun {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn block_movement(&self) -> bool {
        self.state == Obstruction::Blocking
    }

    fn update_state(&mut self, clock: &Clock) {
        self.state = if clock.is_day() {
            Obstruction::Blocking
        } else {
            Obstruction::Free
        };
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Boulder::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Sun::new(pos))
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            state: Obstruction::Free,
        })
    }
}

impl Sun {
    pub fn new(pos: GamePos) -> Sun {
        Sun {
            pos,
            anim: get_default_anim((8, 0)),
            state: Obstruction::Blocking,
        }
    }
}

// Warp
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Warp {
//     pos: GamePos,
//     anim: Animation,
//     load_name: String,
// }
// #[typetag::serde]
// impl Tile for Warp {
//     fn get_pos(&self) -> GamePos {
//         self.pos
//     }
//     fn get_anim_mut(&mut self) -> &mut Animation {
//         &mut self.anim
//     }
//     fn next(&self) -> Option<Box<dyn Tile>> {
//         Some(Box::new(super::mountain::Boulder::new((0, 0).into(), TileVariant::Center)))
//     }
//     fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
//         Box::new(Warp::new(pos))
//     }
//     fn on_player_enter(&mut self, _move_pos: GamePos) -> PostOperation {
//         PostOperation::new_empty()
//             .with_custom(Box::new(|w, p| {
//                 w.load_region(p.text.as_ref().unwrap())
//                     .expect("load region wrong");
//             }))
//             .params(Params::new_empty().with_text(self.load_name.clone()))
//     }
//     fn pick_tile(&self) -> Box<dyn Tile> {
//         Box::new(Self {
//             pos: (0, 0).into(),
//             anim: get_default_anim((0, 0)),
//             load_name: String::from("a"),
//         })
//     }
// }
// impl Warp {
//     pub fn new(pos: GamePos) -> Warp {
//         let mut name = String::new();
//         println!("Enter the name of the zone for this tile to load: ");
//         std::io::stdin().read_line(&mut name).unwrap();
//         Warp {
//             load_name: name.trim().to_string(),
//             pos,
//             anim: get_default_anim((2, 4)),
//         }
//     }
//     pub fn default() -> Warp {
//         Warp {
//             pos: (0, 0).into(),
//             anim: get_default_anim((2, 4)),
//             load_name: String::from(""),
//         }
//     }
// }
