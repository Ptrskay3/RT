const GAMMA: f32 = 2.2;

pub fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}
