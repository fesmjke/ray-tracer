use crate::color::Color;
use crate::lights::PointLight;
use crate::pattern::{Pattern, PatternType};
use crate::point::Point;
use crate::primitives::PrimitiveShape;
use crate::vector::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    // TODO: replace with default Pattern
    pub pattern: Pattern,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            // TODO: add restrictions for setting and creation
            // For ambient, diffuse, and specular, the typical values are between 0 and 1
            // For shininess, values between 10 (very large highlight and 200 (very small highlight) seem to work best
            ambient,
            diffuse,
            specular,
            shininess,
            pattern: Default::default(),
        }
    }

    pub fn color_reflection(
        &self,
        light: PointLight,
        primitive: PrimitiveShape,
        position: Point,
        eye_vector: Vector3,
        normal_vector: Vector3,
        in_shadow: bool,
    ) -> Color {
        let color = self.pattern.pattern_at_local(primitive, position);
        let effective_color = color * light.intensity;
        let ambient = effective_color * self.ambient;

        if in_shadow {
            ambient
        } else {
            let mut diffuse = Color::black();
            let mut specular = Color::black();

            let delta = (light.position - position).normalize();
            let light_vector = Vector3::new(delta.x, delta.y, delta.z);
            let light_dot_normal = light_vector.dot(&normal_vector);

            if light_dot_normal >= 0.0 {
                diffuse = effective_color * self.diffuse * light_dot_normal;
                let reflect_vector = (-light_vector).reflect(&normal_vector);
                let reflect_dot_eye = reflect_vector.dot(&eye_vector);

                if reflect_dot_eye > 0.0 {
                    let factor = f64::powf(reflect_dot_eye, self.shininess);
                    specular = light.intensity * self.specular * factor
                }
            }

            ambient + diffuse + specular
        }
    }

    pub fn apply_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: Default::default(),
        }
    }
}

#[cfg(test)]
mod material_tests {
    use crate::color::Color;
    use crate::lights::PointLight;
    use crate::material::Material;
    use crate::pattern::Pattern;
    use crate::point::Point;
    use crate::primitives::{PrimitiveShape, Sphere};
    use crate::vector::Vector3;

    #[test]
    fn material_default_creation() {
        let default_material = Material::default();
        let expected_color = Color::white();
        let expected_ambient = 0.1;
        let expected_diffuse = 0.9;
        let expected_specular = 0.9;
        let expected_shininess = 200.0;

        assert_eq!(expected_color, default_material.color);
        assert_eq!(expected_ambient, default_material.ambient);
        assert_eq!(expected_diffuse, default_material.diffuse);
        assert_eq!(expected_specular, default_material.specular);
        assert_eq!(expected_shininess, default_material.shininess);
    }

    #[test]
    fn material_reflects_color_between_light_and_surface() {
        let material = Material::default();
        let position = Point::default();
        let eye_vector = Vector3::new(0.0, 0.0, -1.0);
        let normal_vector = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

        let expected_color = Color::new(1.9, 1.9, 1.9);

        assert_eq!(
            expected_color,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position,
                eye_vector,
                normal_vector,
                false
            )
        );
    }

    #[test]
    fn material_reflects_color_between_light_and_surface_eye_offset_45_degree() {
        let material = Material::default();
        let position = Point::default();
        let eye_vector = Vector3::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_vector = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let in_shadow = false;
        let expected_color = Color::new(1.0, 1.0, 1.0);

        assert_eq!(
            expected_color,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position,
                eye_vector,
                normal_vector,
                in_shadow
            )
        );
    }

    #[test]
    fn material_reflects_color_between_light_and_surface_light_offset_45_degree() {
        let material = Material::default();
        let position = Point::default();
        let eye_vector = Vector3::new(0.0, 0.0, -1.0);
        let normal_vector = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let in_shadow = false;
        let expected_color = Color::new(0.7364, 0.7364, 0.7364);

        assert_eq!(
            expected_color,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position,
                eye_vector,
                normal_vector,
                in_shadow
            )
        );
    }

    #[test]
    fn material_reflects_color_between_light_and_surface_reflection() {
        let material = Material::default();
        let position = Point::default();
        let eye_vector = Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_vector = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let in_shadow = false;
        let expected_color = Color::new(1.6364, 1.6364, 1.6364);

        assert_eq!(
            expected_color,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position,
                eye_vector,
                normal_vector,
                in_shadow
            )
        );
    }

    #[test]
    fn material_reflects_color_behind_surface() {
        let material = Material::default();
        let position = Point::default();
        let eye_vector = Vector3::new(0.0, 0.0, -1.0);
        let normal_vector = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0));
        let in_shadow = false;
        let expected_color = Color::new(0.1, 0.1, 0.1);

        assert_eq!(
            expected_color,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position,
                eye_vector,
                normal_vector,
                in_shadow
            )
        );
    }

    #[test]
    fn material_color_with_surface_in_shadow() {
        let material = Material::default();
        let position = Point::default();
        let eye_vector = Vector3::new(0.0, 0.0, -1.0);
        let normal_vector = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let in_shadow = true;

        let expected_color = Color::new(0.1, 0.1, 0.1);

        assert_eq!(
            expected_color,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position,
                eye_vector,
                normal_vector,
                in_shadow
            )
        );
    }

    #[test]
    fn material_with_pattern() {
        let material = Material::default()
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .apply_pattern(Pattern::new_striped(
                Color::new(1.0, 1.0, 1.0),
                Color::new(0.0, 0.0, 0.0),
            ));
        let position_a = Point::new(0.9, 0.0, 0.0);
        let position_b = Point::new(1.1, 0.0, 0.0);
        let eye_vector = Vector3::new(0.0, 0.0, -1.0);
        let normal_vector = Vector3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let in_shadow = false;

        let expected_color_white = Color::white();
        let expected_color_black = Color::black();

        assert_eq!(
            expected_color_white,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position_a,
                eye_vector,
                normal_vector,
                in_shadow
            )
        );

        assert_eq!(
            expected_color_black,
            material.color_reflection(
                light,
                PrimitiveShape::SphereShape(Sphere::default()),
                position_b,
                eye_vector,
                normal_vector,
                in_shadow
            )
        );
    }
}
