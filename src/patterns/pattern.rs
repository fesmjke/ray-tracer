use crate::color::Color;
use crate::matrices::{Matrix, Matrix4};
use crate::patterns::{CheckerPattern, GradientPattern, PlainPattern, RingPattern, StripePattern};
use crate::point::Point;
use crate::primitives::{Primitive, PrimitiveShape};
use crate::transformations::Transformable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pattern {
    pub pattern: PatternType,
    transformation: Matrix4,
    transformation_inverse: Matrix4,
}

impl Pattern {
    pub fn new_stripe(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Stripe(StripePattern::from(color_a, color_b)),
            ..Default::default()
        }
    }

    pub fn new_gradient(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Gradient(GradientPattern::from(color_a, color_b)),
            ..Default::default()
        }
    }

    pub fn new_ring(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Ring(RingPattern::from(color_a, color_b)),
            ..Default::default()
        }
    }

    pub fn new_checker(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Checker(CheckerPattern::from(color_a, color_b)),
            ..Default::default()
        }
    }

    pub fn new_plain(color: Color) -> Self {
        Self {
            pattern: PatternType::Plain(PlainPattern::from(color)),
            ..Default::default()
        }
    }

    pub fn new_test() -> Self {
        Self {
            pattern: PatternType::Test(TestPattern {}),
            ..Default::default()
        }
    }
    pub(crate) fn pattern_at(&self, point: &Point) -> Color {
        match &self.pattern {
            PatternType::Plain(plain) => plain.plain_at(point),
            PatternType::Stripe(stripe) => stripe.stripe_at(point),
            PatternType::Gradient(gradient) => gradient.gradient_at(point),
            PatternType::Ring(ring) => ring.ring_at(point),
            PatternType::Checker(checker) => checker.checker_at(point),
            PatternType::Test(pattern) => pattern.test_at(point),
        }
    }

    pub fn pattern_at_local(&self, primitive: &PrimitiveShape, world_point: &Point) -> Color {
        let primitive_transformation_inv = primitive.transformation_invert();
        let primitive_point = *primitive_transformation_inv * *world_point;

        let pattern_point = self.transformation_inverse * primitive_point;

        self.pattern_at(&pattern_point)
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            pattern: PatternType::Plain(PlainPattern::default()),
        }
    }
}

impl Transformable for Pattern {
    fn transform(self, transformation: &Matrix4) -> Self {
        let delta = *transformation * self.transformation;
        Self {
            transformation: *transformation * self.transformation,
            transformation_inverse: delta.invert(),
            ..self
        }
    }
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum PatternType {
    Plain(PlainPattern),
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checker(CheckerPattern),
    Test(TestPattern),
}

impl PatternType {
    pub fn pattern_at(&self, point: &Point) -> Color {
        match self {
            PatternType::Plain(pattern) => pattern.plain_at(point),
            PatternType::Stripe(pattern) => pattern.stripe_at(point),
            PatternType::Gradient(pattern) => pattern.gradient_at(point),
            PatternType::Ring(pattern) => pattern.ring_at(point),
            PatternType::Checker(pattern) => pattern.checker_at(point),
            PatternType::Test(pattern) => pattern.test_at(point),
        }
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TestPattern {}

impl TestPattern {
    pub fn test_at(&self, point: &Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod pattern_tests {
    use crate::matrices::{Matrix, Matrix4};
    use crate::patterns::pattern::Pattern;
    use crate::patterns::PatternType::Plain;
    use crate::patterns::PlainPattern;

    #[test]
    fn pattern_creation() {
        let pattern = Pattern::default();
        let expected_pattern = Pattern {
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            pattern: Plain(PlainPattern::default()),
        };

        assert_eq!(expected_pattern, pattern);
    }
}
