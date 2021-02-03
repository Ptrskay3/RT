use crate::ray::Ray;
use crate::scene::{cast_ray, Scene};
use image::{DynamicImage, GenericImage};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            img.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }
    img
}
