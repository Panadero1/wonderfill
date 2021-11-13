use std::{collections::{HashMap, HashSet}, fmt::Debug};

use serde::{Deserialize, Serialize};
use speedy2d::{Graphics2D, color::Color};

use crate::{entity::tile::{arrow::Arrow, base_ground::BaseGround, base_pillar::BasePillar, edge::Edge, honeycomb::HoneyComb, moon::Moon, stair::Stair, sun::Sun}, screen::camera::Camera, ui::img::{Img, ImgManager}, utility::animation::{Animation, AnimationSelectError}, world::{space::GamePos, time::Clock}};

use self::rock::Rock;

use super::{Entity, player::Player};

pub mod base_ground;
pub mod base_pillar;
pub mod edge;
pub mod arrow;
pub mod stair;
pub mod sun;
pub mod moon;
pub mod honeycomb;
pub mod rock;

const HEIGHT_GAMEPOS: f32 = 1.0 / 0.7;

pub enum TileEnum {
    BaseGround,
    BasePillar,
    Edge,
    Arrow,
    Stair,
    Sun,
    Moon,
    HoneyComb,
    Rock,
}
impl TileEnum {
    pub fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile> {
        match *self {
            TileEnum::BaseGround => Box::new(BaseGround::default(pos)),
            TileEnum::BasePillar => Box::new(BasePillar::default(pos)),
            TileEnum::Edge => Box::new(Edge::new(pos, variant)),
            TileEnum::Arrow => Box::new(Arrow::new(pos, variant)),
            TileEnum::Stair => Box::new(Stair::new(pos, variant)),
            TileEnum::Sun => Box::new(Sun::new(pos)),
            TileEnum::Moon => Box::new(Moon::new(pos)),
            TileEnum::HoneyComb => Box::new(HoneyComb::new(pos, variant)),
            TileEnum::Rock => Box::new(Rock::new(pos, variant)),
        }
    }
    pub fn cycle(&mut self) {
        *self = match self {
            Self::BaseGround => Self::BasePillar,
            Self::BasePillar => Self::Edge,
            Self::Edge => Self::Arrow,
            Self::Arrow => Self::Stair,
            Self::Stair => Self::Sun,
            Self::Sun => Self::Moon,
            Self::Moon => Self::HoneyComb,
            Self::HoneyComb => Self::Rock,
            Self::Rock => Self::BaseGround,
        };
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TileVariant {
    Left,
    Right,
    Top,
    Bottom,
    CornerBL,
    CornerBR,
    CornerTR,
    CornerTL,
    Center,
}
impl TileVariant {
    pub fn rotate_cw(&mut self) {
        use TileVariant::*;
        *self = match self {
            Center => CornerTL,
            CornerTL => Top,
            Top => CornerTR,
            CornerTR => Right,
            Right => CornerBR,
            CornerBR => Bottom,
            Bottom => CornerBL,
            CornerBL => Left,
            Left => Center, 
        };
    }
    pub fn rotate_ccw(&mut self) {
        use TileVariant::*;
        *self = match self {
            Center => CornerBL,
            CornerBL => Bottom,
            Bottom => CornerBR,
            CornerBR => Right,
            Right => CornerTR,
            CornerTR => Top,
            Top => CornerTL,
            CornerTL => Left,
            Left => Center,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlternatorState {
    Up,
    Down,
}

#[typetag::serde(tag = "type")]
pub trait Tile: Debug {
    fn get_pos(&self) -> GamePos;
    fn get_anim_mut(&mut self) -> &mut Animation;
    fn on_player_enter(&mut self, player: &mut Player, move_pos: GamePos) {}
    fn update_anim(&mut self, clock: &Clock) {
        self.get_anim_mut().select("base").unwrap();
    }
    fn on_update(&mut self, clock: &Clock) {}
    fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, clock: &Clock, camera: &Camera) {
        let pos = self.get_pos();
        self.get_anim_mut().draw(
            graphics,
            manager,
            clock,
            camera.rect_from_offset(pos, (1.0, HEIGHT_GAMEPOS).into(), (0.0, 1.0 - HEIGHT_GAMEPOS).into()),
            Color::WHITE,
        );
    }
    fn get_tile_enum(&self) -> TileEnum;
}

fn get_default_anim(frame: (u16, u16)) -> Animation {
    
    let mut frames: HashMap<String, (bool, Vec<(u16, u16)>)> = HashMap::new();

    frames.insert(String::from("base"), (true, vec![frame]));

    Animation::new(Img::new(String::from("assets\\img\\tiles.png")), (7, 10), frames, (5, 0), 500)
}

fn match_directions(direction: TileVariant, top_left: (u16, u16)) -> (u16, u16) {
    match direction {
        TileVariant::Top => (top_left.0 + 2, top_left.1),
        TileVariant::Bottom => (top_left.0 + 2, top_left.1 + 2),
        TileVariant::Left => (top_left.0, top_left.1 + 1),
        TileVariant::Right => (top_left.0 + 4, top_left.1 + 1),
        TileVariant::CornerBL => (top_left.0, top_left.1 + 2),
        TileVariant::CornerBR => (top_left.0 + 4, top_left.1 + 2),
        TileVariant::CornerTR => (top_left.0 + 4, top_left.1),
        TileVariant::CornerTL => top_left,
        TileVariant::Center => (top_left.0 + 2, top_left.1 + 1),
    }
}