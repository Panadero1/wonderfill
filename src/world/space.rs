use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
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
