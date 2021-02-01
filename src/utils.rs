const GAMMA: f32 = 2.2;

pub fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

pub fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

pub fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}
