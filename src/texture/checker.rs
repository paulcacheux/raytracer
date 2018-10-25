use super::*;

#[derive(Debug, Clone)]
pub struct CheckerTexture<Odd: Texture, Even: Texture> {
    pub odd: Odd,
    pub even: Even,
}

impl<Odd: Texture, Even: Texture> CheckerTexture<Odd, Even> {
    pub fn new(odd: Odd, even: Even) -> Self {
        CheckerTexture { odd, even }
    }
}

impl<Odd: Texture, Even: Texture> Texture for CheckerTexture<Odd, Even> {
    fn value(&self, u: f32, v: f32, point: Point) -> Color {
        let coeff = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if coeff < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
