use bevy::{log::warn, math::Vec2};
use fast_poisson::Poisson;
use rand_xoshiro::Xoshiro256PlusPlus;

/* I may get rid of this in the future if i see that it is useless, for now I just use an extra wrapper */
fn poisson_sampling(
    width: f32,
    hight: f32,
    min_distance: f32,
    max_attempts: u32,
    seed: u64,
    n: usize,
) -> Vec<Vec2> {
    let mut setters = Poisson::<2, Xoshiro256PlusPlus>::new();
    setters.set_seed(seed);
    // Sample on a bounding box of a edge circle
    setters.set_dimensions([width, hight], min_distance);
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

/// Generates up to `n` Poisson Disk points within a bounding box.
///
/// Points will be at least `min_distance` apart and lie inside a square of side length `bounding_side`.
/// The sampling uses up to `max_attempts` tries and is seeded by `seed` for deterministic results.
///
/// # Arguments
///
/// * `width` - Width of a bounding box.
/// * `hight` - Hight of a bounding box.
/// * `min_distance` - Minimum allowed distance between generated points.
/// * `max_attempts` - Maximum number of attempts to place points.
/// * `seed` - Random seed for reproducible sampling.
/// * `n` - Maximum number of points to generate.
///
/// # Returns
///
/// A vector of `Vec2` points within the square bounding box.
pub fn poisson_sample_in_aabb(
    min: Vec2,
    max: Vec2,
    min_distance: f32,
    max_attempts: u32,
    seed: u64,
    n: usize,
) -> Vec<Vec2> {
    let size = max - min;
    let points = poisson_sampling(size.x, size.y, min_distance, max_attempts, seed, n);

    // Translate points into the actual AABB
    points.into_iter().map(|p| p + min).collect()
}
