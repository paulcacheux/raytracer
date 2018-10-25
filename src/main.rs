extern crate rand;
extern crate scoped_threadpool;
extern crate image;
extern crate raytracer;

use std::io;
use std::path::Path;

use rand::prelude::*;
use scoped_threadpool::Pool;

use raytracer::color::*;
use raytracer::ray_image::RayImage;
use raytracer::ray::Ray;
use raytracer::math::*;
use raytracer::hitable::*;
use raytracer::camera::Camera;
use raytracer::material::*;
use raytracer::texture::*;

const WIDTH: usize = 1600;
const HEIGHT: usize = 800;
const MAX_RAYS: usize = 200;
const MAX_DEPTH: usize = 50;
const OUT_PATH: &str = "./output_test/out1.png";

fn emit_image_to_file<P: AsRef<Path>>(path: P, image: &RayImage) -> io::Result<()> {
    let (width, height) = image.get_dimensions();

    let out_image = image::ImageBuffer::from_fn(width as _, height as _, |x, y| {
        let pixel = image.get_pixel(x as _, y as _);
        image::Rgb([pixel.red, pixel.green, pixel.blue])
    });

    out_image.save(path)
}

fn main() {
    let lookfrom = Point::new(10.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let aspect = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(lookfrom, lookat, vup, 90.0, aspect, aperture, dist_to_focus);

    // let lookfrom = Point::new(0.0, 0.0, 20.0);
    // let lookat = Point::new(0.0, 0.0, -1.0);
    // let vup = Vector::new(0.0, 1.0, 0.0);
    // let dist_to_focus = (lookfrom - lookat).norm();
    // let aperture = 0.0;
    // let aspect = WIDTH as f32 / HEIGHT as f32;
    // let camera = Camera::new(lookfrom, lookat, vup, 90.0, aspect, aperture, dist_to_focus);

    let world = random_scene();
    let bvh = BVH::new(world);

    let image = build_in_parallel(WIDTH, HEIGHT, |x, y, _| {
        let y = HEIGHT - y - 1;
        let mut rng = thread_rng();
        let dx: f32 = rng.gen();
        let dy: f32 = rng.gen();
        let u = (x as f32 + dx) / (WIDTH as f32);
        let v = (y as f32 + dy) / (HEIGHT as f32);
        let ray = camera.get_ray(u, v);
        color(ray, &bvh, 0)
    });
    
    emit_image_to_file(OUT_PATH, &image).expect("Error writing image")
}

fn color<H: Hitable>(ray: Ray, hitable: &H, depth: usize) -> Color {
    if let Some(infos) = hitable.hit(ray, 0.001, std::f32::MAX) {
        if depth < MAX_DEPTH {
            if let Some(mat_infos) = infos.material.scatter(ray, &infos) {
                return color(mat_infos.scattered, hitable, depth + 1) * mat_infos.attenuation;
            }
        }
        Color::new(0, 0, 0)
    } else {
        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        let v = Vector::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
        Color::from_vector(v)
    }    
}

fn build_in_parallel<F>(width: usize, height: usize, pixel_func: F) -> RayImage
    where F: Send + Copy + Fn(usize, usize, usize) -> Color
{
    let mut image = RayImage::new(width, height);
    let mut pool = Pool::new(4);

    pool.scoped(|scoped| {
        for (x, y, pixel) in image.pixel_mut_iter() {
            scoped.execute(move || {
                let mut avger = ColorAverager::new();
                for n in 0..MAX_RAYS {
                    let color = pixel_func(x, y, n);
                    avger.add(color);
                }
                let mut final_color = avger.average();
                final_color.apply_func(|c| ((c as f64 / 255.0).sqrt() * 255.0) as u8);
                *pixel = final_color;
            })
        }
    });

    image
}

fn lambertian_from_float_comp(red: f32, green: f32, blue: f32) -> impl Material {
    let color = Color::from_floats(red, green, blue);
    let texture: ConstantTexture = color.into();
    Lambertian::new(texture)
}

#[allow(dead_code)]
fn one_perlin_sphere_scene() -> Vec<Box<dyn Hitable>> {
    vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 10.0, Lambertian::new(PerlinTexture::new(10.0)))),
    ]
}

#[allow(dead_code)]
fn three_sphere_scene() -> Vec<Box<dyn Hitable>> {
    // let lookfrom = Point::new(13.0, 2.0, 3.0);
    // let lookat = Point::new(0.0, 0.0, 0.0);
    // let vup = Vector::new(0.0, 1.0, 0.0);
    // let dist_to_focus = 10.0;
    // let aperture = 0.0;
    // let aspect = WIDTH as f32 / HEIGHT as f32;
    // let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect, aperture, dist_to_focus);

    vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, lambertian_from_float_comp(0.1, 0.2, 0.5))),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, lambertian_from_float_comp(0.8, 0.8, 0.0))),
        Box::new(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, Metal::new(Vector::new(0.8, 0.6, 0.2), 0.0))),
        Box::new(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5))),
        Box::new(Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.45, Dielectric::new(1.5))),
    ]
}

#[allow(dead_code)]
fn two_checker_sphere() -> Vec<Box<dyn Hitable>> {
    let odd_texture: ConstantTexture = Color::from_floats(0.2, 0.3, 0.1).into();
    let even_texture: ConstantTexture = Color::from_floats(0.9, 0.9, 0.9).into();
    let checker_texture = CheckerTexture::new(odd_texture, even_texture);

    vec![
        Box::new(Sphere::new(Point::new(0.0, -10.0, 0.0), 10.0, Lambertian::new(checker_texture.clone()))),
        Box::new(Sphere::new(Point::new(0.0, 10.0, 0.0), 10.0, Lambertian::new(checker_texture)))
    ]
}

#[allow(dead_code)]
fn random_scene() -> Vec<Box<dyn Hitable>> {
    let mut spheres: Vec<Box<dyn Hitable>> = Vec::new();

    let odd_texture: ConstantTexture = Color::from_floats(0.2, 0.3, 0.1).into();
    let even_texture: ConstantTexture = Color::from_floats(0.9, 0.9, 0.9).into();
    let checker_texture = CheckerTexture::new(odd_texture, even_texture);

    spheres.push(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(checker_texture)
    )));

    let mut rng = rand::thread_rng();
    let mut rand_f32 = || rng.gen::<f32>();

    for a in 0..22 {
        let a = (a as f32) - 11.0;
        for b in 0..22 {
            let b = (b as f32) - 11.0;

            let center = Point::new(a + 0.9 * rand_f32(), 0.2, b + 0.9 * rand_f32());
            if (center.as_vector() - Vector::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let mat_choose = rand_f32();

                if mat_choose < 0.8 {
                    spheres.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        lambertian_from_float_comp(rand_f32() * rand_f32(), rand_f32() * rand_f32(), rand_f32() * rand_f32())
                    )));
                } else if mat_choose < 0.95 {
                    spheres.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Metal::new(Vector::new(0.5 * (1.0 + rand_f32()), 0.5 * (1.0 + rand_f32()), 0.5 * (1.0 + rand_f32())), 0.0)
                    )));
                } else {
                    spheres.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Dielectric::new(1.5)
                    )));
                }
            }
        }
    }

    spheres.push(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5)
    )));

    spheres.push(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        lambertian_from_float_comp(0.4, 0.2, 0.1)
    )));

    spheres.push(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0)
    )));

    spheres
}