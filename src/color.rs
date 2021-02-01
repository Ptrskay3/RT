use crate::primitives::TextureCoordinates;
use crate::utils::{gamma_decode, gamma_encode, wrap};
use std::ops::{Add, Mul};
use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, Pixel, Rgba};

pub struct Texture {
    pub path: PathBuf,
    pub texture: DynamicImage,
}

pub fn dummy_texture() -> DynamicImage {
    DynamicImage::new_rgb8(1, 1)
}


pub enum Colorization {
    Color(Color),
    Texture(DynamicImage),
}

impl Colorization {
    pub fn color(&self, texture_coords: &TextureCoordinates) -> Color {
        match *self {
            Colorization::Color(ref c) => c.clone(),

            Colorization::Texture(ref tex) => {
                let tex_x = wrap(texture_coords.x, tex.width());
                let tex_y = wrap(texture_coords.y, tex.height());

                Color::from_rgba(tex.get_pixel(tex_x, tex_y))
            }
        }
    }
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
    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode((rgba[0] as f32) / 255.0),
            green: gamma_decode((rgba[1] as f32) / 255.0),
            blue: gamma_decode((rgba[2] as f32) / 255.0),
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}
