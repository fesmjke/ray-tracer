use crate::vec3::Color;
use crate::vector::Vector3;
use rand::{random, thread_rng, Rng};
use std::f32::consts::PI;
use std::io::Stdout;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

/// Generate a random vector in range (0.0, 1.0]
pub fn random_vector() -> Vector3 {
    Vector3::new(random::<f64>(), random::<f64>(), random::<f64>())
}

/// Generate a random vector in range [min, max]
pub fn random_vector_mm(min: f64, max: f64) -> Vector3 {
    let mut rng = thread_rng();

    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}
pub fn random_in_unit_disk() -> Vector3 {
    let mut rng = thread_rng();

    loop {
        let rx = rng.gen_range(-1.0..=1.00);
        let ry = rng.gen_range(-1.0..=1.00);
        let r_point = Vector3::new(rx, ry, 0.0);

        if r_point.magnitude() < 1.0 {
            return r_point;
        }
    }
}

pub fn random_unit() -> Vector3 {
    Vector3::normalize(&random_in_unit_disk())
}

pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
    let on_unit_sphere = random_unit();

    return if on_unit_sphere.dot(&normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    };
}

pub fn write_color(out: &mut Stdout, pixel_color: Color) {
    let buf = format!(
        "{} {} {}\n",
        pixel_color.r(),
        pixel_color.g(),
        pixel_color.b()
    );

    out.write(buf.as_bytes())
        .expect("Unable to write color data!");
}
