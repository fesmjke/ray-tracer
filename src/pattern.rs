use crate::color::Color;
use crate::matrices::{Matrix, Matrix4};
use crate::point::Point;
use crate::primitives::{Primitive, PrimitiveShape};
use crate::transformations::Transformable;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    transformation: Matrix4,
    pub pattern: PatternType,
}

impl Pattern {
    pub fn new_striped(color_a: Color, color_b: Color) -> Self {
        Self {
            transformation: Matrix4::identity(),
            pattern: PatternType::Stripe(StripePattern::from(color_a, color_b)),
        }
    }

    pub fn new_gradient(color_a: Color, color_b: Color) -> Self {
        Self {
            transformation: Matrix4::identity(),
            pattern: PatternType::Gradient(GradientPattern::from(color_a, color_b)),
        }
    }

    pub fn new_ring(colors: Vec<Color>) -> Self {
        Self {
            transformation: Matrix4::identity(),
            pattern: PatternType::Ring(RingPattern::from(colors)),
        }
    }

    fn pattern_at(&self, point: Point) -> Color {
        match &self.pattern {
            PatternType::Plain(plain) => plain.plain_at(point),
            PatternType::Stripe(stripe) => stripe.stripe_at(point),
            PatternType::Gradient(gradient) => gradient.gradient_at(point),
            PatternType::Ring(ring) => ring.ring_at(point),
        }
    }

    pub fn pattern_at_local(&self, primitive: PrimitiveShape, world_point: Point) -> Color {
        let primitive_transformation_inv = primitive.transformation().invert();
        let primitive_point = primitive_transformation_inv * world_point;

        let pattern_point = self.transformation.invert() * primitive_point;

        self.pattern_at(pattern_point)
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            transformation: Matrix4::identity(),
            pattern: PatternType::Plain(PlainPattern::default()),
        }
    }
}

impl Transformable for Pattern {
    fn transform(self, transformation: &Matrix4) -> Self {
        Self {
            transformation: transformation.clone() * self.transformation,
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternType {
    Plain(PlainPattern),
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
}

impl PatternType {
    pub fn pattern_at(&self, point: Point) -> Color {
        match self {
            PatternType::Plain(pattern) => pattern.plain_at(point),
            PatternType::Stripe(pattern) => pattern.stripe_at(point),
            PatternType::Gradient(pattern) => pattern.gradient_at(point),
            PatternType::Ring(pattern) => pattern.ring_at(point),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
}

impl StripePattern {
    fn from(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor().abs() % 2.0 == 0.0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PlainPattern {
    color: Color,
}

impl Default for PlainPattern {
    fn default() -> Self {
        Self {
            color: Color::white(),
        }
    }
}

impl PlainPattern {
    pub fn from(color: Color) -> Self {
        Self { color }
    }
    fn plain_at(&self, _point: Point) -> Color {
        self.color
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GradientPattern {
    color_from: Color,
    color_to: Color,
}

impl GradientPattern {
    pub fn from(color_from: Color, color_to: Color) -> Self {
        Self {
            color_from,
            color_to,
        }
    }
    pub fn gradient_at(&self, point: Point) -> Color {
        self.color_from + ((self.color_to - self.color_from) * point.x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RingPattern {
    colors: Vec<Color>,
}

impl RingPattern {
    pub fn from(colors: Vec<Color>) -> Self {
        Self { colors }
    }
    pub fn ring_at(&self, point: Point) -> Color {
        let distance = (point.x * point.x + point.z * point.z).sqrt();
        let index = distance.floor() as usize % self.colors.len();

        self.colors[index]
    }
}

#[cfg(test)]
mod pattern_tests {
    use crate::color::Color;
    use crate::material::Material;
    use crate::matrices::{Matrix, Matrix4};
    use crate::pattern::PatternType::Plain;
    use crate::pattern::{Pattern, PlainPattern};
    use crate::point::Point;
    use crate::primitives::{PrimitiveShape, Sphere};
    use crate::transformations::Transformable;

    #[test]
    fn pattern_creation() {
        let pattern = Pattern::default();
        let expected_pattern = Pattern {
            transformation: Matrix4::identity(),
            pattern: Plain(PlainPattern::default()),
        };

        assert_eq!(expected_pattern, pattern);
    }

    #[test]
    fn pattern_striped_constant_in_y() {
        let pattern = Pattern::new_striped(Color::white(), Color::black());
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
    fn pattern_striped_constant_in_z() {
        let pattern = Pattern::new_striped(Color::white(), Color::black());
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
    fn pattern_striped_alternates_in_x() {
        let pattern = Pattern::new_striped(Color::white(), Color::black());
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
    fn pattern_striped_with_primitive_transformation() {
        let pattern = Pattern::new_striped(Color::white(), Color::black());
        let sphere = PrimitiveShape::SphereShape(
            Sphere::default()
                .scale(2.0, 2.0, 2.0)
                .transform()
                .apply_material(Material::default().apply_pattern(pattern.clone())),
        );
        let expected_color = Color::white();

        let color = pattern.pattern_at_local(sphere, Point::new(1.5, 0.0, 0.0));

        assert_eq!(expected_color, color);
    }

    #[test]
    fn pattern_striped_with_pattern_transformation() {
        let pattern = Pattern::new_striped(Color::white(), Color::black())
            .scale(2.0, 2.0, 2.0)
            .transform();
        let sphere = PrimitiveShape::SphereShape(
            Sphere::default().apply_material(Material::default().apply_pattern(pattern.clone())),
        );
        let expected_color = Color::white();

        let color = pattern.pattern_at_local(sphere, Point::new(1.5, 0.0, 0.0));

        assert_eq!(expected_color, color);
    }

    #[test]
    fn pattern_striped_with_both_transformation() {
        let pattern = Pattern::new_striped(Color::white(), Color::black())
            .translate(0.5, 0.0, 0.0)
            .transform();
        let sphere = PrimitiveShape::SphereShape(
            Sphere::default()
                .scale(2.0, 2.0, 2.0)
                .transform()
                .apply_material(Material::default().apply_pattern(pattern.clone())),
        );
        let expected_color = Color::white();

        let color = pattern.pattern_at_local(sphere, Point::new(2.5, 0.0, 0.0));

        assert_eq!(expected_color, color);
    }

    #[test]
    fn pattern_gradient_linearly_interpolate() {
        let pattern = Pattern::new_gradient(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            pattern.pattern_at(Point::new(0.25, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            pattern.pattern_at(Point::new(0.5, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            pattern.pattern_at(Point::new(0.75, 0.0, 0.0))
        );
    }

    #[test]
    fn pattern_ring_extend_in_both_x_z() {
        let pattern = Pattern::new_ring(vec![Color::white(), Color::black()]);

        assert_eq!(
            Color::white(),
            pattern.pattern_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(Point::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(Point::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(Point::new(0.708, 0.0, 0.708)),
        );
    }
}
