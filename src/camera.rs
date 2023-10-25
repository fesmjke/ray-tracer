use crate::color;
use crate::hit::{Hit, Hittable, HittableList};
use crate::material::Scattered;
use crate::vec3::{Color, Point3, Vec3};

use crate::ray::Ray;
use crate::utils::degrees_to_radians;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::io::{self, Write};

pub struct Camera {
    // Image settings
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,

    // FOV
    vfov: f32,

    // Camera orientation
    look_from: Point3,
    look_at: Point3,
    up_vector: Vec3,

    // Camera basis
    w: Vec3,
    u: Vec3,
    v: Vec3,

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
    zero_pixel: Point3,
    delta_horizontal: Point3,
    delta_vertical: Point3,
}

impl Camera {
    pub fn new(image_width: i32) -> Self {
        // Image settings
        let aspect_ratio = 16.0 / 9.0;
        let image_height = (image_width as f32 / aspect_ratio) as i32;

        // FOV
        let fov = 20.0;

        let theta = degrees_to_radians(fov);
        let h = (theta / 2.0).tan();

        // Camera orientation
        let look_from = Point3::new(-2.0, 2.0, 1.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);
        let up_vector = Vec3::new(0.0, 1.0, 0.0);

        // Camera settings
        let focal_length = (look_from - look_at).length();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = aspect_ratio * viewport_height;

        // Camera basis
        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross_product(&up_vector, &w));
        let v = Vec3::cross_product(&w, &u);

        // Antialiasing
        let samples_per_pixel = 10;

        // Ray depth
        let depth = 10;

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * (-v);

        let delta_horizontal = horizontal / image_width as f32;
        let delta_vertical = vertical / image_height as f32;

        let viewport_upper_left = origin - (focal_length * w) - horizontal / 2.0 - vertical / 2.0;
        let lower_left_corner = viewport_upper_left + 0.5 * (delta_horizontal + delta_vertical);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            vfov: fov,
            look_from,
            look_at,
            up_vector,
            w,
            u,
            v,
            viewport_height,
            viewport_width,
            focal_length,
            samples_per_pixel,
            depth,
            origin,
            horizontal,
            vertical,
            delta_horizontal,
            delta_vertical,
            zero_pixel: lower_left_corner,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n {} {}\n255\n", self.image_width, self.image_height);

        // different streams
        let mut out = io::stdout(); // write a image data
        let mut outerr = io::stderr(); // write a indicator of execution

        let mut generator = rand::thread_rng();

        for j in 0..self.image_height {
            let indicator = format!("\rScan lines remaining: {} ", self.image_height - j - 1);
            outerr
                .write(indicator.as_bytes())
                .expect("Unable to write indicator data to stderr");
            out.flush().expect("Unable to flush stdout");
            for i in 0..self.image_width {
                let mut fallen_color = Vec3::empty_new();

                for _ in 0..self.samples_per_pixel {
                    let px = generator.gen::<f32>();
                    let py = generator.gen::<f32>();

                    let ray = self.get_ray(i, j, px, py);

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

    fn get_ray(&self, i: i32, j: i32, rd_px: f32, rd_py: f32) -> Ray {
        let pixel_center =
            self.zero_pixel + (i as f32 * self.delta_horizontal) + (j as f32 * self.delta_vertical);
        let pixel_sample = pixel_center + self.pixel_sample_square(rd_px, rd_py);

        let ray_origin = self.origin;
        let ray_direction = pixel_sample - ray_origin;

        Ray::ray(self.origin, ray_direction)
    }

    fn pixel_sample_square(&self, rd_px: f32, rd_py: f32) -> Vec3 {
        let px = -0.5 + rd_px;
        let py = -0.5 + rd_py;

        (px * self.delta_horizontal) + (py * self.delta_vertical)
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

    pub fn set_samples_per_pixel(&mut self, samples_per_pixel: i32) {
        self.samples_per_pixel = samples_per_pixel;
    }

    pub fn set_depth(&mut self, depth: u32) {
        self.depth = depth;
    }
}
