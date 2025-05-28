use super::planet::{Radius, materials::CommonMaterial};
use crate::core::{
    collision::CollisionShape,
    gravity::{Mass, Static},
    physics::{PhysicsSet, Position},
};
use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_ggrs::GgrsSchedule;

mod visuals;
use visuals::{BlackHoleMaterial, BlackHoleRingMaterial};

// TODO: move to config
pub const BLACKHOLE_MASS: u32 = 1000;

#[derive(Component, Debug, Reflect, Clone, PartialEq)]
#[require(Visibility)]
pub struct BlackHole;

#[derive(Event)]
pub struct SpawnBlackHoleEvent {
    pub position: Position,
    pub radius: Radius,
    // TODO: add DecayTimer
}

#[derive(Bundle)]
struct BlackHoleBundle {
    marker: BlackHole,
    position: Position,
    radius: Radius,
    mass: Mass,
    _collision_shape: CollisionShape,
}

impl BlackHoleBundle {
    fn new(position: Position, radius: Radius, mass: Mass) -> Self {
        Self {
            marker: BlackHole,
            position,
            mass,
            radius,
            _collision_shape: CollisionShape::Circle(radius.0 as f32),
        }
    }
}

pub struct BlackHolePlugin;
impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components_with::<BlackHole, Static>(|| Static)
            .register_required_components_with::<BlackHole, Name>(|| Name::new("Blackhole"))
            .add_event::<SpawnBlackHoleEvent>()
            // visuals
            .add_plugins(Material2dPlugin::<BlackHoleMaterial>::default())
            .add_plugins(Material2dPlugin::<BlackHoleRingMaterial>::default())
            .add_systems(Update, (add_visuals,))
            .add_systems(
                GgrsSchedule,
                handle_spawn_black_hole_event.before(PhysicsSet::Player),
            );
    }
}

fn handle_spawn_black_hole_event(
    mut events: EventReader<SpawnBlackHoleEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands.spawn((BlackHoleBundle::new(
            event.position.clone(),
            event.radius,
            Mass(BLACKHOLE_MASS),
        ),));
    }
}

fn add_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut core_materials: ResMut<Assets<BlackHoleMaterial>>,
    mut ring_materials: ResMut<Assets<BlackHoleRingMaterial>>,
    query: Query<(Entity, &Radius), (With<BlackHole>, Without<Children>)>,
) {
    for (black_hole, bh_radius) in query.iter() {
        let mut common = CommonMaterial {
            pixels: f32::min(bh_radius.0 as f32 / 2., 200.),
            seed: 69.,
            ..Default::default()
        };
        // Common
        let size = 5.0;
        let octaves = 3;
        common.size = size;
        common.octaves = octaves;

        // Material specific
        let radius = 0.5;
        let light_width = 0.05;
        let core_palette: Vec<&str> = vec!["#000000", "#fef4df", "#ff884d"];

        let core = core_materials.add(BlackHoleMaterial {
            common,
            radius,
            light_width,
            color_core: Srgba::hex(core_palette[0]).unwrap().into(),
            color_inner: Srgba::hex(core_palette[1]).unwrap().into(),
            color_outer: Srgba::hex(core_palette[2]).unwrap().into(),
            _wasm_padding: Vec2::ZERO,
        });

        let scale = 5.0;
        common = CommonMaterial {
            pixels: f32::min(bh_radius.0 as f32 / 2., 200.),
            seed: 69.,
            ..Default::default()
        }
        .scale(scale);

        // common
        let size = 6.598;
        let octaves = 3;
        let rotation = 0.766;
        common.rotation = rotation;
        common.size = size;
        common.octaves = octaves;

        // material specific
        let disk_width = 0.065;
        let ring_perspective = 14.;
        let ring_palette: Vec<&str> = vec!["#000000", "#ffb45c", "#ff8243", "#f25c19", "#fff5cc"];

        let ring = ring_materials.add(BlackHoleRingMaterial {
            common,
            disk_width,
            ring_perspective,
            should_dither: true as u32,
            n_colors: 5,
            colors: [
                Srgba::hex(ring_palette[0]).unwrap().into(),
                Srgba::hex(ring_palette[1]).unwrap().into(),
                Srgba::hex(ring_palette[2]).unwrap().into(),
                Srgba::hex(ring_palette[3]).unwrap().into(),
                Srgba::hex(ring_palette[4]).unwrap().into(),
            ],
            _wasm_padding: Vec2::ZERO,
        });

        let core_entity = commands
            .spawn((
                Name::new("CoreMesh"),
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(core),
                Transform::from_scale(Vec3::splat(bh_radius.0 as f32 * 2.0)).with_translation(
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 1.,
                    },
                ),
            ))
            .id();
        let ring_entity = commands
            .spawn((
                Name::new("RingMesh"),
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(ring),
                Transform::from_scale(Vec3::splat((bh_radius.0 as f32 * 2.0) * scale))
                    .with_translation(Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 2.,
                    }),
            ))
            .id();
        commands.entity(black_hole).add_children(&[core_entity]);
        commands.entity(black_hole).add_children(&[ring_entity]);
    }
}
