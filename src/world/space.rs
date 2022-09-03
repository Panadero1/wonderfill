use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// The height of a wall or other standard extended sprite. Measured in tiles, not pixels
pub const SPRITE_EXTENSION_HEIGHT: f32 = 1.0 / 0.7;

// GamePos
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct GamePos {
    pub x: f32,
    pub y: f32,
}

impl Add for GamePos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}
impl AddAssign for GamePos {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}
impl Sub for GamePos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}
impl SubAssign for GamePos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}
impl Mul<f32> for GamePos {
    type Output = GamePos;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.x * rhs, self.y * rhs).into()
    }
}
impl MulAssign<f32> for GamePos {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs);
    }
}
impl Div<f32> for GamePos {
    type Output = GamePos;

    fn div(self, rhs: f32) -> Self::Output {
        (self.x / rhs, self.y / rhs).into()
    }
}
impl DivAssign<f32> for GamePos {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div(rhs);
    }
}
impl Into<(f32, f32)> for GamePos {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}
impl From<(f32, f32)> for GamePos {
    fn from(pos: (f32, f32)) -> Self {
        GamePos { x: pos.0, y: pos.1 }
    }
}
impl From<(i32, i32)> for GamePos {
    fn from(pos: (i32, i32)) -> Self {
        GamePos {
            x: pos.0 as f32,
            y: pos.1 as f32,
        }
    }
}
impl Neg for GamePos {
    type Output = GamePos;

    fn neg(self) -> Self::Output {
        (-self.x, -self.y).into()
    }
}

impl GamePos {
    pub fn origin() -> GamePos {
        GamePos { x: 0.0, y: 0.0 }
    }
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn largest_component_difference(&self, rhs: GamePos) -> f32 {
        let dif = rhs.sub(*self);
        dif.x.max(dif.y)
    }
    pub fn abs(&self) -> GamePos {
        (self.x.abs(), self.y.abs()).into()
    }
    pub fn floor(self) -> GamePos {
        (self.x.floor(), self.y.floor()).into()
    }
    pub fn round(self) -> GamePos {
        // Need this bc Rust can't infer type :((
        let result: GamePos = (self.x + 0.5, self.y + 0.5).into();
        result.floor()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Direction {
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
impl Direction {
    pub fn rotate_cw(&mut self) {
        use Direction::*;
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
        use Direction::*;
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
        use Direction::*;
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
impl From<GamePos> for Direction {
    fn from(dir: GamePos) -> Self {
        let unit_dir = (
            if dir.x > 0. {
                1
            } else if dir.x < 0. {
                -1
            } else {
                0
            },
            if dir.y > 0. {
                1
            } else if dir.y < 0. {
                -1
            } else {
                0
            },
        );
        use Direction::*;
        match unit_dir {
            (-1, 0) => Left,
            (1, 0) => Right,
            (0, -1) => Top,
            (0, 1) => Bottom,
            (-1, 1) => CornerBL,
            (1, 1) => CornerBR,
            (1, -1) => CornerTR,
            (-1, -1) => CornerTL,
            (0, 0) => Center,
            _ => panic!("Not expected unit direction: {:?}", unit_dir),
        }
    }
}
