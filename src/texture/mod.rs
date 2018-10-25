use crate::prelude::*;

pub mod checker;
pub mod constant;
pub mod perlin;
pub use self::checker::*;
pub use self::constant::*;
pub use self::perlin::*;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, point: Point) -> Color;
}