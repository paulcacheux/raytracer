use super::*;

use perlin_noise::PerlinNoise;

#[derive(Clone)]
pub struct PerlinTexture {
    perlin: PerlinNoise,
    scale: f64,
}

impl PerlinTexture {
    pub fn new(scale: f32) -> Self {
        PerlinTexture {
            perlin: PerlinNoise::new(),
            scale: scale as f64
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _: f32, _: f32, point: Point) -> Color {
        let mut color = Color::new(255, 255, 255);
        let noise = self.perlin.get3d([self.scale * point.x as f64, self.scale * point.y as f64, self.scale * point.z as f64]);
        color.apply_func(|c| (c as f64 / noise) as u8);
        color
    }
}