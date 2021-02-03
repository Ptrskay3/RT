use rays::color::{Color, Colorization};
use rays::light::{Light, SphericalLight};
use rays::point::Point;
use rays::primitives::{Element, Material, Plane, Sphere, SurfaceType};
use rays::render::render;
use rays::scene::Scene;
use rays::vector3::Vector3;

fn main() {
    let material = Material {
        color: Colorization::Color(Color {
            red: 0.8,
            green: 0.2,
            blue: 0.4,
        }),
        albedo: 1.9,
        surface: SurfaceType::Diffuse,
    };

    let material2 = Material {
        color: Colorization::Color(Color {
            red: 0.4,
            green: 0.4,
            blue: 0.4,
        }),
        albedo: 1.0,
        surface: SurfaceType::Reflective { reflectivity: 0.97 },
    };

    let material3 = Material {
        color: Colorization::Color(Color {
            red: 0.2,
            green: 0.2,
            blue: 0.2,
        }),
        albedo: 1.0,
        surface: SurfaceType::Diffuse,
    };

    let material4 = Material {
        color: Colorization::Color(Color {
            red: 0.2,
            green: 0.3,
            blue: 0.3,
        }),
        albedo: 1.0,
        surface: SurfaceType::Diffuse,
    };

    // let material5 = Material {
    //     color: Colorization::Color(Color {
    //         red: 0.3,
    //         green: 0.3,
    //         blue: 0.3,
    //     }),
    //     albedo: 1.0,
    //     surface: SurfaceType::Diffuse,
    // };

    let elements = vec![
        Element::Sphere(Sphere {
            center: Point {
                x: -1.1,
                y: 0.5,
                z: -2.0,
            },
            radius: 0.5,
            material: material2,
        }),
        Element::Sphere(Sphere {
            center: Point {
                x: 0.9,
                y: 1.5,
                z: -3.6,
            },
            radius: 1.0,
            material: material,
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
        Element::Plane(Plane {
            p: Point {
                x: 0.0,
                y: 0.0,
                z: -6.5,
            },
            normal: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            material: material4,
        }),
        // Element::Plane(Plane {
        //     p: Point {
        //         x: 0.0,
        //         y: 0.0,
        //         z: 1.0,
        //     },
        //     normal: Vector3 {
        //         x: 0.0,
        //         y: 0.0,
        //         z: 5.0,
        //     },
        //     material: material5,
        // }),
    ];

    let lights = vec![
        Light::Spherical(SphericalLight {
            position: Point {
                x: 1.0,
                y: -50.0,
                z: -6.5,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 1500.0,
        }),
        Light::Spherical(SphericalLight {
            position: Point {
                x: 1.0,
                y: -5.0,
                z: -19.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 1500.0,
        }),
        // Light::Spherical(SphericalLight {
        //     position: Point {
        //         x: 0.0,
        //         y: -5.0,
        //         z: -5.9,
        //     },
        //     color: Color {
        //         red: 1.0,
        //         green: 1.0,
        //         blue: 1.0,
        //     },
        //     intensity: 100.0,
        // }),
        // Light::Directional(DirectionalLight {
        //     direction: Vector3 {
        //         x: 0.0,
        //         y: 2.0,
        //         z: 1.5,
        //     },
        //     color: Color {
        //         red: 1.0,
        //         green: 1.0,
        //         blue: 1.0,
        //     },
        //     intensity: 0.3,
        // }),
        // Light::Directional(DirectionalLight {
        //     direction: Vector3 {
        //         x: 0.0,
        //         y: 0.0,
        //         z: 1.0,
        //     },
        //     color: Color {
        //         red: 1.0,
        //         green: 1.0,
        //         blue: 1.0,
        //     },
        //     intensity: 1.0,
        // }),
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
        origin: Point {
            x: 0.,
            y: 0.3,
            z: 1.0,
        },
        fov: 90.0,
        elements: elements,
        lights: lights,
        shadow_bias: 1E-10,
        max_recursion: 15,
    };
    let img = render(&scene);
    img.save("test.png").unwrap();
}
