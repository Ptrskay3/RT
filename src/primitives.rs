use crate::color::{Color, Colorization};
use crate::point::Point;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
}

#[derive(Clone, Copy)]
pub struct TextureCoordinates {
    pub x: f32,
    pub y: f32,
}

pub struct Material {
    pub color: Colorization,
    pub albedo: f32,
    pub surface: SurfaceType,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f64>;

    fn surface_normal(&self, hit_point: &Point) -> Vector3;

    fn texture_coordinates(&self, hit_point: &Point) -> TextureCoordinates;
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self, hit_point: &Point) -> Color {
        match *self {
            Element::Sphere(ref s) => s.material.color.color(&s.texture_coordinates(&hit_point)),
            Element::Plane(ref p) => {
                let text_coords = &p.texture_coordinates(&hit_point);
                p.material.color.color(text_coords)
            }
        }
    }

    pub fn material(&self) -> &Material {
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
        }
    }

    pub fn material_mut(&mut self) -> &mut Material {
        match *self {
            Element::Sphere(ref mut s) => &mut s.material,
            Element::Plane(ref mut p) => &mut p.material,
        }
    }

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.albedo,
            Element::Plane(ref p) => p.material.albedo,
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
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
    pub material: Material,
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

    fn texture_coordinates(&self, hit_point: &Point) -> TextureCoordinates {
        match *self {
            Element::Sphere(ref s) => s.texture_coordinates(hit_point),
            Element::Plane(ref p) => p.texture_coordinates(hit_point),
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
            // in case there's two solutions, return the closer one
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn texture_coordinates(&self, hit_point: &Point) -> TextureCoordinates {
        let hit_vector = *hit_point - self.center;
        TextureCoordinates {
            x: (1.0 + (hit_vector.z.atan2(hit_vector.x) as f32) / std::f32::consts::PI) * 0.5,
            y: (hit_vector.y / self.radius).acos() as f32 / std::f32::consts::PI,
        }
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

    fn texture_coordinates(&self, hit_point: &Point) -> TextureCoordinates {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);
        let hit_vector = *hit_point - self.p;

        TextureCoordinates {
            x: hit_vector.dot(&x_axis) as f32,
            y: hit_vector.dot(&y_axis) as f32,
        }
    }
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
