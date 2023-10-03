use crate::color::world_color;
use crate::hit::HittableList;
use crate::objects::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::io::{self, Write};

mod color;
mod hit;
mod objects;
mod ray;
mod vec3;

fn main() {
    // Image settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 512;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera settings

    let viewport_height = 2.0f32;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0f32, 0f32, 0f32);
    let horizontal = Vec3::new(viewport_width, 0f32, 0f32);
    let vertical = Vec3::new(0f32, viewport_height, 0f32);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0f32, 0f32, focal_length);

    let mut world = HittableList::new();
    let first_sphere = Box::new(Sphere::from(Vec3::new(0f32, 0f32, -1f32), 0.5));
    let second_sphere = Box::new(Sphere::from(Vec3::new(0f32, -100.5f32, -1f32), 100.0));

    world.attach(first_sphere);
    world.attach(second_sphere);

    println!("P3\n {} {}\n255\n", image_width, image_height);

    // different streams
    let mut out = io::stdout(); // write a image data
    let mut outerr = io::stderr(); // write a indicator of execution

    for j in (0..image_height).rev() {
        let indicator = format!("\rScan lines remaining: {} ", j);
        outerr
            .write(indicator.as_bytes())
            .expect("Unable to write indicator data to stderr");
        out.flush().expect("Unable to flush stdout");
        for i in 0..image_width {
            let u = i as f32 / (image_width) as f32;
            let v = j as f32 / (image_height) as f32;

            let ray = Ray::ray(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let fallen_color = world_color(&ray, &world);

            let ir = (255.999 * fallen_color.r()) as i32;
            let ig = (255.999 * fallen_color.g()) as i32;
            let ib = (255.999 * fallen_color.b()) as i32;

            let pixel_color = Color::new(ir as f32, ig as f32, ib as f32);

            color::write_color(&mut out, pixel_color);
        }
    }
    outerr
        .write(b"\nDone.\n")
        .expect("Unable to write to stderr");
}
