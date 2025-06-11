use bevy::prelude::*;
use voronoice::*;

// Passing diagram as is for the moment
#[derive(Event)]
pub struct VoronoiGeneratedEvent {
    pub polygons: Vec<Vec<Vec2>>,
    pub centroids: Vec<Vec2>,
}

pub struct VoronoPlugin;
impl Plugin for VoronoPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VoronoiGeneratedEvent>()
            .add_systems(Update, draw_voronoi);
    }
}

/// Builds a Voronoi diagram from given sites within a centered square bounding box.
/// Applies the specified number of Lloyd relaxation iterations to produce a more
/// centroidal, regular cell distribution.
///
/// # Parameters
/// - `sites`: Points to generate the Voronoi diagram for.
/// - `bounding_side`: Side length of the square bounding box.
/// - `relaxation`: Number of Lloyd relaxation iterations.
///
/// # Returns
/// The computed `Voronoi` diagram.
/// **TODO: after fixes swap to polygons instead.**
pub fn build_voronoi(sites: Vec<Vec2>, bounding_side: f64, relaxation: usize) -> Voronoi {
    let voronoi_graph = VoronoiBuilder::default()
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
    return voronoi_graph;
}

/// Clamps a Voronoi diagram to fit within a bounding circle.
///
/// For each cell in the diagram, its vertices are clamped to the given radius if they exceed it.
/// Returns the adjusted polygons and their barycenters (centroids).
///
/// # Returns
/// A tuple containing:
/// - `Vec<Vec<Vec2>>`: Clamped polygon vertices for each cell.
/// - `Vec<Vec2>`: Barycenters of the clamped polygons.
pub fn adjust_to_circle(voronoi: Voronoi, bounding_radius: f32) -> (Vec<Vec<Vec2>>, Vec<Vec2>) {
    let mut centroids = Vec::new();
    let mut clamped_cells = Vec::new();

    voronoi.iter_cells().for_each(|cell| {
        let mut barycenter = Vec2::ZERO;
        let mut vert_cnt = 0;
        let vertices: Vec<Vec2> = cell
            .iter_vertices()
            .map(|p| {
                // Convert to bevy's format of points
                let mut v = Vec2::new(p.x as f32, p.y as f32);
                let dist = v.length();
                // Clamp to radius
                if dist > bounding_radius {
                    v = Vec2::ZERO + v.normalize() * bounding_radius;
                }
                // Compute the new barycenter simultaniously
                barycenter += v;
                vert_cnt += 1;
                v
            })
            .collect();
        barycenter /= vert_cnt as f32;
        clamped_cells.push(vertices);
        centroids.push(barycenter);
    });

    return (clamped_cells, centroids);
}

fn draw_voronoi(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<VoronoiGeneratedEvent>,
) {
    for VoronoiGeneratedEvent {
        polygons,
        centroids,
    } in events.read()
    {
        for (polygon, centroid) in polygons.iter().zip(centroids.iter()) {
            // use the same random color for current polygon to separate zones
            let hue = rand::random::<f32>() * 360.0;
            let cell_color = Color::hsla(hue, 0.7, 0.5, 0.1);

            // Triangulate the mesh over the cell's barycenter
            let card_vertices = polygon.len();
            for i in 0..card_vertices {
                let v0 = Vec2::new(polygon[i].x as f32, polygon[i].y as f32);
                let v1 = Vec2::new(
                    polygon[(i + 1) % card_vertices].x as f32,
                    polygon[(i + 1) % card_vertices].y as f32,
                );
                let triangle_mesh = Triangle2d::new(*centroid, v0, v1);
                commands.spawn((
                    Mesh2d(meshes.add(triangle_mesh)),
                    MeshMaterial2d(color_materials.add(ColorMaterial::from_color(cell_color))),
                    Transform::default(),
                ));
            }
        }
    }
}
