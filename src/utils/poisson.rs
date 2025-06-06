use bevy::{log::warn, math::Vec2};
use fast_poisson::Poisson;
use rand_xoshiro::Xoshiro256PlusPlus;

/// Generates up to `n` Poisson Disk points within a square bounding box.
///
/// Points will be at least `min_distance` apart and lie inside a square of side length `bounding_side`.
/// The sampling uses up to `max_attempts` tries and is seeded by `seed` for deterministic results.
///
/// # Arguments
///
/// * `bounding_side` - Length of each side of the square bounding box.
/// * `min_distance` - Minimum allowed distance between generated points.
/// * `max_attempts` - Maximum number of attempts to place points.
/// * `seed` - Random seed for reproducible sampling.
/// * `n` - Maximum number of points to generate.
///
/// # Returns
///
/// A vector of `Vec2` points within the square bounding box.
pub fn poisson_box_sampling(
    bounding_side: f32,
    min_distance: f32,
    max_attempts: u32,
    seed: u64,
    n: usize,
) -> Vec<Vec2> {
    let mut setters = Poisson::<2, Xoshiro256PlusPlus>::new();
    setters.set_seed(seed);
    // Sample on a bounding box of a edge circle
    setters.set_dimensions([bounding_side, bounding_side], min_distance);
    setters.set_samples(max_attempts);
    let sample: Vec<Vec2> = setters
        .generate()
        .iter()
        .map(|[x, y]| Vec2 { x: *x, y: *y })
        .collect();
    let points: Vec<Vec2> = sample.into_iter().take(n).collect();
    if points.len() < n {
        warn!(
            "Consider tweaking values as Poisson Disk Sampling returned {} < {} points!",
            points.len(),
            n
        );
    }
    return points;
}
