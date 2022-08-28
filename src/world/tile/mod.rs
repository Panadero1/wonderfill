use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
use speedy2d::{color::Color, Graphics2D};

use crate::{
    draw::{
        animation::Animation,
        ui::img::{Img, ImgManager},
    },
    screen::camera::Camera,
};

use self::core::Arrow;

use super::{
    space::{GamePos, SPRITE_EXTENSION_HEIGHT},
    time::Clock,
    VIEW_DIST,
};

pub mod beehive;
pub mod core;
pub mod mountain;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
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
    pub fn direction_vector(&self) -> GamePos {
        use TileVariant::*;
        match self {
            Left => (-1, 0),
            Right => (1, 0),
            Top => (0, -1),
            Bottom => (0, 1),
            CornerBL => (-1, 1),
            CornerBR => (1, 1),
            CornerTR => (1, -1),
            CornerTL => (-1, -1),
            Center => (0, 0),
        }
        .into()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Obstruction {
    Blocking,
    Free,
}
impl Obstruction {
    pub fn toggle(&mut self) {
        *self = match *self {
            Obstruction::Free => Obstruction::Blocking,
            Obstruction::Blocking => Obstruction::Free,
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait Tile: Debug {
    fn get_pos(&self) -> GamePos;
    fn get_anim_mut(&mut self) -> &mut Animation;

    fn block_movement(&self) -> bool {
        false
    }
    /// To trigger some update of the tile's state
    fn change_self(&mut self) {}

    /// For updating the tile's state given the clock
    fn update_state(&mut self, _clock: &Clock) {}

    /// For selecting different animations based on the current state
    fn update_anim(&mut self) {
        self.get_anim_mut().select("base").unwrap();
    }

    fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
    ) {
        let color = self.draw_color();
        let pos = self.get_pos();
        self.get_anim_mut().draw_overworld(
            graphics,
            manager,
            clock,
            camera.rect_from_offset(
                pos,
                (1.0, SPRITE_EXTENSION_HEIGHT).into(),
                (0.0, 1.0 - SPRITE_EXTENSION_HEIGHT).into(),
            ),
            color,
        );
    }
    fn draw_color(&self) -> Color {
        Color::WHITE
    }

    fn create(&self, pos: GamePos, variant: TileVariant) -> Box<dyn Tile>;
    fn pick_tile(&self) -> Box<dyn Tile>;

    fn next(&self) -> Option<Box<dyn Tile>>;
    fn cycle(&self) -> Box<dyn Tile> {
        if let Some(next_tile) = self.next() {
            println!("{}", format!("{:?}", next_tile).split_once(' ').unwrap().0);
            return next_tile;
        }
        return Box::new(Arrow::new((0, 0).into(), TileVariant::Center));
    }
}

pub fn match_directions(direction: TileVariant, top_left: (u16, u16)) -> (u16, u16) {
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

fn get_default_anim(frame: (u16, u16)) -> Animation {
    let mut frames: HashMap<String, (bool, Vec<(u16, u16)>)> = HashMap::new();

    frames.insert(String::from("base"), (true, vec![frame]));

    anim_with_frames(frames)
}

fn anim_with_frames(frames: HashMap<String, (bool, Vec<(u16, u16)>)>) -> Animation {
    Animation::new(
        Img::new(String::from("assets/img/tiles.png")),
        (7, 10),
        frames,
        (5, 0),
        100,
    )
}
