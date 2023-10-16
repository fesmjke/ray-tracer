use crate::camera::Camera;
use crate::hit::HittableList;
use crate::material::Material;
use crate::objects::sphere::Sphere;
use crate::vec3::{Color, Vec3};

mod camera;
mod color;
mod hit;
mod material;
mod objects;
mod ray;
mod vec3;

fn main() {
    let image_width = 512;

    let mut world = HittableList::new();

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let material_left = Material::Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
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
    let right_sphere = Box::new(Sphere::from(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    world.attach(central_sphere);
    world.attach(ground_sphere);
    world.attach(left_sphere);
    world.attach(right_sphere);

    let camera = Camera::new(image_width);

    camera.render(&world);
}
