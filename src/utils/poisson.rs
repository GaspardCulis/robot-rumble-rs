use bevy::{log::warn, math::Vec2};
use fast_poisson::Poisson;
use rand_xoshiro::Xoshiro256PlusPlus;

pub fn poisson_disk_sampling_circle(
    bounding_radius: f32,
    min_distance: f32,
    max_attempts: u32,
    seed: u64,
    n: usize,
) -> Vec<Vec2> {
    let mut setters = Poisson::<2, Xoshiro256PlusPlus>::new();
    setters.set_seed(seed);
    // Sample on a bounding box of a edge circle
    setters.set_dimensions([bounding_radius * 2.0, bounding_radius * 2.0], min_distance);
    setters.set_samples(max_attempts);
    let sample: Vec<Vec2> = setters
        .generate()
        .iter()
        .map(|[x, y]| Vec2 { x: *x, y: *y })
        .collect();
    let points: Vec<Vec2> = sample.into_iter().take(n).collect();
    if points.len() < n {
        warn!(
            "Poisson Disk Sampling returned less points: {}!",
            points.len()
        );
    }
    return points;
}
