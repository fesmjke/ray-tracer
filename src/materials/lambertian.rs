use crate::hit::Hit;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &Hit,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        *ray_scattered = Ray::ray(hit.point, scatter_direction);
        *attenuation = self.albedo;

        return true;
    }
}
