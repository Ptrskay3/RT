use crate::color::Color;
use crate::light::Light;
use crate::point::Point;
use crate::primitives::{Element, Hittable, Intersection, SurfaceType};
use crate::ray::Ray;
use crate::vector3::Vector3;

const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
    pub max_recursion: u32,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.hit(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

pub fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let mut color = shade_diffuse(scene, intersection.element, hit_point, surface_normal);
    if let SurfaceType::Reflective { reflectivity } = intersection.element.material().surface {
        let reflection_ray =
            Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
        color = color * (1.0 - reflectivity);
        color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
    }
    color
}



fn shade_diffuse(
    scene: &Scene,
    element: &Element,
    hit_point: Point,
    surface_normal: Vector3,
) -> Color {
    let texture_coords = element.texture_coordinates(&hit_point);
    let mut color = BLACK;
    for light in &scene.lights {
        let direction_to_light = -light.direction_from(&hit_point);

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
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
        let material = element.material();
        let light_power =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = material.albedo / std::f32::consts::PI;

        let light_color = light.color() * light_power * light_reflected;
        color = color + (material.color.color(&texture_coords) * light_color);
    }
    color.clamp()
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.max_recursion {
        return BLACK;
    }

    let intersection = scene.trace(&ray);
    intersection
        .map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(BLACK)
}
