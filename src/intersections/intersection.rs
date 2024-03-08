use crate::primitives::Sphere;
use crate::ray::Ray;
use std::fmt::Debug;
use std::ops::Index;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Intersections;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    time: f64,
    object: &'a Sphere, // TODO: add general object or replace with Intersectable
}

impl<'a> Intersection<'a> {
    pub fn new(time: f64, object: &Sphere) -> Intersection {
        Intersection { time, object }
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {
            intersections: Vec::new(),
        }
    }

    pub fn with(mut self, intersections: Vec<Intersection<'a>>) -> Self {
        self.intersections = intersections;
        self
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.time >= 0.0)
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod intersection_tests {
    use crate::intersections::intersection::Intersections;
    use crate::intersections::Intersection;
    use crate::primitives::Sphere;

    #[test]
    fn intersection_creation() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(3.5, &sphere);

        assert_eq!(3.5, intersection_a.time,);
        assert_eq!(&sphere, intersection_a.object);
    }

    #[test]
    fn intersections_aggregating() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(1.0, &sphere);
        let intersection_b = Intersection::new(2.0, &sphere);

        let intersections = Intersections::new().with(vec![intersection_a, intersection_b]);

        assert_eq!(1.0, intersections[0].time);
        assert_eq!(2.0, intersections[1].time);

        assert_eq!(intersections[0].object, intersections[1].object);
    }

    #[test]
    fn intersections_hit_all_positive() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(1.0, &sphere);
        let intersection_b = Intersection::new(2.0, &sphere);
        let intersections = Intersections::new().with(vec![intersection_a.clone(), intersection_b]);
        let expected_hit = Some(&intersection_a);

        assert_eq!(expected_hit, intersections.hit());
    }
}
