use crate::math::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn map_float_u8(f: f32) -> u8 {
    (f * 255.0) as u8 
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn from_floats(red: f32, green: f32, blue: f32) -> Color {
        Color::new(map_float_u8(red), map_float_u8(green), map_float_u8(blue))
    }
    
    pub fn from_vector(vector: Vector) -> Color {
        Color::from_floats(vector.x, vector.y, vector.z)
    }
}

#[derive(Debug, Clone)]
pub struct ColorAverager {
    red: u64,
    green: u64,
    blue: u64,
    counter: u64,
}

impl ColorAverager {
    pub fn new() -> ColorAverager {
        ColorAverager {
            red: 0,
            green: 0,
            blue: 0,
            counter: 0,
        }
    }

    pub fn average(&self) -> Color {
        fn map(c: u64, t: u64) -> u8 {
            (c / t) as u8
        }
 
        Color {
            red: map(self.red, self.counter),
            green: map(self.green, self.counter),
            blue: map(self.blue, self.counter),
        }
    }

    pub fn add(&mut self, color: Color) {
        self.red += color.red as u64;
        self.green += color.green as u64;
        self.blue += color.blue as u64;
        self.counter += 1;
    }
}