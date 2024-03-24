use crate::color::Color;
use crate::point::Point;

#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
}

impl StripePattern {
    pub fn from(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[cfg(test)]
mod stripe_pattern_tests {
    use crate::color::Color;
    use crate::material::Material;
    use crate::patterns::pattern::Pattern;
    use crate::point::Point;
    use crate::primitives::{PrimitiveShape, Sphere};
    use crate::transformations::Transformable;

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = Pattern::new_stripe(Color::white(), Color::black());
        let expected_color = Color::white();

        assert_eq!(
            expected_color,
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            expected_color,
            pattern.pattern_at(Point::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            expected_color,
            pattern.pattern_at(Point::new(0.0, 2.0, 0.0))
        );
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = Pattern::new_stripe(Color::white(), Color::black());
        let expected_color = Color::white();

        assert_eq!(
            expected_color,
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            expected_color,
            pattern.pattern_at(Point::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            expected_color,
            pattern.pattern_at(Point::new(0.0, 0.0, 2.0))
        );
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::new_stripe(Color::white(), Color::black());
        let expected_color_white = Color::white();
        let expected_color_black = Color::black();

        assert_eq!(
            expected_color_white,
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            expected_color_white,
            pattern.pattern_at(Point::new(0.9, 0.0, 0.0))
        );
        assert_eq!(
            expected_color_black,
            pattern.pattern_at(Point::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            expected_color_black,
            pattern.pattern_at(Point::new(-0.1, 0.0, 0.0))
        );
        assert_eq!(
            expected_color_black,
            pattern.pattern_at(Point::new(-1.0, 0.0, 0.0))
        );
        assert_eq!(
            expected_color_white,
            pattern.pattern_at(Point::new(-1.1, 0.0, 0.0))
        );
    }

    #[test]
    fn stripe_pattern_with_primitive_transformation() {
        let pattern = Pattern::new_stripe(Color::white(), Color::black());
        let sphere = PrimitiveShape::SphereShape(
            Sphere::default()
                .scale(2.0, 2.0, 2.0)
                .transform()
                .apply_material(Material::default().apply_pattern(pattern.clone())),
        );
        let expected_color = Color::white();

        let color = pattern.pattern_at_local(&sphere, &Point::new(1.5, 0.0, 0.0));

        assert_eq!(expected_color, color);
    }

    #[test]
    fn stripe_pattern_with_pattern_transformation() {
        let pattern = Pattern::new_stripe(Color::white(), Color::black())
            .scale(2.0, 2.0, 2.0)
            .transform();
        let sphere = PrimitiveShape::SphereShape(
            Sphere::default().apply_material(Material::default().apply_pattern(pattern.clone())),
        );
        let expected_color = Color::white();

        let color = pattern.pattern_at_local(&sphere, &Point::new(1.5, 0.0, 0.0));

        assert_eq!(expected_color, color);
    }

    #[test]
    fn stripe_pattern_with_both_transformation() {
        let pattern = Pattern::new_stripe(Color::white(), Color::black())
            .translate(0.5, 0.0, 0.0)
            .transform();
        let sphere = PrimitiveShape::SphereShape(
            Sphere::default()
                .scale(2.0, 2.0, 2.0)
                .transform()
                .apply_material(Material::default().apply_pattern(pattern.clone())),
        );
        let expected_color = Color::white();

        let color = pattern.pattern_at_local(&sphere, &Point::new(2.5, 0.0, 0.0));

        assert_eq!(expected_color, color);
    }
}
