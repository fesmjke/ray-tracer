use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrices::{Matrix, Matrix4};
use crate::point::Point;
use crate::ray::Ray;
use crate::render::{Render, Rendering};
use crate::transformations::Transformable;
use crate::vector::Vector3;
use crate::world::World;
use rayon::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Camera {
    horizontal_size: usize,
    vertical_size: usize,
    fov: f64,
    transformation: Matrix4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Self {
            horizontal_size: hsize,
            vertical_size: vsize,
            fov,
            transformation: Matrix4::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transformation_inv = self.transformation.invert();
        let pixel = transformation_inv.clone() * Point::new(world_x, world_y, -1.0);

        let origin = transformation_inv * Point::default();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, Vector3::new(direction.x, direction.y, direction.z))
    }

    pub fn sequential_render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.horizontal_size, self.vertical_size, Color::default());

        for x in 0..self.horizontal_size {
            for y in 0..self.vertical_size {
                let ray = self.ray_for_pixel(x as f64, y as f64);
                let color = world.color_at(&ray);

                image[y][x] = color;
            }
        }

        image
    }

    pub fn parallel_render(&self, world: &World) -> Canvas {
        const BAND_SIZE: usize = 3;
        let mut image = Canvas::new(self.horizontal_size, self.vertical_size, Color::default());

        image
            .pixels()
            .par_chunks_mut(self.horizontal_size * BAND_SIZE)
            .enumerate()
            .for_each(|(i, band)| {
                for row in 0..BAND_SIZE {
                    for col in 0..self.horizontal_size {
                        let ray = self.ray_for_pixel(col as f64, (row + i * BAND_SIZE) as f64);
                        let color = world.color_at(&ray);
                        band[row * self.horizontal_size + col] = color;
                    }
                }
            });

        image
    }

    pub fn render(&self, world: &World, setting: Render) -> Canvas {
        match setting.render_mode {
            Rendering::Parallel => self.parallel_render(&world),
            Rendering::Sequential => self.sequential_render(&world),
        }
    }
}

impl Transformable for Camera {
    fn transform(self, transformation: &Matrix4) -> Self {
        Self {
            transformation: transformation.clone() * self.transformation,
            ..self
        }
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::camera::Camera;
    use crate::matrices::{Matrix, Matrix4};
    use crate::point::Point;
    use crate::ray::Ray;
    use crate::transformations::{Over, Transformable};
    use crate::vector::Vector3;
    use std::f64::consts::PI;

    #[test]
    fn camera_creation() {
        let camera = Camera::new(160, 120, PI / 2.0);
        let expected_camera = Camera {
            horizontal_size: 160,
            vertical_size: 120,
            fov: PI / 2.0,
            transformation: Matrix4::identity(),
            pixel_size: 0.012499999999999999,
            half_width: 0.9999999999999999,
            half_height: 0.75,
        };

        assert_eq!(expected_camera, camera);
    }

    #[test]
    fn camera_ray_trough_center() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let expected_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));

        assert_eq!(expected_ray, camera.ray_for_pixel(100.0, 50.0));
    }

    #[test]
    fn camera_ray_trough_corner() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let expected_ray = Ray::new(
            Point::new(0.0, 0.0, 0.0),
            Vector3::new(0.66519, 0.33259, -0.66851),
        );

        assert_eq!(expected_ray, camera.ray_for_pixel(0.0, 0.0));
    }

    #[test]
    fn camera_ray_when_transformed() {
        let camera = Camera::new(201, 101, PI / 2.0)
            .translate(0.0, -2.0, 5.0)
            .rotate(Over::Y, PI / 4.0)
            .transform();

        let expected_ray = Ray::new(
            Point::new(0.0, 2.0, -5.0),
            Vector3::new(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0),
        );

        assert_eq!(expected_ray, camera.ray_for_pixel(100.0, 50.0));
    }
}
