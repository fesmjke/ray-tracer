use crate::color::Color;
use crate::point::Point;

#[derive(Clone, Debug, PartialEq)]
pub struct RingPattern {
    color_a: Color,
    color_b: Color,
}

impl RingPattern {
    pub fn from(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }
    pub fn ring_at(&self, point: &Point) -> Color {
        let distance = (point.x.powf(2.0) + point.z.powf(2.0)).sqrt();

        if distance.floor() as i64 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[cfg(test)]
mod ring_pattern_tests {
    use crate::color::Color;
    use crate::patterns::pattern::Pattern;
    use crate::point::Point;

    #[test]
    fn pattern_ring_extend_in_both_x_z() {
        let pattern = Pattern::new_ring(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(&Point::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(&Point::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::black(),
            pattern.pattern_at(&Point::new(0.708, 0.0, 0.708)),
        );
    }
}
