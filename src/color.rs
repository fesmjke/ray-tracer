use std::io::{Stdout, Write};

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> f32 {
    let oc: Vec3 = ray.origin() - *center;
    let a = Vec3::dot(&ray.direction(), &ray.direction());
    let b = 2.0 * Vec3::dot(&oc, &ray.direction());
    let c = Vec3::dot(&oc, &oc) - (radius * radius);
    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
        return -1.0;
    } else {
        return (-b - f32::sqrt(disc))  / (2.0 * a);
    }
}

pub fn write_color(out: &mut Stdout, pixel_color: Color) {
    let buf = format!(
        "{} {} {}\n",
        pixel_color.r(),
        pixel_color.g(),
        pixel_color.b()
    );

    out.write(buf.as_bytes()).expect("Unable to write color data!");
}

pub fn ray_color(ray: &Ray) -> Color {
    let mut transition = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    
    if transition > 0.0 {
        let temp = ray.at(transition) - Vec3::new(0.0, 0.0, -1.0);
        let n : Vec3 = Vec3::unit_vector(&temp);
        
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = Vec3::unit_vector(&ray.direction());
    transition = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - transition) * Color::new(1f32, 1f32, 1f32) + transition * Color::new(0.5f32, 0.7f32, 1.0f32)
}
