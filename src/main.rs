use crate::camera::Camera;
use crate::hit::HittableList;
use crate::material::Material;
use crate::objects::sphere::Sphere;
use crate::objects::triangle::Triangle;
use crate::preset::{parse_preset, Preset};
use crate::vec3::{Color, Vec3};
use rand::thread_rng;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::Write;

mod camera;
mod color;
mod hit;
mod material;
mod objects;
mod preset;
mod ray;
mod utils;
mod vec3;

// TODO:remove later or move to separate file

fn write_image(pixels: &[u8], image_width: usize, image_height: usize) {
    let file = File::create("image.ppm");

    match file {
        Ok(mut f) => {
            f.write(format!("P3\n {} {}\n255\n", image_width, image_height).as_bytes())
                .expect("unable to write to file!");

            for band in pixels.chunks(3) {
                let pixel_color = Color::new(band[0] as f32, band[1] as f32, band[2] as f32);

                let buf = format!(
                    "{} {} {}\n",
                    pixel_color.r(),
                    pixel_color.g(),
                    pixel_color.b()
                );

                f.write(buf.as_bytes()).expect("unable to write to file!");
            }
        }

        Err(_) => {
            println!("unable to create file!");
        }
    };
}

fn main() {
    let image_width = 512;

    let parsed = parse_preset(env::args());

    let mut world = HittableList::new();

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };

    let ground = Box::from(Sphere::from(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    world.attach(ground);

    let mut rng = thread_rng();

    // for i in -11..11 {
    //     for j in -11..11 {
    //         let choose: f32 = rng.gen();
    //         let center = Point3::new(
    //             i as f32 + 0.9 * rng.gen::<f32>(),
    //             0.2,
    //             j as f32 + rng.gen::<f32>(),
    //         );
    //
    //         if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             let mut sphere_material = Material::new();
    //
    //             if choose < 0.8 {
    //                 let albedo = Color::random_vector() * Color::random_vector();
    //                 sphere_material = Material::Lambertian { albedo };
    //                 world.attach(Box::from(Sphere::from(center, 0.2, sphere_material)));
    //             } else if choose < 0.95 {
    //                 let albedo = Color::random_vector_mm(0.5, 1.0);
    //                 let fuzz = rng.gen_range(0.0..=0.5);
    //                 sphere_material = Material::Metal { albedo, fuzz };
    //                 world.attach(Box::from(Sphere::from(center, 0.2, sphere_material)));
    //             } else {
    //                 sphere_material = Material::Dielectric {
    //                     index_of_refraction: 1.5,
    //                 };
    //                 world.attach(Box::from(Sphere::from(center, 0.2, sphere_material)));
    //             }
    //         }
    //     }
    // }

    let dielectric_material = Material::Dielectric {
        index_of_refraction: 1.5,
    };
    let lambertian_material = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    let metal_material = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    // world.attach(Box::from(Sphere::from(
    //     Vec3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     dielectric_material,
    // )));

    world.attach(Box::from(Sphere::from(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        lambertian_material,
    )));

    world.attach(Box::from(Triangle::from(
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, -1.0),
        lambertian_material,
    )));

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

    let image_width = image_width;

    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let mut pixels = vec![0; (image_width * image_height * 3) as usize];
    let bands: Vec<(usize, &mut [u8])> = pixels
        .chunks_mut((image_width * 3) as usize)
        .enumerate()
        .collect();

    let start = std::time::Instant::now();

    bands.into_par_iter().rev().for_each(|(i, band)| {
        camera.render_parallel(&world, band, i as i32);
    });

    println!("Frame time: {}ms", start.elapsed().as_millis());

    write_image(&pixels, image_width as usize, image_height as usize);
}
