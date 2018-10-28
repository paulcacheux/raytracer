use std::path::Path;
use std::io::{self, prelude::*};
use std::fs::File;
use std::sync::Arc;

use raytracer::prelude::*;
use raytracer::hitable::Triangle;
use raytracer::material::Lambertian;

pub fn read_obj_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Box<dyn Hitable>>> {
    let file = File::open(path)?;
    let buf_reader = io::BufReader::new(file);

    let mut points = Vec::new();
    let mut triangles: Vec<Box<dyn Hitable>> = Vec::new();
    let texture: raytracer::texture::ConstantTexture = Color::from_floats(0.9, 0.2, 0.1).into();
    let material: Arc<dyn Material> = Arc::new(Lambertian::new(texture));

    for line in buf_reader.lines() {
        match parse_line(line?) {
            Some(LineItem::Vertex(p)) => points.push(p),
            Some(LineItem::Triangle(a, b, c)) => {
                let p0 = points[a - 1];
                let p1 = points[b - 1];
                let p2 = points[c - 1];

                let t0 = Triangle::new_with_arc(p0, p1, p2, material.clone());

                triangles.push(Box::new(t0));
            }
            Some(LineItem::Quad(a, b, c, d)) => {
                let p0 = points[a - 1];
                let p1 = points[b - 1];
                let p2 = points[c - 1];
                let p3 = points[d - 1];

                let t0 = Triangle::new_with_arc(p0, p1, p2, material.clone());
                let t1 = Triangle::new_with_arc(p0, p2, p3, material.clone());

                triangles.push(Box::new(t0));
                triangles.push(Box::new(t1));
            },
            _ => {}
        }
    }

    Ok(triangles)
}

enum LineItem {
    Vertex(Point),
    Triangle(usize, usize, usize),
    Quad(usize, usize, usize, usize),
}

fn parse_line(line: String) -> Option<LineItem> {
    if let Some(index) = line.find("v ") {
        let rest = &line[(index + 2)..];
        let c: Vec<f32> = rest.split(' ')
            .filter(|a| !a.trim().is_empty())
            .take(3)
            .map(|a| a.parse().unwrap())
            .collect();
        let point = Point::new(c[0], c[1], c[2]);
        Some(LineItem::Vertex(point))
    } else if let Some(index) = line.find("f ") {
        let rest = &line[(index + 2)..];
        let c: Vec<usize> = rest.split(' ')
            .filter(|a| !a.trim().is_empty())
            .map(|a| a.trim().split('/').next().unwrap())
            .map(|a| a.parse().unwrap())
            .collect();

        if c.len() == 3 {
            Some(LineItem::Triangle(c[0], c[1], c[2]))
        } else if c.len() == 4 {
            Some(LineItem::Quad(c[0], c[1], c[2], c[3]))
        } else {
            unimplemented!()
        }
    } else {
        None
    }
}