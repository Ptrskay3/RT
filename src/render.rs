use crate::ray::Ray;
use crate::scene::{cast_ray, get_color, Scene};
use image::{DynamicImage, GenericImage, Pixel, Rgba};

// let screen_width: i32 = 3840;
// let screen_height: i32 = 2160;
// let aspect_ratio = (screen_width/screen_height) as i32;

// let viewport_height: f64 = 2.0;
// let viewport_width: f64 = aspect_ratio as f64 * viewport_height;
// let focal_length: f64 = 1.0;

// let origin = Point{
//     x: 0.0, y: 0.0, z: 0.0
//         };
// let horizontal = Vector3{
//     x: viewport_width,
//     y: 0.0,
//     z: 0.0,
// };
// let vertical = Vector3{
//     x: 0.0,
//     y: viewport_height,
//     z: 0.0,
// };

// let lower_left_corner = origin - horizontal.as_point() / 2.0 - vertical / 2.0 - Vector3 {
//     x: 0.0,
//     y: 0.0,
//     z: focal_length,
// };

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    // let black = Rgba::from_channels(0, 0, 0, 0);
    // let white = Rgba::from_channels(1, 1, 1, 1);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            // let trace = scene.trace(&ray);
            // if let Some(trace) = trace {
            img.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
            // } else {
            // img.put_pixel(x, y, black);
            // }
        }
    }
    img
}
