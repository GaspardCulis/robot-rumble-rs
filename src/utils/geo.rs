use bevy::{log::warn, math::Vec2};

use crate::utils::poisson::poisson_sample_in_aabb;

/// Returns true if `point` is inside the convex `polygon` using vectors.
pub fn is_point_in_convex_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    if polygon.len() < 3 {
        warn!("Not a polygon!");
        return false;
    }

    let mut sign = 0.0;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];

        let edge = b - a;
        let to_point = point - a;

        let cross = edge.x * to_point.y - edge.y * to_point.x;

        if cross == 0.0 {
            // Point is on the edge, sign for it is inconsistent
            continue;
        }

        if sign == 0.0 {
            sign = cross.signum(); // First non-zero cross product sign
        } else if cross.signum() != sign {
            return false;
        }
    }
    return true;
}

fn point_to_line_distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let cross = ab.perp_dot(ap);
    cross.abs() / ab.length()
}

pub fn is_circle_inside_convex_polygon(center: Vec2, radius: f32, polygon: &[Vec2]) -> bool {
    if !is_point_in_convex_polygon(center, polygon) {
        return false;
    }

    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        let dist = point_to_line_distance(center, a, b);
        if dist < radius {
            return false;
        }
    }

    return true;
}

pub fn get_aabb(polygon: &[Vec2]) -> Option<(Vec2, Vec2)> {
    if polygon.len() < 3 {
        warn!("Not a polygon!");
        return None;
    }

    let mut min = polygon[0];
    let mut max = polygon[0];

    for &p in polygon.iter().skip(1) {
        min = min.min(p);
        max = max.max(p);
    }

    Some((min, max))
}

// Uses Poisson Sampling for now
pub fn _sample_points_in_polygon(polygon: &[Vec2], seed: u64, n: usize) -> Option<Vec<Vec2>> {
    if let Some((min, max)) = get_aabb(polygon) {
        let mut points = poisson_sample_in_aabb(min, max, 200., 100, seed, 2 * n);

        // Retain only the points inside the polygon
        points.retain(|&p| is_point_in_convex_polygon(p, polygon));

        // Truncate to the first `n` valid points
        points.truncate(n);
        Some(points)
    } else {
        None
    }
}
