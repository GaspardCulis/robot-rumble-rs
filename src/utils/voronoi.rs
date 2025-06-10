use bevy::prelude::*;
use voronoice::*;

// For the moment it returns Diagram as is, might be useful.*
pub fn build_voronoi_diagram(sites: Vec<Vec2>, bounding_side: f64, relaxation: usize) -> Voronoi {
    return VoronoiBuilder::default()
        .set_sites(
            sites
                .iter()
                .map(|site| Point {
                    x: site.x as f64,
                    y: site.y as f64,
                })
                .collect(),
        )
        .set_bounding_box(BoundingBox::new_centered_square(bounding_side))
        .set_lloyd_relaxation_iterations(relaxation)
        .build()
        .unwrap();
}
