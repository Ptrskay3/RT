use rays::color::{Color, Colorization};
use rays::light::{DirectionalLight, Light, SphericalLight};
use rays::point::Point;
use rays::primitives::{Element, Material, Plane, Sphere, SurfaceType};
use rays::render::render;
use rays::scene::Scene;
use rays::vector3::Vector3;

fn main() {
    let material = Material {
        color: Colorization::Color(Color {
            red: 0.4,
            green: 0.2,
            blue: 0.4,
        }),
        albedo: 1.0,
        surface: SurfaceType::Diffuse,
    };

    let material2 = Material {
        color: Colorization::Color(Color {
            red: 0.4,
            green: 0.4,
            blue: 0.4,
        }),
        albedo: 1.0,
        surface: SurfaceType::Reflective { reflectivity: 1.0 },
    };

    let material3 = Material {
        color: Colorization::Color(Color {
            red: 0.3,
            green: 0.3,
            blue: 0.3,
        }),
        albedo: 1.0,
        surface: SurfaceType::Diffuse,
    };

    let elements = vec![
        Element::Sphere(Sphere {
            center: Point {
                x: -1.1,
                y: 0.5,
                z: -2.0,
            },
            radius: 0.5,
            material: material,
        }),
        Element::Sphere(Sphere {
            center: Point {
                x: 0.4,
                y: 0.5,
                z: -1.6,
            },
            radius: 0.5,
            material: material2,
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
                z: -5.0,
            },
            material: material3,
        }),
    ];

    let lights = vec![
        Light::Spherical(SphericalLight {
            position: Point {
                x: 1.0,
                y: -18.0,
                z: -6.5,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 600.0,
        }),
        // Light::Spherical(SphericalLight {
        //     position: Point {
        //         x: -1.1,
        //         y: 0.5,
        //         z: -0.4,
        //     },
        //     color: Color {
        //         red: 1.0,
        //         green: 1.0,
        //         blue: 1.0,
        //     },
        //     intensity: 3000.0,
        // }),
        Light::Directional(DirectionalLight {
            direction: Vector3 {
                x: 0.0,
                y: -5.0,
                z: 20.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 0.2,
        }),
        Light::Directional(DirectionalLight {
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 1.0,
        }),
        // Light::Directional(DirectionalLight {
        //     direction: Vector3 {
        //         x: 0.0,
        //         y: -10.0,
        //         z: 10.0,
        //     },
        //     color: Color {
        //         red: 1.0,
        //         green: 1.0,
        //         blue: 1.0,
        //     },
        //     intensity: 0.5,
        // }),
    ];

    let scene = Scene {
        width: 3840,
        height: 2160,
        fov: 90.0,
        elements: elements,
        lights: lights,
        shadow_bias: 1e-13,
        max_recursion: 8,
    };
    let img = render(&scene);
    img.save("test.png").unwrap();
}
