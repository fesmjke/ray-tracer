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
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { index_of_refraction: f32 },
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
            Material::Metal { albedo, fuzz } => {
                let reflected_ray =
                    Vec3::reflect(&Vec3::unit_vector(&ray_in.direction()), &hit.normal);
                *ray_scattered = Ray::ray(hit.point, reflected_ray + Vec3::random_unit() * *fuzz);
                *attenuation = *albedo;

                return Vec3::dot(&ray_scattered.direction(), &hit.normal) > 0.0;
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);

                let refraction_ratio = if hit.front_face {
                    1.0 / (*index_of_refraction)
                } else {
                    *index_of_refraction
                };

                let unit_direction = Vec3::unit_vector(&ray_in.direction());
                let refracted = Vec3::refract(&unit_direction, &hit.normal, refraction_ratio);

                *ray_scattered = Ray::ray(hit.point, refracted);

                return true;
            }
        };
    }
}
