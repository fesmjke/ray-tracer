use crate::color::Color;
use crate::point::Point;

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
        let fraction = point.x - point.x.floor();
        let distance = self.color_to - self.color_from;
        self.color_from + (distance * fraction)
    }
}

#[cfg(test)]
mod gradient_pattern_tests {
    use crate::color::Color;
    use crate::patterns::pattern::Pattern;
    use crate::point::Point;

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
}
