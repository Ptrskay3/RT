use rays::color::Color;
use rays::light::{DirectionalLight, Light, SphericalLight};
use rays::point::Point;
use rays::primitives::{Element, Plane, Sphere};
use rays::render::render;
use rays::scene::Scene;
use rays::vector3::Vector3;

fn main() {
    let elements = vec![
        Element::Sphere(Sphere {
            center: Point {
                x: -0.9,
                y: 0.5,
                z: -2.0,
            },
            radius: 0.5,
            color: Color {
                red: 0.4,
                green: 0.0,
                blue: 0.4,
            },
        }),
        Element::Sphere(Sphere {
            center: Point {
                x: 0.5,
                y: 1.0,
                z: -2.5,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.0,
            },
        }),
        Element::Plane(Plane {
            p: Point {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: -20.0,
                z: -1.0,
            },
            color: Color {
                red: 0.3,
                green: 0.3,
                blue: 0.3,
            },
        }),
    ];

    let lights = vec![
        Light::Spherical(SphericalLight {
            position: Point {
                x: -2.0,
                y: -1.0,
                z: -1.0,
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
        }),
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
