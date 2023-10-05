use crate::camera::Camera;
use crate::hit::HittableList;
use crate::objects::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod color;
mod hit;
mod objects;
mod ray;
mod vec3;

fn main() {
    let image_width = 512;

    let mut world = HittableList::new();
    let first_sphere = Box::new(Sphere::from(Vec3::new(0f32, 0f32, -1f32), 0.5));
    let second_sphere = Box::new(Sphere::from(Vec3::new(0f32, -100.5f32, -1f32), 100.0));

    world.attach(first_sphere);
    world.attach(second_sphere);

    let camera = Camera::new(image_width);

    camera.render(&world);
}
