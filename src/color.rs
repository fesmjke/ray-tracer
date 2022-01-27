use std::io::{Stdout, Write};

use crate::ray::ray::{Ray};
use crate::vec3::vec3::{Color,Vec3};

pub fn write_color(out : &mut Stdout,pixel_color : Color) {

    let buf = format!("{} {} {}\n",pixel_color.r(),pixel_color.g(),pixel_color.b());

    out.write(buf.as_bytes());
}

pub fn ray_color(ray : &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t)*Color::new(1f32,1f32,1f32) + t*Color::new(0.5f32,0.7f32,1.0f32)
}