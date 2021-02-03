use crate::color::Color;
use crate::ray::Ray;
use crate::scene::{cast_ray, Scene};
use image::{DynamicImage, GenericImage};
use rand::prelude::*;

// Antialiasing
const SAMPLE_PER_PIXEL: u64 = 8;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    let mut rng = rand::thread_rng();
    for x in 0..scene.width {
        for y in 0..scene.height {
            let mut color = Color {
                red: 0.,
                green: 0.,
                blue: 0.,
            };
            for _ in 0..SAMPLE_PER_PIXEL {
                let x_bias: f64 = rng.gen();
                let y_bias: f64 = rng.gen();
                let ray = Ray::create_prime(x as f64 + x_bias, y as f64 + y_bias, scene);
                color = color + cast_ray(scene, &ray, 0);
            }
            img.put_pixel(x, y, (color / SAMPLE_PER_PIXEL as u8).to_rgba());
        }
    }
    img
}
