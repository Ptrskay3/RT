use image;
use image::{DynamicImage, GenericImage, Rgba, Pixel};
use rays::vector3::Vector3;
use std::ops::Sub;


const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
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

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
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
    fn hit(&self, ray: &Ray) -> bool;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        // A line between the ray origin and the center of sphere
        let l: Vector3 = self.center - ray.origin;

        // length of the hypotenuse
        let hypo: f64 = l.dot(&ray.direction);

        // distance from the circle (squared)

        let dist = l.dot(&l) - (hypo * hypo);

        dist < (self.radius * self.radius)

    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.hit(&ray) {
                img.put_pixel(x, y, scene.sphere.color.to_rgba());
            } else {
                img.put_pixel(x, y, black);
            }
        }
    }
    img
}

#[test]
fn render_scene() {
    let scene = Scene {
        width: 800,
        height: 800,
        fov: 120.0,
        sphere: Sphere {
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
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
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
        }
    };
    let img = render(&scene);
    img.save("test.png").unwrap();
}