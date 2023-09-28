use std::io::{Stdout, Write};
use crate::hit::{Hit, Hittable, HittableList};

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub fn write_color(out: &mut Stdout, pixel_color: Color) {
    let buf = format!(
        "{} {} {}\n",
        pixel_color.r(),
        pixel_color.g(),
        pixel_color.b()
    );

    out.write(buf.as_bytes()).expect("Unable to write color data!");
}

pub fn world_color(ray: &Ray, world: &HittableList) -> Color {
    let mut temp_hit = Hit::new();

    return if world.hit(&ray, 0.0, f32::MAX, &mut temp_hit) {
        0.5 * Color::new(temp_hit.normal.x() + 1.0, temp_hit.normal.y() + 1.0, temp_hit.normal.z() + 1.0)
    } else {
        let unit_direction = Vec3::unit_vector(&ray.direction());
        let transition = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - transition) * Color::new(1f32, 1f32, 1f32) + transition * Color::new(0.5f32, 0.7f32, 1.0f32)
    }
}
