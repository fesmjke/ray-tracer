use crate::matrices::{Matrix, Matrix4};

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
}

#[cfg(test)]
mod camera_tests {
    use crate::camera::Camera;
    use crate::matrices::{Matrix, Matrix4};
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
}
