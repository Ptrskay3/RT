use crate::color::Color;
use crate::light::Light;
use crate::primitives::{Element, Hittable, Intersection};
use crate::ray::Ray;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.hit(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

pub fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let mut color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    for light in &scene.lights {
        let direction_to_light = -light.direction_from(&hit_point).normalize();
        let shadow_ray = Ray {
            origin: hit_point + (direction_to_light * scene.shadow_bias),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(&hit_point);
        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };
        let light_power =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = intersection.element.albedo() / std::f32::consts::PI;
        let light_color = light.color() * light_power * light_reflected;
        color = color + (*intersection.element.color() * light_color)
    }

    color.clamp()
}
