use super::*;

#[derive(Debug, Clone)]
pub struct ConstantTexture {
    pub color: Color,
}

impl ConstantTexture {
    pub fn new(color: Color) -> ConstantTexture {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: f32, _: f32, _: Point) -> Color {
        self.color
    }
}

impl From<Color> for ConstantTexture {
    fn from(color: Color) -> ConstantTexture {
        ConstantTexture::new(color)
    }
}
