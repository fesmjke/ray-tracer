use crate::color;
use crate::hit::{Hit, Hittable, HittableList};
use crate::material::Scattered;
use crate::vec3::{Color, Point3, Vec3};

use crate::ray::Ray;
use rand::Rng;
use std::io::{self, Write};

pub struct Camera {
    // Image settings
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,

    // Antialiasing
    samples_per_pixel: i32,

    // Rays depth
    depth: u32,

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

        // Antialiasing
        let samples_per_pixel = 10;

        // Ray depth
        let depth = 10;

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
            samples_per_pixel,
            depth,
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

        let mut generator = rand::thread_rng();

        for j in (0..self.image_height).rev() {
            let indicator = format!("\rScan lines remaining: {} ", j);
            outerr
                .write(indicator.as_bytes())
                .expect("Unable to write indicator data to stderr");
            out.flush().expect("Unable to flush stdout");
            for i in 0..self.image_width {
                let mut fallen_color = Vec3::empty_new();

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + generator.gen::<f32>()) / (self.image_width) as f32;
                    let v = (j as f32 + generator.gen::<f32>()) / (self.image_height) as f32;

                    let ray = self.get_ray(u, v);

                    fallen_color = fallen_color + self.world_color(&ray, &world, self.depth);
                }

                fallen_color = fallen_color * (1.0 / self.samples_per_pixel as f32);

                let ir = (255.999 * fallen_color.r().sqrt().clamp(0.000, 0.999)) as i32;
                let ig = (255.999 * fallen_color.g().sqrt().clamp(0.000, 0.999)) as i32;
                let ib = (255.999 * fallen_color.b().sqrt().clamp(0.000, 0.999)) as i32;

                let pixel_color = Color::new(ir as f32, ig as f32, ib as f32);

                color::write_color(&mut out, pixel_color);
            }
        }
        outerr
            .write(b"\nDone.\n")
            .expect("Unable to write to stderr");
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::ray(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    fn world_color(&self, ray: &Ray, world: &HittableList, depth: u32) -> Color {
        let mut temp_hit = Hit::new();

        if depth <= 0 {
            return Color::empty_new();
        }

        return if world.hit(&ray, 0.001, f32::MAX, &mut temp_hit) {
            let mut scattered = Ray::new_empty();
            let mut attenuation: Color = Color::new(0.0, 0.0, 0.0);

            if temp_hit
                .material
                .scatter(&ray, &temp_hit, &mut attenuation, &mut scattered)
            {
                return attenuation * self.world_color(&scattered, &world, depth - 1);
            }

            // let direction = temp_hit.normal + Vec3::random_unit();

            // 0.1 * self.world_color(&Ray::ray(temp_hit.point, direction), world, depth - 1)

            Color::new(0.0, 0.0, 0.0)
        } else {
            let unit_direction = Vec3::unit_vector(&ray.direction());
            let transition = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - transition) * Color::new(1f32, 1f32, 1f32)
                + transition * Color::new(0.5f32, 0.7f32, 1.0f32)
        };
    }
}
