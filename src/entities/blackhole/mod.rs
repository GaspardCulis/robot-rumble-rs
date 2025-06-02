use super::planet::{Radius, materials::CommonMaterial};
use super::projectile::DecayTimer;
use crate::core::{
    collision::CollisionShape,
    gravity::{Mass, Static},
    physics::{PhysicsSet, Position},
};
use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_ggrs::GgrsSchedule;

mod visuals;
use visuals::*;

// TODO: move to config
const BLACKHOLE_MASS: u32 = 1000;
const BH_DECAY_TIME: f32 = 10.;

#[derive(Component, Debug, Reflect, Clone, PartialEq)]
#[require(Visibility)]
pub struct BlackHole;

#[derive(Event)]
pub struct SpawnBlackHoleEvent {
    pub position: Position,
    pub radius: Radius,
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
            .add_plugins(Material2dPlugin::<BlackHoleMaterial>::default())
            .add_plugins(Material2dPlugin::<BlackHoleRingMaterial>::default())
            .add_systems(Update, (add_visuals,))
            .add_systems(
                GgrsSchedule,
                (tick_blackhole_timer, handle_spawn_black_hole_event).before(PhysicsSet::Player),
            );
    }
}

fn tick_blackhole_timer(
    mut query: Query<(Entity, &mut DecayTimer), With<BlackHole>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (blackhole, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());

        if despawn_timer.0.just_finished() {
            commands.entity(blackhole).despawn();
        }
    }
}

fn handle_spawn_black_hole_event(
    mut events: EventReader<SpawnBlackHoleEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands
            .spawn((BlackHoleBundle::new(
                event.position.clone(),
                event.radius,
                Mass(BLACKHOLE_MASS),
            ),))
            .insert(DecayTimer(Timer::from_seconds(
                BH_DECAY_TIME,
                TimerMode::Once,
            )));
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

        // Material spesific
        common.size = CORE_SIZE;
        common.octaves = CORE_OCTAVES;

        let core = core_materials.add(BlackHoleMaterial {
            common,
            radius: CORE_RADIUS,
            light_width: CORE_LIGHT_WIDTH,
            color_core: Srgba::hex(CORE_PALETTE[0]).unwrap().into(),
            color_inner: Srgba::hex(CORE_PALETTE[1]).unwrap().into(),
            color_outer: Srgba::hex(CORE_PALETTE[2]).unwrap().into(),
            _wasm_padding: Vec2::ZERO,
        });

        common = CommonMaterial {
            pixels: f32::min(bh_radius.0 as f32 / 2., 200.),
            seed: 69.,
            ..Default::default()
        }
        .scale(CORE_SCALE);

        common.rotation = RING_ROTATION;
        common.size = RING_SIZE;
        common.octaves = RING_OCTAVES;

        let ring = ring_materials.add(BlackHoleRingMaterial {
            common,
            disk_width: RING_DISK_WIDTH,
            ring_perspective: RING_DISK_PERSPECTIVE,
            should_dither: true as u32,
            n_colors: 5,
            colors: [
                Srgba::hex(RING_PALETTE[0]).unwrap().into(),
                Srgba::hex(RING_PALETTE[1]).unwrap().into(),
                Srgba::hex(RING_PALETTE[2]).unwrap().into(),
                Srgba::hex(RING_PALETTE[3]).unwrap().into(),
                Srgba::hex(RING_PALETTE[4]).unwrap().into(),
            ],
            _wasm_padding: Vec2::ZERO,
        });

        let core_entity = commands
            .spawn((
                Name::new("BH_CoreMesh"),
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
                Name::new("BH_RingMesh"),
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(ring),
                Transform::from_scale(Vec3::splat((bh_radius.0 as f32 * 2.0) * CORE_SCALE))
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
