use crate::color::Color;
use crate::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pattern {
    pub pattern: PatternType,
}

impl Pattern {
    pub fn new_striped(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Striped(StripedPattern::from(color_a, color_b)),
        }
    }

    pub fn pattern_at(&self, point: Point) -> Color {
        match &self.pattern {
            PatternType::Plain(plain) => plain.plain_at(point),
            PatternType::Striped(striped) => striped.striped_at(point),
        }
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            pattern: PatternType::Plain(PlainPattern::default()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PatternType {
    Plain(PlainPattern),
    Striped(StripedPattern),
}

impl PatternType {
    pub fn pattern_at(&self, point: Point) -> Color {
        match self {
            PatternType::Plain(pattern) => pattern.plain_at(point),
            PatternType::Striped(pattern) => pattern.striped_at(point),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StripedPattern {
    color_a: Color,
    color_b: Color,
}

impl StripedPattern {
    fn from(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn striped_at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
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
    fn plain_at(&self, _point: Point) -> Color {
        self.color
    }
}

#[cfg(test)]
mod pattern_tests {
    use crate::color::Color;
    use crate::pattern::PatternType::Plain;
    use crate::pattern::{Pattern, PlainPattern};
    use crate::point::Point;

    #[test]
    fn pattern_creation() {
        let pattern = Pattern::default();
        let expected_pattern = Pattern {
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
}
