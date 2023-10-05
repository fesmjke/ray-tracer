use crate::color;
use crate::hit::{Hit, Hittable, HittableList};
use crate::vec3::{Color, Point3, Vec3};

use crate::ray::Ray;
use std::io::{self, Write};

pub struct Camera {
    // Image settings
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,

    // Camera settings
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,

    origin: Point3,
    horizontal: Point3,
    vertical: Point3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(image_width: i32) -> Self {
        // Image settings
        let aspect_ratio = 16.0 / 9.0;
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

        Self {
            aspect_ratio,
            image_width,
            image_height,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n {} {}\n255\n", self.image_width, self.image_height);

        // different streams
        let mut out = io::stdout(); // write a image data
        let mut outerr = io::stderr(); // write a indicator of execution

        for j in (0..self.image_height).rev() {
            let indicator = format!("\rScan lines remaining: {} ", j);
            outerr
                .write(indicator.as_bytes())
                .expect("Unable to write indicator data to stderr");
            out.flush().expect("Unable to flush stdout");
            for i in 0..self.image_width {
                let u = i as f32 / (self.image_width) as f32;
                let v = j as f32 / (self.image_height) as f32;

                let ray = Ray::ray(
                    self.origin,
                    self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
                );
                let fallen_color = self.world_color(&ray, &world);

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

    fn world_color(&self, ray: &Ray, world: &HittableList) -> Color {
        let mut temp_hit = Hit::new();

        return if world.hit(&ray, 0.0, f32::MAX, &mut temp_hit) {
            0.5 * Color::new(
                temp_hit.normal.x() + 1.0,
                temp_hit.normal.y() + 1.0,
                temp_hit.normal.z() + 1.0,
            )
        } else {
            let unit_direction = Vec3::unit_vector(&ray.direction());
            let transition = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - transition) * Color::new(1f32, 1f32, 1f32)
                + transition * Color::new(0.5f32, 0.7f32, 1.0f32)
        };
    }
}