use crate::camera::Camera;
use crate::hit::HittableList;
use crate::material::Material;
use crate::objects::sphere::Sphere;
use crate::preset::{parse_preset, Preset};
use crate::vec3::{Color, Vec3};
use std::env;
use std::f32::consts::PI;

mod camera;
mod color;
mod hit;
mod material;
mod objects;
mod preset;
mod ray;
mod utils;
mod vec3;

fn main() {
    let image_width = 512;

    let parsed = parse_preset(env::args());

    let mut world = HittableList::new();

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };
    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Material::Dielectric {
        index_of_refraction: 1.5,
    };

    let ground_sphere = Box::new(Sphere::from(
        Vec3::new(0f32, -100.5f32, -1f32),
        100.0,
        material_ground,
    ));

    let central_sphere = Box::new(Sphere::from(
        Vec3::new(0f32, 0f32, -1f32),
        0.5,
        material_center,
    ));
    let left_sphere = Box::new(Sphere::from(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    let inner_left_sphere = Box::new(Sphere::from(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    ));
    let right_sphere = Box::new(Sphere::from(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    world.attach(central_sphere);
    world.attach(ground_sphere);
    world.attach(left_sphere);
    world.attach(inner_left_sphere);
    world.attach(right_sphere);

    let mut camera = Camera::new(image_width);

    if parsed.is_some() {
        match parsed.unwrap() {
            Preset::Fast {
                samples_per_pixel,
                depth,
            } => {
                camera.set_samples_per_pixel(samples_per_pixel);
                camera.set_depth(depth);
            }
            Preset::Slow {
                samples_per_pixel,
                depth,
            } => {
                camera.set_samples_per_pixel(samples_per_pixel);
                camera.set_depth(depth);
            }
        }
    }

    camera.render(&world);
}
