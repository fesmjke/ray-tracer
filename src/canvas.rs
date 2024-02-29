use crate::color::Color;
use image::{ImageBuffer, ImageResult};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn export(&self, path: &str) -> ImageResult<()> {
        let mut img = ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self[y as usize][x as usize];
            let (r, g, b) = self.convert_color(color);
            *pixel = image::Rgb([r, g, b]);
        }

        img.save(path)
    }

    fn convert_color(&self, color: &Color) -> (u8, u8, u8) {
        (
            self.convert_component(color.r),
            self.convert_component(color.g),
            self.convert_component(color.b),
        )
    }

    fn convert_component(&self, component: f64) -> u8 {
        let component = if component < 0.0 {
            0.0
        } else if component > 1.0 {
            1.0
        } else {
            component
        };

        (component * 255.0) as u8
    }

    pub fn pixels(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }
}

impl Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, row: usize) -> &[Color] {
        let start = row * self.width;

        &self.pixels[start..start + self.width]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let start = row * self.width;

        &mut self.pixels[start..start + self.width]
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn canvas_creation() {
        let canvas = Canvas::new(5, 5, Color::new(0.0, 0.0, 0.0));

        assert_eq!(canvas[1][1], Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn set_pixel() {
        let mut canvas = Canvas::new(10, 20, Color::new(0.0, 0.0, 0.0));
        canvas[2][3] = Color::new(1.0, 0.0, 0.0);

        assert_eq!(canvas[2][3], Color::new(1.0, 0.0, 0.0));
        assert_eq!(canvas[0][1], Color::new(0.0, 0.0, 0.0));
    }
}
