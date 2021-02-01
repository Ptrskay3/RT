use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }
    pub fn albedo(&self) -> f32 {
        return 1.0;
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}

impl Plane {
    pub fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal
    }
}

pub struct Plane {
    pub p: Point,
    pub normal: Vector3,
    pub color: Color,
}

impl Hittable for Element {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.hit(ray),
            Element::Plane(ref p) => p.hit(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        // A line between the ray origin and the center of sphere
        let l: Vector3 = self.center - ray.origin;

        // length of the hypotenuse
        let hypo: f64 = l.dot(&ray.direction);

        // distance from the circle (squared)

        let dist = l.dot(&l) - (hypo * hypo);

        let radius_squared = self.radius * self.radius;
        if dist > radius_squared {
            return None;
        }

        let thc = (radius_squared - dist).sqrt();
        let t0 = hypo - thc;
        let t1 = hypo + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            // in case there's two solutions, return the closer intersection
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denominator = normal.dot(&ray.direction);
        // numerical errors
        if denominator > 1e-6 {
            let v = self.p - ray.origin;
            let dist = v.dot(&normal) / denominator;
            if dist >= 0.0 {
                return Some(dist);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f64>;

    fn surface_normal(&self, hit_point: &Point) -> Vector3;
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            element: element,
        }
    }
}
