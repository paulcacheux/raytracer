use std::fmt;

use crate::color::*;

#[derive(Debug, Clone)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![Color::new(0, 0, 0); width * height]
        }
    }

    fn assert_coord_in_range(&self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.assert_coord_in_range(x, y);
        self.pixels[y * self.width + x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.assert_coord_in_range(x, y);
        self.pixels[y * self.width + x] = color;
    }
}

impl fmt::Display for Image {
    // we display the ppm version
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[y * self.width + x];
                write!(f, "{} {} {}\n", color.red, color.green, color.blue)?;
            }
        }
        Ok(())
    }
}