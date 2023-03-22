use std::io::{Stdout, Write};

use crate::ray::ray::Ray;
use crate::vec3::vec3::{Color, Point3, Vec3};

pub fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin() - *center;
    let a = Vec3::dot(&ray.direction(), &ray.direction());
    let b = 2.0 * Vec3::dot(&oc, &ray.direction());
    let c = Vec3::dot(&oc, &oc) - (radius * radius);
    let disc = b * b - 4.0 * a * c;

    return disc > 0.0;
}

pub fn write_color(out: &mut Stdout, pixel_color: Color) {
    let buf = format!(
        "{} {} {}\n",
        pixel_color.r(),
        pixel_color.g(),
        pixel_color.b()
    );

    out.write(buf.as_bytes());
}

pub fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1f32, 1f32, 1f32) + t * Color::new(0.5f32, 0.7f32, 1.0f32)
}
