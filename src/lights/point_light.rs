use crate::color::Color;
use crate::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

impl PointLight {
    pub fn new(intensity: Color, position: Point) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            intensity: Color::default(),
            position: Point::default(),
        }
    }
}

#[cfg(test)]
mod point_light_tests {
    use crate::color::Color;
    use crate::lights::point_light::PointLight;
    use crate::point::Point;

    #[test]
    fn point_light_creation() {
        let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::default());
        let expected_intensity = Color::new(1.0, 1.0, 1.0);

        assert_eq!(expected_intensity, point_light.intensity);
        assert_eq!(Point::default(), point_light.position);
    }
}
