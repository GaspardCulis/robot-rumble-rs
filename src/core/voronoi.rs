use bevy::prelude::*;
use voronoice::*;

#[derive(Event)]
pub struct VoronoiGeneratedEvent {
    pub diagram: Voronoi,
}

pub struct VoronoPlugin;
impl Plugin for VoronoPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VoronoiGeneratedEvent>()
            .add_systems(Update, draw_voronoi_diagram);
    }
}

// For the moment it returns Voronoi as is, since it stocks useful information, maybe change for polygons vertices later
pub fn build_voronoi_diagram(sites: Vec<Vec2>, bounding_side: f64, relaxation: usize) -> Voronoi {
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

fn draw_voronoi_diagram(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<VoronoiGeneratedEvent>,
) {
    for VoronoiGeneratedEvent { diagram } in events.read() {
        // iterate over each cell of the diagram
        diagram.iter_cells().for_each(|cell| {
            // convert to bevy's format of points
            let vertices: Vec<Vec2> = cell
                .iter_vertices()
                .map(|p| Vec2::new(p.x as f32, p.y as f32))
                .collect();

            // Triangulate the mesh
            let center = cell.site_position();
            let center_vec2 = Vec2::new(center.x as f32, center.y as f32);
            let card_vertices = vertices.len();

            // use the same color for current cell
            let hue = rand::random::<f32>() * 360.0;
            let cell_color = Color::hsla(hue, 0.7, 0.5, 0.1);

            for i in 0..card_vertices {
                let v0 = Vec2::new(vertices[i].x as f32, vertices[i].y as f32);
                let v1 = Vec2::new(
                    vertices[(i + 1) % card_vertices].x as f32,
                    vertices[(i + 1) % card_vertices].y as f32,
                );

                let triangle_mesh = Triangle2d::new(center_vec2, v0, v1);
                commands.spawn((
                    Mesh2d(meshes.add(triangle_mesh)),
                    MeshMaterial2d(color_materials.add(ColorMaterial::from_color(cell_color))),
                    Transform::default(),
                ));
            }
        });
    }
}
