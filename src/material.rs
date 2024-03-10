use crate::color::Color;

pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
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
        }
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
        }
    }
}

#[cfg(test)]
mod material_tests {
    use crate::color::Color;
    use crate::material::Material;

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
}
