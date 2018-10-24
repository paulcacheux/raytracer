use std::ops::Mul;

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

    pub fn apply_func<F>(&mut self, func: F) where F: Fn(u8) -> u8 {
        self.red = func(self.red);
        self.green = func(self.green);
        self.blue = func(self.blue);
    }
    
    pub fn from_vector(vector: Vector) -> Color {
        Color::from_floats(vector.x, vector.y, vector.z)
    }

    // pub fn as_vector(self) -> Vector {
    //     Vector::new(self.red as f32 / 255.0, self.green as f32 / 255.0, self.blue as f32 / 255.0)
    // }
}

impl Mul<Vector> for Color {
    type Output = Color;

    fn mul(self, other: Vector) -> Color {
        fn map(c: u8, coeff: f32) -> u8 {
            (c as f32 * coeff) as u8
        }

        Color {
            red: map(self.red, other.x),
            green: map(self.green, other.y),
            blue: map(self.blue, other.z),
        }
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