use std::fmt;
use std::marker::PhantomData;

use crate::color::*;

#[derive(Debug, Clone)]
pub struct RayImage {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl RayImage {
    pub fn new(width: usize, height: usize) -> RayImage {
        RayImage {
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

    // pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
    //     self.assert_coord_in_range(x, y);
    //     self.pixels[y * self.width + x] = color;
    // }

    pub fn pixel_mut_iter<'a>(&'a mut self) -> RayImagePixelIter<'a> {
        RayImagePixelIter {
            image_start: self.pixels.as_mut_ptr(),
            current: 0,
            width: self.width,
            height: self.height,
            _marker: PhantomData,
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl fmt::Display for RayImage {
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

pub struct RayImagePixelIter<'a> {
    image_start: *mut Color,
    current: isize,
    width: usize,
    height: usize,
    _marker: PhantomData<&'a ()>
}

impl<'a> Iterator for RayImagePixelIter<'a> {
    type Item = (usize, usize, &'a mut Color);

    fn next(&mut self) -> Option<Self::Item> {
        let current_u = self.current as usize;
        if current_u >= self.width * self.height {
            return None;
        }

        let y = current_u / self.width;
        let x = current_u % self.width;
        
        let pixel_ref = unsafe {
            let pixel_ref: &'a mut Color = &mut *self.image_start.offset(self.current);
            self.current += 1;
            pixel_ref
        };

        Some((x, y, pixel_ref))
    }
}