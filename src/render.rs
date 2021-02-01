use crate::ray::Ray;
use crate::scene::{cast_ray, get_color, Scene};
use image::{DynamicImage, GenericImage, Pixel, Rgba};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
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
