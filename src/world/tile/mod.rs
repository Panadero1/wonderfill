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

use self::{core::Arrow, operation::*};

use super::{space::GamePos, time::Clock, VIEW_DIST};

pub mod beehive;
pub mod core;
pub mod mountain;

const HEIGHT_GAMEPOS: f32 = 1.0 / 0.7;

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

pub mod operation {
    pub type OpFn = Box<dyn Fn(&mut World, &Params)>;

    use std::{cell::RefCell, rc::Rc};

    use crate::world::{
        minigame::Minigame,
        space::GamePos,
        tile::{Obstruction, TileVariant},
        World, entity::Entity,
    };

    use serde::{de::Visitor, Deserialize, Serialize};

    pub struct PostOperation {
        op_fns: Rc<RefCell<Vec<OpFn>>>,
        params: Params,
    }

    impl Clone for PostOperation {
        fn clone(&self) -> Self {
            PostOperation {
                op_fns: self.op_fns.clone(),
                params: self.params.clone(),
            }
        }
    }

    impl Serialize for PostOperation {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str("0")
        }
    }

    impl<'de> Deserialize<'de> for PostOperation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(PostOperationVisitor)
        }
    }

    impl PostOperation {
        pub fn new_empty() -> PostOperation {
            PostOperation {
                op_fns: Rc::new(RefCell::new(Vec::new())),
                params: Params::new_empty(),
            }
        }

        pub fn with_custom(self, op_fn: OpFn) -> PostOperation {
            self.op_fns.borrow_mut().push(op_fn);
            self
        }

        pub fn with_block_player(self, move_pos: GamePos) -> PostOperation {
            self.with_move_player(-move_pos)
        }

        pub fn with_move_player(self, move_pos: GamePos) -> PostOperation {
            self.with_custom(Box::new(move |w, _p| w.player.moove(move_pos)))
        }

        pub fn with_block_when<P>(self, predicate: P, move_pos: GamePos) -> PostOperation
        where
            P: 'static + Fn(&Params) -> bool,
        {
            self.with_custom(Box::new(move |w, p| {
                if predicate(p) {
                    w.player.moove(-move_pos)
                }
            }))
        }

        pub fn with_block_when_obstructing(
            mut self,
            move_pos: GamePos,
            obstruction: Obstruction,
        ) -> PostOperation {
            self.params = self.params.with_obstruction(obstruction);
            self.with_block_when(
                |p| p.obstruction.as_ref().unwrap() == &Obstruction::Blocking,
                move_pos,
            )
        }

        pub fn with_minigame(mut self, minigame: Box<dyn Minigame>) -> PostOperation {
            // The `Some(p.minigame.unwrap())` may seem unneccessary but we want to assert that p always has a minigame
            // ... and we want to break when it doesn't because that's UB
            // I could do assert! but then I'd have to make the closure multi-line and that's kinda ugly
            self.params = self.params.with_minigame(minigame);
            self.with_custom(Box::new(move |w, p| {
                w.minigame = Some(p.minigame.as_ref().unwrap().create())
            }))
        }

        pub fn params(mut self, params: Params) -> PostOperation {
            self.params = params;
            self
        }

        pub fn execute(&self, world: &mut World) {
            let op_fns = self.op_fns.as_ref().borrow();
            let op_fns: &Vec<OpFn> = op_fns.as_ref();
            for op_fn in op_fns {
                op_fn(world, &self.params)
            }
        }
    }

    struct PostOperationVisitor;

    impl<'de> Visitor<'de> for PostOperationVisitor {
        type Value = PostOperation;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("str")
        }

        fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(PostOperation::new_empty())
        }
    }

    pub struct Params {
        pub pos: Vec<GamePos>,
        pub obstruction: Option<Obstruction>,
        pub tile_variant: Option<TileVariant>,
        pub text: Option<String>,
        pub minigame: Option<Box<dyn Minigame>>,
    }

    impl Clone for Params {
        fn clone(&self) -> Self {
            Params {
                pos: self.pos.clone(),
                obstruction: self.obstruction.clone(),
                tile_variant: self.tile_variant.clone(),
                text: self.text.clone(),
                minigame: {
                    if let Some(minigame) = &self.minigame {
                        Some(minigame.create())
                    } else {
                        None
                    }
                },
            }
        }
    }

    impl Params {
        pub fn new_empty() -> Params {
            Params {
                pos: Vec::new(),
                obstruction: None,
                tile_variant: None,
                text: None,
                minigame: None,
            }
        }

        // GamePos
        pub fn with_pos(mut self, pos: Vec<GamePos>) -> Params {
            self.pos = pos;
            self
        }

        pub fn add_pos(mut self, add_pos: GamePos) -> Params {
            self.pos.push(add_pos);
            self
        }

        // Obstruction
        pub fn with_obstruction(mut self, obstruction: Obstruction) -> Params {
            self.obstruction = Some(obstruction);
            self
        }

        // TileVariant
        pub fn with_tile_variant(mut self, tile_variant: TileVariant) -> Params {
            self.tile_variant = Some(tile_variant);
            self
        }

        // String
        pub fn with_text(mut self, text: String) -> Params {
            self.text = Some(text);
            self
        }

        // Minigame
        pub fn with_minigame(mut self, minigame: Box<dyn Minigame>) -> Params {
            self.minigame = Some(minigame);
            self
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
        let pos = self.get_pos();
        self.get_anim_mut().draw_overworld(
            graphics,
            manager,
            clock,
            camera.rect_from_offset(
                pos,
                (1.0, HEIGHT_GAMEPOS).into(),
                (0.0, 1.0 - HEIGHT_GAMEPOS).into(),
            ),
            Color::WHITE,
        );
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

