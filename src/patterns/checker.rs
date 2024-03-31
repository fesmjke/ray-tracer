use crate::color::Color;
use crate::float_eq::ApproxEq;
use crate::point::Point;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct CheckerPattern {
    color_a: Color,
    color_b: Color,
}

impl CheckerPattern {
    pub fn from(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub(crate) fn checker_at(&self, point: &Point) -> Color {
        let sum = point.x.floor() + point.y.floor() + point.z.floor();
        if (sum % 2.0).approx_eq_low(&0.0) {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[cfg(test)]
mod checker_pattern_tests {
    use crate::color::Color;
    use crate::patterns::pattern::Pattern;
    use crate::point::Point;

    #[test]
    fn pattern_checker_should_repeat_in_x() {
        let pattern = Pattern::new_checker(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.pattern_at(&Point::new(0.99, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(&Point::new(1.01, 0.0, 0.0))
        );
    }

    #[test]
    fn pattern_checker_should_repeat_in_y() {
        let pattern = Pattern::new_checker(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.pattern_at(&Point::new(0.0, 0.99, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(&Point::new(0.0, 1.01, 0.0))
        );
    }

    #[test]
    fn pattern_checker_should_repeat_in_z() {
        let pattern = Pattern::new_checker(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.9))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(&Point::new(0.0, 0.0, 1.01))
        );
    }
}
