use bevy::{log::warn, math::Vec2};
use fast_poisson::Poisson;
use rand_xoshiro::Xoshiro256PlusPlus;

/// Generates `n` Poisson Disk points within a square bounding box of size `bounding_radius * 2`.
///
/// Points will be at least `min_distance` apart. Up to `max_attempts` are performed.
///
/// # Note
/// The resulting points are **not automatically centered around (0, 0)**.
///
/// # Arguments
/// * `bounding_radius` - Half of the box size; after centering, points will be in [-bounding_radius, +bounding_radius].
/// * `min_distance` - Minimum distance between points.
/// * `max_attempts` - Maximum number of sampling attempts.
/// * `seed` - Random seed for deterministic generation.
/// * `n` - Number of points to generate.
///
/// # Returns
/// A `Vec<Vec2>` of up to `n` valid points
///
///
pub fn poisson_box_sampling(
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
