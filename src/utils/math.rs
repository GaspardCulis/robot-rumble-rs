pub fn lerp<T>(start: T, end: T, t: f32) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + Copy,
{
    start + (end - start) * t
}
