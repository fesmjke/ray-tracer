use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Scattered {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &Hit,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool;
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

impl Material {
    pub fn new() -> Self {
        Self::Lambertian {
            albedo: Color::empty_new(),
        }
    }
}

impl Scattered for Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &Hit,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        return match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit.normal + Vec3::random_unit();

                if scatter_direction.near_zero() {
                    scatter_direction = hit.normal;
                }

                *ray_scattered = Ray::ray(hit.point, scatter_direction);
                *attenuation = *albedo;

                true
            }
            Material::Metal { albedo } => {
                let reflected_ray =
                    Vec3::reflect(&Vec3::unit_vector(&ray_in.direction()), &hit.normal);
                *ray_scattered = Ray::ray(hit.point, reflected_ray);
                *attenuation = *albedo;

                true
            }
            _ => false,
        };
    }
}
