use image;
use image::{DynamicImage, GenericImage, Rgba, Pixel};
use rays::vector3::Vector3;
use std::ops::{Sub, Mul, Add};
use rays::light::*;


const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

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


}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn zero() -> Self {
        Self {
            x: 0., y: 0., z: 0.
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;
    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;
    fn add(self, other: Vector3) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
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

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.hit(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}


pub struct Plane {
    pub p: Point,
    pub normal: Vector3,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}


impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}


impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
      Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        )
}

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Self {
        assert!(scene.width > scene.height);

        let field_of_view_adjustment = (scene.fov.to_radians() / 2.0).tan(); 

        let aspect = (scene.width as f64) / (scene.height as f64);
        // translate the (width x height) to the (-1..1 x -1..1) range
        // take account for aspect ratio
        let sensor_x = field_of_view_adjustment * (aspect * (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0));
        let sensor_y = field_of_view_adjustment * (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0);
        Self {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0
            }.normalize(),
        }
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

impl Element {
    fn albedo(&self) -> f32 {
        return 1.0;
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
       }

       // in case there's two solutions, return the closer intersection
       let distance = if t0 < t1 { t0 } else { t1 };
       Some(distance)
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

    fn surface_normal(&self, point: &Point) -> Vector3 {
        -self.normal
    }
}


fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    
    // let direction_to_light = -scene.light.direction.normalize();
    let mut color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    // let mut color = intersection.element.color().clone();
    for light in &scene.lights {
        // println!("{:?}", light);
        let direction_to_light = -light.direction_from(&hit_point).normalize();
        let shadow_ray = Ray {
            origin: hit_point + (direction_to_light * scene.shadow_bias),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none() || shadow_intersection.unwrap().distance > light.distance(&hit_point);
        // let in_light = scene.trace(&shadow_ray).is_none();
        let light_intensity = if in_light { light.intensity(&hit_point) } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = intersection.element.albedo() / std::f32::consts::PI;
        let light_color = light.color() * light_power * light_reflected;
        color =  color + (*intersection.element.color() * light_color)
    }

    // intersection.element.color().clone() * scene.light.color.clone() *  light_power *
                //light_reflected;
    color.clamp()
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    let white = Rgba::from_channels(1, 1, 1, 1);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let trace = scene.trace(&ray);
            if let Some(trace) = trace {
                img.put_pixel(x, y, get_color(&scene, &ray, &trace).to_rgba()); // trace.element.color().to_rgba()
            } else {
                img.put_pixel(x, y, white);
            }
            
        }
    }
    img
}



pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color, 
    pub intensity: f32,
}

pub struct SphericalLight {
    pub position: Point,
    pub color: Color, 
    pub intensity: f32,
}


pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => d.color,
            Light::Spherical(ref s) => s.color,
        }
    }

    pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Spherical(ref s) => (s.position - *hit_point).normalize(),
        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Spherical(ref s) => {
                let r2 = (s.position - *hit_point).length_squared() as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }

    pub fn distance(&self, hit_point: &Point) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_point).length(),
        }
    }
}

#[test]
fn render_scene() {
    // let elements = 
    let scene = Scene {
        width: 800,
        height: 800,
        fov: 120.0,
        elements: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
        },
    };

    let img: DynamicImage = render(&scene);
    
}


fn main() {
    let elements = vec![
        Element::Sphere(Sphere {
            center: Point {
                x: -0.2,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 0.0,
                blue: 0.4,
            },
        }),
                Element::Sphere(Sphere {
            center: Point {
                x: 2.0,
                y: 1.0,
                z: -5.0,
            },
            radius: 1.5,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.0,
            },
        }),
        Element::Plane(Plane {
            p: Point {
                x: 0.0,
                y: -2.0,
                z: -6.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: -20.0,
                z: -2.0,
            },
            color: Color {
                red: 0.3,
                green: 0.3,
                blue: 0.3,
            },
        })
    ];

    let lights = vec![
        Light::Spherical(SphericalLight {
            position: Point {
                x: -2.0,
                y: -1.0,
                z: -15.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 1.0,
        }),
        Light::Directional(DirectionalLight {
            direction: Vector3 {
                x: -5.0,
                y: 5.0,
                z: 8.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 0.1,
        }),
        Light::Directional(DirectionalLight {
            direction: Vector3 {
                x: -3.0,
                y: 3.0,
                z: 8.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 1.0,
        })
        ];
    let scene = Scene {
        width: 1920,
        height: 1080,
        fov: 90.0,
        elements: elements,
        lights: lights,
        shadow_bias: 1e-13,
    };
    let img = render(&scene);
    img.save("test.png").unwrap();
}