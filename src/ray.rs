use crate::point::Point;
use crate::scene::Scene;
use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
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
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }
}
