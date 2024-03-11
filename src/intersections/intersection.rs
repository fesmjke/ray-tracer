use crate::primitives::PrimitiveShape;
use std::{cmp::Ordering, fmt::Debug, ops::Index};

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub time: f64,
    pub object: PrimitiveShape,
}

impl Intersection {
    pub fn new(time: f64, object: PrimitiveShape) -> Intersection {
        Intersection { time, object }
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time.is_nan() {
            Ordering::Greater
        } else if other.time.is_nan() {
            Ordering::Less
        } else if self.time > other.time {
            Ordering::Greater
        } else if self.time < other.time {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new() -> Self {
        Self {
            intersections: Vec::new(),
        }
    }

    pub fn with(mut self, intersections: Vec<Intersection>) -> Self {
        self.intersections = intersections;
        self
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections
            .iter()
            .filter(|i| i.time >= 0.0)
            .into_iter()
            .min()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod intersection_tests {
    use crate::intersections::intersection::Intersections;
    use crate::intersections::Intersection;
    use crate::primitives::{PrimitiveShape, Sphere};

    #[test]
    fn intersection_creation() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(3.5, PrimitiveShape::SphereShape(sphere.clone()));

        assert_eq!(3.5, intersection_a.time);
        assert_eq!(
            PrimitiveShape::SphereShape(sphere.clone()),
            intersection_a.object
        );
    }

    #[test]
    fn intersections_aggregating() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(1.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersection_b = Intersection::new(2.0, PrimitiveShape::SphereShape(sphere.clone()));

        let intersections = Intersections::new().with(vec![intersection_a, intersection_b]);

        assert_eq!(1.0, intersections[0].time);
        assert_eq!(2.0, intersections[1].time);

        assert_eq!(intersections[0].object, intersections[1].object);
    }

    #[test]
    fn intersections_hit_all_positive() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(1.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersection_b = Intersection::new(2.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersections = Intersections::new().with(vec![intersection_a.clone(), intersection_b]);
        let expected_hit = Some(&intersection_a);

        assert_eq!(expected_hit, intersections.hit());
    }

    #[test]
    fn intersections_hit_some_negative() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(-1.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersection_b = Intersection::new(2.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersections = Intersections::new().with(vec![intersection_a, intersection_b.clone()]);
        let expected_hit = Some(&intersection_b);

        assert_eq!(expected_hit, intersections.hit());
    }

    #[test]
    fn intersections_hit_all_negative() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(-1.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersection_b = Intersection::new(-2.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersections = Intersections::new().with(vec![intersection_a, intersection_b]);
        let expected_hit = None;

        assert_eq!(expected_hit, intersections.hit());
    }

    #[test]
    fn intersections_hit_lowest_non_negative() {
        let sphere = Sphere::default();
        let intersection_a = Intersection::new(5.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersection_b = Intersection::new(7.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersection_c = Intersection::new(-1.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersection_d = Intersection::new(2.0, PrimitiveShape::SphereShape(sphere.clone()));
        let intersections = Intersections::new().with(vec![
            intersection_a,
            intersection_b,
            intersection_c,
            intersection_d.clone(),
        ]);
        let expected_hit = Some(&intersection_d);

        assert_eq!(expected_hit, intersections.hit());
    }
}
