extern crate rand;
extern crate rayon;

use std::fs::File;
use std::io;
use std::path::Path;
use std::io::prelude::*;
use std::sync::mpsc::channel;

use rand::prelude::*;
use rayon::prelude::*;

mod color;
mod image;
mod math;
mod ray;
mod hitable;
mod camera;

use self::color::*;
use self::image::Image;
use self::ray::Ray;
use self::math::*;
use self::hitable::*;
use self::camera::Camera;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const MAX_RAYS: usize = 500;
const OUT_PATH: &str = "./output_test/out1.ppm";

fn emit_image_to_file<P: AsRef<Path>>(path: P, image: &Image) -> io::Result<()> {
    let mut file = File::create(path)?;
    write!(&mut file, "{}", image)
}

fn main() {
    let lower_left_corner = Point::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 2.0, 0.0);
    let origin = Point::origin();

    let camera = Camera {
        lower_left_corner,
        origin,
        horizontal,
        vertical
    };

    let spheres = vec![
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0),
    ];

    let image = build_in_parallel(WIDTH, HEIGHT, |x, y, _| {
        let mut rng = thread_rng();
        let dx: f32 = rng.gen();
        let dy: f32 = rng.gen();
        let u = (x as f32 + dx) / (WIDTH as f32);
        let v = (y as f32 + dy) / (HEIGHT as f32);
        let ray = camera.get_ray(u, v);
        color(ray, &spheres)
    });
    
    emit_image_to_file(OUT_PATH, &image).expect("Error writing image")
}

fn color<H: Hitable>(ray: Ray, hitable: &H) -> Color {
    if let Some(infos) = hitable.hit(ray, 0.0, std::f32::MAX) {
        let n = infos.normal;
        let c = Vector::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
        Color::from_vector(c)
    } else {
        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        let v = Vector::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
        Color::from_vector(v)
    }    
}

/* fn build_in_sequence<F>(width: usize, height: usize, pixel_func: F) -> Image
    where F: Fn(usize, usize, usize) -> Color
{
    let mut image = Image::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut avger = ColorAverager::new();
            for n in 0..MAX_RAYS {
                let color = pixel_func(x, y, n);
                avger.add(color);
            }
            image.set_pixel(x, height - y - 1, avger.average());
        }
    }
    image
} */

fn build_in_parallel<F>(width: usize, height: usize, pixel_func: F) -> Image
    where F: Sync + Fn(usize, usize, usize) -> Color
{
    let xy_iter = (0..height).into_par_iter().flat_map(|y| (0..width).into_par_iter().map(move |x| (x, y)));
    let (sender, receiver) = channel();

    xy_iter.for_each_with(sender, |sender, (x, y)| {
        let mut avger = ColorAverager::new();
        for n in 0..MAX_RAYS {
            let color = pixel_func(x, y, n);
            avger.add(color);
        }
        let final_color = avger.average();
        sender.send((x, y, final_color)).unwrap();
    });

    let mut image = Image::new(width, height);
    for (x, y, color) in receiver.iter() {
        image.set_pixel(x, height - y - 1, color);
    }

    image
}