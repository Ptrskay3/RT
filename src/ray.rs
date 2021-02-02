use crate::point::Point;
use crate::scene::Scene;
use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    /// Return the ray at point `t`.
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Self {
        assert!(scene.width > scene.height);

        let field_of_view_adjustment = (scene.fov.to_radians() / 2.0).tan();

        let aspect = (scene.width as f64) / (scene.height as f64);
        // translate the (width x height) to the (-1..1 x -1..1) range
        // take account for aspect ratio
        let sensor_x = field_of_view_adjustment
            * (aspect * (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0));
        let sensor_y =
            field_of_view_adjustment * (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0);
        Self {
            origin: scene.origin,
            direction: Vector3 {
                x: sensor_x + scene.direction.x,
                y: sensor_y + scene.direction.y,
                z: -1.5 + scene.direction.z,
            }
            .normalize(),
        }
    }

    pub fn create_reflection(
        normal: Vector3,
        incident: Vector3,
        intersection: Point,
        bias: f64,
    ) -> Ray {
        Ray {
            origin: intersection + (normal * bias),
            direction: incident - (2.0 * incident.dot(&normal) * normal),
        }
    }

    pub fn create_transmission(
        normal: Vector3,
        incident: Vector3,
        intersection: Point,
        bias: f64,
        index: f32,
    ) -> Option<Ray> {
        let mut ref_n = normal;
        let mut n_t = index as f64;
        let mut n_i = 1.0f64;
        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < 0.0 {
            //Outside the surface
            i_dot_n = -i_dot_n;
        } else {
            //Inside the surface; invert the normal and swap the indices of refraction
            ref_n = -normal;
            n_t = 1.0;
            n_i = index as f64;
        }

        let eta = n_i / n_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: intersection + (ref_n * -bias),
                direction: (incident + i_dot_n * ref_n) * eta - ref_n * k.sqrt(),
            })
        }
    }
}
