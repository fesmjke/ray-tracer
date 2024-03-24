use crate::color::Color;
use crate::point::Point;

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
    pub(crate) fn plain_at(&self, _point: Point) -> Color {
        self.color
    }
}

#[cfg(test)]
mod plain_pattern_tests {
    use crate::color::Color;
    use crate::material::Material;
    use crate::patterns::pattern::Pattern;
    use crate::point::Point;
    use crate::primitives::Plane;
    use crate::primitives::PrimitiveShape::PlaneShape;

    #[test]
    fn plain_pattern_constant_at_any_point() {
        let pattern = Pattern::new_plain(Color::white());
        let plane_default =
            Plane::default().apply_material(Material::default().apply_pattern(pattern.clone()));
        let plane = PlaneShape(&plane_default);

        let color = pattern.pattern_at_local(&plane, &Point::new(2.5, 0.0, 0.0));

        let expected_color = Color::white();

        assert_eq!(expected_color, color);
    }
}
