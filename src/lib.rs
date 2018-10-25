extern crate rand;
extern crate perlin_noise;

pub mod color;
pub mod ray_image;
pub mod math;
pub mod ray;
pub mod hitable;
pub mod camera;
pub mod material;
pub mod texture;

pub mod prelude {
    pub use super::color::Color;
    pub use super::math::*;
    pub use super::ray::Ray;
    pub use super::hitable::Hitable;
    pub use super::material::Material;
    pub use super::texture::Texture;
}