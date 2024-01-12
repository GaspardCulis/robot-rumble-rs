use std::f32::consts::PI;

pub const RAD: f32 = 2. * PI;

pub fn lerp<T>(start: T, end: T, t: f32) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + Copy,
{
    start + (end - start) * t
}

pub fn clip_angle(angle: f32) -> f32 {
    ((angle + PI).rem_euclid(2.0 * PI)) - PI
}
