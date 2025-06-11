use bevy::prelude::*;
use voronoice::*;

// Passing diagram as is for the moment
#[derive(Event)]
pub struct VoronoiGeneratedEvent {
    pub voronoi: Voronoi,
    pub bounding_radius: f32,
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

/// Converts a Voronoi diagram to polygons clamping them to circle.
/// # Returns
/// Adjusted olygons with new barycenters.
pub fn adjust_to_circle(voronoi: Voronoi, bounding_radius: f32) -> (Vec<Vec<Vec2>>, Vec<Vec2>) {
    // iterate over each cell of the generated diagram
    let mut centroids = Vec::new();
    let mut clamped_cells = Vec::new();
    // iterate over each cell of the generated diagram
    voronoi.iter_cells().for_each(|cell| {
        // convert to bevy's format of points
        let mut barycenter = Vec2::ZERO;
        let mut vert_cnt = 0;
        let vertices: Vec<Vec2> = cell
            .iter_vertices()
            .map(|p| {
                let mut v = Vec2::new(p.x as f32, p.y as f32);
                let dist = v.length();
                // clamp to radius
                if dist > bounding_radius {
                    v = Vec2::ZERO + v.normalize() * bounding_radius;
                }
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
        voronoi,
        bounding_radius,
    } in events.read()
    {
        // iterate over each cell of the generated diagram
        let mut centroids: Vec<Vec2> = Vec::new();
        // iterate over each cell of the generated diagram
        voronoi.iter_cells().for_each(|cell| {
            // convert to bevy's format of points
            let mut barycenter = Vec2::ZERO;
            let mut vert_cnt = 0;
            let vertices: Vec<Vec2> = cell
                .iter_vertices()
                .map(|p| {
                    let mut v = Vec2::new(p.x as f32, p.y as f32);
                    let dist = v.length();
                    // clamp to radius
                    if dist > *bounding_radius {
                        v = Vec2::ZERO + v.normalize() * bounding_radius;
                    }
                    barycenter += v;
                    vert_cnt += 1;
                    v
                })
                .collect();
            barycenter /= vert_cnt as f32;
            centroids.push(barycenter);

            // use the same random color for current cell to separate zones
            let hue = rand::random::<f32>() * 360.0;
            let cell_color = Color::hsla(hue, 0.7, 0.5, 0.1);

            // Triangulate the mesh over the cell's barycenter
            let card_vertices = vertices.len();
            for i in 0..card_vertices {
                let v0 = Vec2::new(vertices[i].x as f32, vertices[i].y as f32);
                let v1 = Vec2::new(
                    vertices[(i + 1) % card_vertices].x as f32,
                    vertices[(i + 1) % card_vertices].y as f32,
                );
                let triangle_mesh = Triangle2d::new(barycenter, v0, v1);
                commands.spawn((
                    Mesh2d(meshes.add(triangle_mesh)),
                    MeshMaterial2d(color_materials.add(ColorMaterial::from_color(cell_color))),
                    Transform::default(),
                ));
            }
        });
    }
}
