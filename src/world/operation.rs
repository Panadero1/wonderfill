use std::{cell::RefCell, rc::Rc};

use crate::world::{entity::Entity, minigame::Minigame, space::GamePos, tile::Obstruction};

use serde::{de::Visitor, Deserialize, Serialize};

use super::World;

pub type OpFn = Box<dyn Fn(&mut World, &Params)>;

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

    pub fn with_custom<F: Fn(&mut World, &Params) + 'static>(self, op_fn: F) -> PostOperation {
        self.op_fns.borrow_mut().push(Box::new(op_fn));
        self
    }

    /// !Warning! Be very VERY careful with this function. It can cause clumping..
    pub fn with_move_entity(
        self,
        move_pos: GamePos,
        entity_pos: GamePos,
        index: usize,
    ) -> PostOperation {
        self.with_custom(move |w, _| {
            if let Some(other) = w.mgr.get_entity(index) {
                other.moove(move_pos);
            }
        })
    }

    /// !Warning! Be very VERY careful with this function. It can cause clumping..
    pub fn with_block_entity(
        self,
        move_pos: GamePos,
        entity_pos: GamePos,
        index: usize,
    ) -> PostOperation {
        self.with_move_entity(-move_pos, entity_pos, index)
    }

    pub fn with_block_player(self, move_pos: GamePos) -> PostOperation {
        self.with_move_player(-move_pos)
    }

    pub fn with_move_player(self, move_pos: GamePos) -> PostOperation {
        self.with_custom(move |w, _p| w.player.moove(move_pos))
    }

    pub fn with_block_when<P>(self, predicate: P, move_pos: GamePos) -> PostOperation
    where
        P: 'static + Fn(&Params) -> bool,
    {
        self.with_custom(move |w, p| {
            if predicate(p) {
                w.player.moove(-move_pos)
            }
        })
    }

    pub fn with_block_when_obstructing(
        self,
        move_pos: GamePos,
        obstruction: Obstruction,
    ) -> PostOperation {
        self.with_block_when(move |_| obstruction == Obstruction::Blocking, move_pos)
    }

    pub fn with_minigame(mut self, minigame: Box<dyn Minigame>) -> PostOperation {
        // The `Some(p.minigame...unwrap())` may seem unneccessary but we want to assert that p always has a minigame
        // I could do assert! but then I'd have to make the closure multi-line and that's kinda ugly
        self.params = self.params.with_minigame(minigame);
        self.with_custom(move |w, p| w.minigame = Some(p.minigame.as_ref().unwrap().create()))
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
    pub text: Option<String>,
    pub minigame: Option<Box<dyn Minigame>>,
}

impl Clone for Params {
    fn clone(&self) -> Self {
        Params {
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
            text: None,
            minigame: None,
        }
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
